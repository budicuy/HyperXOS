use std::os::unix::io::OwnedFd;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

use smithay::{
    desktop::{Space, Window},
    delegate_compositor, delegate_data_device, delegate_layer_shell, delegate_output,
    delegate_seat, delegate_shm, delegate_xdg_shell,
    input::{pointer::CursorImageStatus, Seat, SeatHandler, SeatState},
    output::{Mode, Output, PhysicalProperties, Subpixel},
    reexports::wayland_server::{
        backend::{ClientData, ClientId, DisconnectReason},
        protocol::{
            wl_buffer,
            wl_output::WlOutput,
            wl_seat,
            wl_surface::{self, WlSurface},
        },
        Client, Display, ListeningSocket,
    },
    utils::{Logical, Point, Serial, Transform},
    wayland::{
        buffer::BufferHandler,
        compositor::{
            with_surface_tree_downward, CompositorClientState, CompositorHandler, CompositorState,
            SurfaceAttributes, TraversalAction,
        },
        output::{OutputHandler, OutputManagerState},
        seat::WaylandFocus,
        selection::{
            data_device::{
                ClientDndGrabHandler, DataDeviceHandler, DataDeviceState, ServerDndGrabHandler,
            },
            SelectionHandler,
        },
        shell::{
            wlr_layer::{Layer, LayerSurface, WlrLayerShellHandler, WlrLayerShellState},
            xdg::{
                PopupSurface, PositionerState, ToplevelSurface, XdgShellHandler, XdgShellState,
            },
        },
        shm::{ShmHandler, ShmState},
    },
};

#[derive(Debug, Clone)]
pub struct CompositorSnapshot {
    pub socket_name: String,
    pub connected_clients: usize,
    pub toplevel_count: usize,
}

impl Default for CompositorSnapshot {
    fn default() -> Self {
        Self {
            socket_name: "wayland-hyperx-0".to_string(),
            connected_clients: 0,
            toplevel_count: 0,
        }
    }
}

#[derive(Default)]
pub struct HyperXClientState {
    pub compositor_state: CompositorClientState,
}

impl ClientData for HyperXClientState {
    fn initialized(&self, _client_id: ClientId) {}
    fn disconnected(&self, _client_id: ClientId, _reason: DisconnectReason) {}
}

pub struct HyperXCompositor {
    pub compositor_state: CompositorState,
    pub xdg_shell_state: XdgShellState,
    pub layer_shell_state: WlrLayerShellState,
    pub shm_state: ShmState,
    pub _output_manager_state: OutputManagerState,
    pub seat_state: SeatState<Self>,
    pub data_device_state: DataDeviceState,
    pub _seat: Seat<Self>,
    pub space: Space<Window>,
}

impl BufferHandler for HyperXCompositor {
    fn buffer_destroyed(&mut self, _buffer: &wl_buffer::WlBuffer) {}
}

impl CompositorHandler for HyperXCompositor {
    fn compositor_state(&mut self) -> &mut CompositorState {
        &mut self.compositor_state
    }

    fn client_compositor_state<'a>(&self, client: &'a Client) -> &'a CompositorClientState {
        &client
            .get_data::<HyperXClientState>()
            .unwrap()
            .compositor_state
    }

    fn commit(&mut self, surface: &WlSurface) {
        smithay::backend::renderer::utils::on_commit_buffer_handler::<Self>(surface);
        for window in self.space.elements() {
            if window.wl_surface().as_deref() == Some(surface) {
                window.on_commit();
            }
        }
    }
}

impl ShmHandler for HyperXCompositor {
    fn shm_state(&self) -> &ShmState {
        &self.shm_state
    }
}

impl SeatHandler for HyperXCompositor {
    type KeyboardFocus = WlSurface;
    type PointerFocus = WlSurface;
    type TouchFocus = WlSurface;

    fn seat_state(&mut self) -> &mut SeatState<Self> {
        &mut self.seat_state
    }

    fn focus_changed(&mut self, _seat: &Seat<Self>, _focused: Option<&WlSurface>) {}
    fn cursor_image(&mut self, _seat: &Seat<Self>, _image: CursorImageStatus) {}
}

impl SelectionHandler for HyperXCompositor {
    type SelectionUserData = ();
}

impl DataDeviceHandler for HyperXCompositor {
    fn data_device_state(&self) -> &DataDeviceState {
        &self.data_device_state
    }
}

impl ClientDndGrabHandler for HyperXCompositor {}
impl ServerDndGrabHandler for HyperXCompositor {
    fn send(&mut self, _mime_type: String, _fd: OwnedFd, _seat: Seat<Self>) {}
}

impl OutputHandler for HyperXCompositor {}

impl XdgShellHandler for HyperXCompositor {
    fn xdg_shell_state(&mut self) -> &mut XdgShellState {
        &mut self.xdg_shell_state
    }

    fn new_toplevel(&mut self, surface: ToplevelSurface) {
        let window = Window::new_wayland_window(surface.clone());
        let offset = (self.space.elements().count() as i32 * 32) % 240;
        self.space
            .map_element(window, Point::<i32, Logical>::from((80 + offset, 64 + offset)), true);
        surface.send_configure();
    }

