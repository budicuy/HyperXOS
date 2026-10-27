use smithay::{
    delegate_layer_shell,
    desktop::{layer_map_for_output, LayerSurface},
    reexports::wayland_server::protocol::wl_output::WlOutput,
    wayland::shell::wlr_layer::{
        Layer, LayerSurface as WlrLayerSurface, WlrLayerShellHandler, WlrLayerShellState,
    },
};

use crate::state::HyperXCompState;

impl WlrLayerShellHandler for HyperXCompState {
    fn shell_state(&mut self) -> &mut WlrLayerShellState {
        &mut self.layer_shell_state
    }

    fn new_layer_surface(
        &mut self,
        surface: WlrLayerSurface,
        output: Option<WlOutput>,
        _layer: Layer,
        namespace: String,
    ) {
        tracing::info!("New layer surface mapped [namespace: {}]", namespace);

        let output = output
            .as_ref()
            .and_then(smithay::output::Output::from_resource)
            .or_else(|| self.space.outputs().next().cloned());

        if let Some(output) = output {
            let mut map = layer_map_for_output(&output);
            let layer_surface = LayerSurface::new(surface, namespace);
            let _ = map.map_layer(&layer_surface);
        }
    }
}

delegate_layer_shell!(HyperXCompState);
