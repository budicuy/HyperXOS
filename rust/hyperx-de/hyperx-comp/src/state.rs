use smithay::{
    desktop::{PopupManager, Space, Window},
    input::{Seat, SeatState},
    reexports::{
        calloop::LoopHandle,
        wayland_server::{
            backend::{ClientData, ClientId, DisconnectReason},
            Display, DisplayHandle,
        },
    },
    utils::{Clock, Monotonic, Point, Logical},
    wayland::{
        compositor::{CompositorClientState, CompositorState},
        output::OutputManagerState,
        selection::data_device::DataDeviceState,
        shell::{
            wlr_layer::WlrLayerShellState,
            xdg::XdgShellState,
        },
        shm::ShmState,
    },
};

pub struct ClientState {
    pub compositor_state: CompositorClientState,
}

impl Default for ClientState {
    fn default() -> Self {
        Self {
            compositor_state: CompositorClientState::default(),
        }
    }
}

impl ClientData for ClientState {
    fn initialized(&self, _client_id: ClientId) {}
    fn disconnected(&self, _client_id: ClientId, _reason: DisconnectReason) {}
}

pub struct HyperXCompState {
    pub display_handle: DisplayHandle,
    pub loop_handle: LoopHandle<'static, HyperXCompState>,
    pub space: Space<Window>,
    pub popups: PopupManager,
    pub compositor_state: CompositorState,
    pub xdg_shell_state: XdgShellState,
    pub layer_shell_state: WlrLayerShellState,
    pub shm_state: ShmState,
    pub seat_state: SeatState<HyperXCompState>,
    pub seat: Seat<HyperXCompState>,
    pub output_manager_state: OutputManagerState,
    pub data_device_state: DataDeviceState,
    pub clock: Clock<Monotonic>,
    pub is_running: bool,
    pub pointer_location: Point<f64, Logical>,
}

impl HyperXCompState {
    pub fn new(display: &mut Display<HyperXCompState>, loop_handle: LoopHandle<'static, HyperXCompState>) -> Self {
        let display_handle = display.handle();
        let clock = Clock::new();

        let compositor_state = CompositorState::new::<Self>(&display_handle);
        let xdg_shell_state = XdgShellState::new::<Self>(&display_handle);
        let layer_shell_state = WlrLayerShellState::new::<Self>(&display_handle);
        let shm_state = ShmState::new::<Self>(&display_handle, vec![]);
        let mut seat_state = SeatState::new();
        let mut seat = seat_state.new_wl_seat(&display_handle, "seat0");
        let output_manager_state = OutputManagerState::new_with_xdg_output::<Self>(&display_handle);
        let data_device_state = DataDeviceState::new::<Self>(&display_handle);

        let space = Space::default();
        let popups = PopupManager::default();

        // Enable pointer and keyboard for default seat
        let _ = seat.add_keyboard(Default::default(), 200, 25);
        let _ = seat.add_pointer();

        Self {
            display_handle,
            loop_handle,
            space,
            popups,
            compositor_state,
            xdg_shell_state,
            layer_shell_state,
            shm_state,
            seat_state,
            seat,
            output_manager_state,
            data_device_state,
            clock,
            is_running: true,
            pointer_location: Point::from((0.0, 0.0)),
        }
    }
}