    fn new_popup(&mut self, _surface: PopupSurface, _positioner: PositionerState) {}

    fn grab(&mut self, _surface: PopupSurface, _seat: wl_seat::WlSeat, _serial: Serial) {}

    fn reposition_request(
        &mut self,
        _surface: PopupSurface,
        _positioner: PositionerState,
        _token: u32,
    ) {
    }
}

impl WlrLayerShellHandler for HyperXCompositor {
    fn shell_state(&mut self) -> &mut WlrLayerShellState {
        &mut self.layer_shell_state
    }

    fn new_layer_surface(
        &mut self,
        surface: LayerSurface,
        _output: Option<WlOutput>,
        _layer: Layer,
        _namespace: String,
    ) {
        surface.send_configure();
    }
}

delegate_compositor!(HyperXCompositor);
delegate_shm!(HyperXCompositor);
delegate_seat!(HyperXCompositor);
delegate_data_device!(HyperXCompositor);
delegate_output!(HyperXCompositor);
delegate_xdg_shell!(HyperXCompositor);
delegate_layer_shell!(HyperXCompositor);

pub fn send_frames_surface_tree(surface: &wl_surface::WlSurface, time: u32) {
    with_surface_tree_downward(
        surface,
        (),
        |_, _, &()| TraversalAction::DoChildren(()),
        |_surf, states, &()| {
            for callback in states
                .cached_state
                .get::<SurfaceAttributes>()
                .current()
                .frame_callbacks
                .drain(..)
            {
                callback.done(time);
            }
        },
        |_, _, &()| true,
    );
}

/// Starts the embedded Smithay Wayland server loop on a dedicated background thread
/// so Wayland protocols (xdg-shell, wlr-layer-shell, shm, seat, output) are live.
pub fn spawn_smithay_server() -> Arc<Mutex<CompositorSnapshot>> {
    let snapshot = Arc::new(Mutex::new(CompositorSnapshot::default()));
    let snap_clone = Arc::clone(&snapshot);

    std::thread::spawn(move || {
        let Ok(mut display) = Display::<HyperXCompositor>::new() else {
            return;
        };
        let dh = display.handle();

        let compositor_state = CompositorState::new::<HyperXCompositor>(&dh);
        let xdg_shell_state = XdgShellState::new::<HyperXCompositor>(&dh);
        let layer_shell_state = WlrLayerShellState::new::<HyperXCompositor>(&dh);
        let shm_state = ShmState::new::<HyperXCompositor>(&dh, vec![]);
        let output_manager_state = OutputManagerState::new_with_xdg_output::<HyperXCompositor>(&dh);
        let mut seat_state = SeatState::new();
        let mut seat = seat_state.new_wl_seat(&dh, "seat0");
        let _ = seat.add_keyboard(Default::default(), 200, 25);
        let _ = seat.add_pointer();
        let data_device_state = DataDeviceState::new::<HyperXCompositor>(&dh);

        let mut space = Space::default();
        let output = Output::new(
            "HyperX-Display-1".to_string(),
            PhysicalProperties {
                size: (520, 290).into(),
                subpixel: Subpixel::HorizontalRgb,
                make: "HyperXOS".into(),
                model: "Smithay-WGPU-Display".into(),
            },
        );
        let _global = output.create_global::<HyperXCompositor>(&dh);
        let mode = Mode {
            size: (1920, 1080).into(),
            refresh: 60_000,
        };
        output.change_current_state(
            Some(mode),
            Some(Transform::Normal),
            None,
            Some((0, 0).into()),
        );
        output.set_preferred(mode);
        space.map_output(&output, (0, 0));

        let mut state = HyperXCompositor {
            compositor_state,
            xdg_shell_state,
            layer_shell_state,
            shm_state,
            _output_manager_state: output_manager_state,
            seat_state,
            data_device_state,
            _seat: seat,
            space,
        };

        let Ok(listener) = ListeningSocket::bind_auto("wayland-hyperx", 0..16) else {
            return;
        };

        let sock_name = listener
            .socket_name()
            .and_then(|s| s.to_str())
            .unwrap_or("wayland-hyperx-0")
            .to_string();

        if let Ok(mut guard) = snap_clone.lock() {
            guard.socket_name = sock_name;
        }

        let start_time = Instant::now();
        let mut clients = Vec::new();

        loop {
            if let Ok(Some(stream)) = listener.accept() {
                if let Ok(client) = display
                    .handle()
                    .insert_client(stream, Arc::new(HyperXClientState::default()))
                {
                    clients.push(client);
                }
            }

            let _ = display.dispatch_clients(&mut state);
            let _ = display.flush_clients();

            let elapsed_ms = start_time.elapsed().as_millis() as u32;
            for surface in state.xdg_shell_state.toplevel_surfaces() {
                send_frames_surface_tree(surface.wl_surface(), elapsed_ms);
            }
            state.space.refresh();

            if let Ok(mut guard) = snap_clone.lock() {
                guard.connected_clients = clients.len();
                guard.toplevel_count = state.xdg_shell_state.toplevel_surfaces().len();
            }

            std::thread::sleep(Duration::from_millis(16));
        }
    });

    snapshot
}
