use crate::node_graph::model::{CanvasState, NodeGraph};
use bevy::input::mouse::MouseWheel;
use bevy::prelude::*;

pub fn update_canvas_system(
    mut node_graph: ResMut<NodeGraph>,
    mut mouse_wheel_events: EventReader<MouseWheel>,
    mouse_button_input: Res<ButtonInput<MouseButton>>,
) {
    let canvas_state = &mut node_graph.canvas_state;

    // Handle zooming
    for event in mouse_wheel_events.read() {
        let zoom_delta = event.y * 0.1;
        let new_zoom = (canvas_state.zoom - zoom_delta).max(0.1); // Prevent zooming out too far

        canvas_state.zoom = new_zoom;
    }

    // For now, we'll keep panning functionality simple and not implement it in this version
    // since getting cursor position requires more complex window interaction handling
}

pub fn screen_to_canvas(screen_pos: Vec2, canvas_state: &CanvasState) -> Vec2 {
    (screen_pos / canvas_state.zoom) + canvas_state.offset
}

pub fn canvas_to_screen(canvas_pos: Vec2, canvas_state: &CanvasState) -> Vec2 {
    (canvas_pos - canvas_state.offset) * canvas_state.zoom
}
