use smithay::{
    delegate_data_device, delegate_seat,
    input::{pointer::CursorImageStatus, Seat, SeatHandler, SeatState},
    reexports::wayland_server::protocol::wl_surface::WlSurface,
    wayland::selection::{
        data_device::{
            ClientDndGrabHandler, DataDeviceHandler, DataDeviceState, ServerDndGrabHandler,
        },
        SelectionHandler,
    },
};

use crate::state::HyperXCompState;

impl SeatHandler for HyperXCompState {
    type KeyboardFocus = WlSurface;
    type PointerFocus = WlSurface;
    type TouchFocus = WlSurface;

    fn seat_state(&mut self) -> &mut SeatState<Self> {
        &mut self.seat_state
    }

    fn focus_changed(&mut self, _seat: &Seat<Self>, _focused: Option<&WlSurface>) {}

    fn cursor_image(&mut self, _seat: &Seat<Self>, _image: CursorImageStatus) {}
}

impl ClientDndGrabHandler for HyperXCompState {}
impl ServerDndGrabHandler for HyperXCompState {}

impl SelectionHandler for HyperXCompState {
    type SelectionUserData = ();
}

impl DataDeviceHandler for HyperXCompState {
    fn data_device_state(&self) -> &DataDeviceState {
        &self.data_device_state
    }
}

delegate_seat!(HyperXCompState);
delegate_data_device!(HyperXCompState);
