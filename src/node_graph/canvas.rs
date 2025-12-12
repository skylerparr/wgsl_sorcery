use crate::node_graph::model::{CanvasState, NodeGraph};
use bevy::prelude::*;

pub fn update_canvas_system(
    mut node_graph: ResMut<NodeGraph>,
    mut mouse_wheel_events: EventReader<bevy::input::mouse::MouseWheel>,
) {
    // Handle zooming
    for event in mouse_wheel_events.read() {
        let canvas_state = &mut node_graph.canvas_state;
        let zoom_delta = event.y * 0.1;
        let new_zoom = (canvas_state.zoom - zoom_delta).max(0.1); // Prevent zooming out too far

        // We can't get cursor position here in a simple way, so we'll skip zoom centering
        canvas_state.zoom = new_zoom;
    }
}

pub fn screen_to_canvas(screen_pos: Vec2, canvas_state: &CanvasState) -> Vec2 {
    (screen_pos / canvas_state.zoom) + canvas_state.offset
}

pub fn canvas_to_screen(canvas_pos: Vec2, canvas_state: &CanvasState) -> Vec2 {
    (canvas_pos - canvas_state.offset) * canvas_state.zoom
}

pub fn render_canvas_background_system(_node_graph: Res<NodeGraph>) {
    // Render a simple background grid or other canvas elements here if needed
    // For now, we'll just ensure the canvas is cleared properly
}
