use std::sync::Arc;
use std::time::Duration;

use smithay::{
    backend::{
        input::{
            AbsolutePositionEvent, Axis, Event, InputEvent, KeyboardKeyEvent,
            PointerAxisEvent, PointerButtonEvent,
        },
        renderer::{
            damage::OutputDamageTracker,
            gles::GlesRenderer,
        },
        winit::{self, WinitEvent},
    },
    desktop::{space::render_output, Window},
    input::{
        keyboard::FilterResult,
        pointer::{AxisFrame, ButtonEvent, MotionEvent},
    },
    output::{Mode, Output, PhysicalProperties, Subpixel},
    reexports::{
        calloop::EventLoop,
        wayland_server::Display,
    },
    utils::{Point, Size, Transform, SERIAL_COUNTER},
    wayland::{
        seat::WaylandFocus,
        socket::ListeningSocketSource,
    },
};

use crate::state::{ClientState, HyperXCompState};

pub fn run_winit() -> Result<(), Box<dyn std::error::Error>> {
    tracing::info!("Initializing hyperx-comp in Winit nested backend mode...");

    let mut event_loop: EventLoop<HyperXCompState> = EventLoop::try_new()?;
    let mut display: Display<HyperXCompState> = Display::new()?;
    let mut state = HyperXCompState::new(&mut display, event_loop.handle());

    // 1. Initialize Winit backend with GLES renderer
    let (mut backend, mut winit_event_loop) = winit::init::<GlesRenderer>()?;

    let window_size = backend.window_size();
    let mode = Mode {
        size: Size::from((window_size.w as i32, window_size.h as i32)),
        refresh: 60_000,
    };

    let output = Output::new(
        "hyperx-nested-1".to_string(),
        PhysicalProperties {
            size: (0, 0).into(),
            subpixel: Subpixel::Unknown,
            make: "HyperXOS".to_string(),
            model: "Virtual Nested Display".to_string(),
        },
    );

    output.change_current_state(Some(mode), Some(Transform::Normal), None, Some((0, 0).into()));
    output.set_preferred(mode);

    state.space.map_output(&output, (0, 0));
    let mut damage_tracker = OutputDamageTracker::from_output(&output);

    // 2. Setup Wayland listening socket
    let socket_source = ListeningSocketSource::new_auto()?;
    let socket_name = socket_source.socket_name().to_string_lossy().into_owned();
    tracing::info!("==> [HyperXOS] Wayland compositor active at: WAYLAND_DISPLAY={}", socket_name);
    println!("======================================================================");
    println!("  HyperXOS Compositor (hyperx-comp) Active!");
    println!("  Wayland Display Socket: {}", socket_name);
    println!("  Nested Window Ready: {}x{}", window_size.w, window_size.h);
    println!("======================================================================");
    std::env::set_var("WAYLAND_DISPLAY", &socket_name);

    event_loop.handle().insert_source(socket_source, |stream, _, state| {
        if let Err(err) = state.display_handle.insert_client(stream, Arc::new(ClientState::default())) {
            tracing::warn!("Failed to insert client into display: {:?}", err);
        }
    })?;

    // 3. Main render & event pump loop
    let clear_color = [0.08f32, 0.08f32, 0.12f32, 1.0f32]; // HyperXOS Dark Frosted Glass color

    while state.is_running {
        // Dispatch incoming Wayland protocol requests
        display.dispatch_clients(&mut state)?;

        let mut needs_redraw = false;

        winit_event_loop.dispatch_new_events(|event| match event {
            WinitEvent::Resized { size, scale_factor: _ } => {
                let new_mode = Mode {
                    size: Size::from((size.w, size.h)),
                    refresh: 60_000,
                };
                output.change_current_state(Some(new_mode), None, None, None);
                needs_redraw = true;
            }
            WinitEvent::CloseRequested => {
                state.is_running = false;
            }
            WinitEvent::Redraw => {
                needs_redraw = true;
            }
            WinitEvent::Input(event) => match event {
                InputEvent::DeviceAdded { device: _ } | InputEvent::DeviceRemoved { device: _ } => {}
                InputEvent::PointerMotionAbsolute { event } => {
                    let pos = event.position();
                    let current_location = Point::from((pos.x, pos.y));
                    state.pointer_location = current_location;
                    let serial = SERIAL_COUNTER.next_serial();
                    let under = state.space.element_under(current_location);

                    if let Some(pointer) = state.seat.get_pointer() {
                        let focus = under.and_then(|(win, loc)| {
                            win.wl_surface().map(|s| (s.into_owned(), loc.to_f64()))
                        });
                        pointer.motion(
                            &mut state,
                            focus,
                            &MotionEvent {
                                location: current_location,
                                serial,
                                time: event.time_msec(),
                            },
                        );
                    }
                    needs_redraw = true;
                }
                InputEvent::PointerButton { event } => {
                    let serial = SERIAL_COUNTER.next_serial();
                    let button = event.button_code();
                    let button_state = event.state();

                    if let Some(pointer) = state.seat.get_pointer() {
                        pointer.button(
                            &mut state,
                            &ButtonEvent {
                                serial,
                                time: event.time_msec(),
                                button,
                                state: button_state,
                            },
                        );
                    }
                    needs_redraw = true;
                }
                InputEvent::PointerAxis { event } => {
                    let mut frame = AxisFrame::new(event.time_msec());
                    if let Some(horizontal) = event.amount(Axis::Horizontal) {
                        frame = frame.value(Axis::Horizontal, horizontal);
                    }
                    if let Some(vertical) = event.amount(Axis::Vertical) {
                        frame = frame.value(Axis::Vertical, vertical);
                    }

                    if let Some(pointer) = state.seat.get_pointer() {
                        pointer.axis(&mut state, frame);
                    }
                    needs_redraw = true;
                }
                InputEvent::Keyboard { event } => {
                    let serial = SERIAL_COUNTER.next_serial();
                    let time = event.time_msec();
                    let keycode = event.key_code();
                    let key_state = event.state();

                    if let Some(keyboard) = state.seat.get_keyboard() {
                        keyboard.input::<(), _>(
                            &mut state,
                            keycode,
                            key_state,
                            serial,
                            time,
                            |_, _, _| FilterResult::Forward,
                        );
                    }
                }
                _ => {}
            },
            _ => {}
        });

        // 4. Render output with damage tracking
        if needs_redraw {
            {
                let (renderer, mut framebuffer) = backend.bind()?;
                let _ = render_output::<GlesRenderer, smithay::backend::renderer::element::surface::WaylandSurfaceRenderElement<GlesRenderer>, Window, _>(
                    &output,
                    renderer,
                    &mut framebuffer,
                    1.0,
                    0,
                    [&state.space],
                    &[],
                    &mut damage_tracker,
                    clear_color,
                );
            }

            let _ = backend.submit(None);
        }

        // Send protocol messages to clients
        display.flush_clients()?;

        // Dispatch calloop sources
        event_loop.dispatch(Some(Duration::from_millis(16)), &mut state)?;
    }

    tracing::info!("hyperx-comp shut down gracefully.");
    Ok(())
}
