use crate::node_graph::model::{CanvasState, NodeGraph};
use bevy::input::mouse::{MouseMotion, MouseWheel};
use bevy::prelude::*;

pub fn update_canvas_system(
    mut node_graph: ResMut<NodeGraph>,
    mut mouse_wheel_events: MessageReader<MouseWheel>,
    mut mouse_motion_events: MessageReader<MouseMotion>,
    mouse_button_input: Res<ButtonInput<MouseButton>>,
    key_input: Res<ButtonInput<KeyCode>>,
) {
    let canvas_state = &mut node_graph.canvas_state;

    // Pan: RMB or MMB drag.
    let pan_pressed = mouse_button_input.pressed(MouseButton::Right)
        || mouse_button_input.pressed(MouseButton::Middle);

    if pan_pressed {
        let mut delta = Vec2::ZERO;
        for ev in mouse_motion_events.read() {
            delta += ev.delta;
        }

        if delta != Vec2::ZERO {
            // Moving the mouse right should move the canvas right (nodes appear to follow the hand).
            // Because offset is in *canvas* space, we divide by zoom.
            let canvas_delta = delta / canvas_state.zoom;
            canvas_state.offset -= canvas_delta;
            debug!(
                "Canvas pan: mouse_delta={:?} canvas_delta={:?} zoom={} new_offset={:?}",
                delta, canvas_delta, canvas_state.zoom, canvas_state.offset
            );
        }
    } else {
        // If we are not panning, still drain motion events so they don't accumulate.
        for _ in mouse_motion_events.read() {}
    }

    // Zooming: mouse wheel. Hold Ctrl to zoom (matches the README), but allow without Ctrl too.
    // NOTE: we don't currently zoom around cursor; we just adjust zoom.
    let ctrl = key_input.pressed(KeyCode::ControlLeft) || key_input.pressed(KeyCode::ControlRight);

    for event in mouse_wheel_events.read() {
        // Some mouse wheels report large deltas; keep it tame.
        let scroll = event.y;

        // If you *want* Ctrl-only zoom, uncomment the guard below.
        // if !ctrl { continue; }

        let zoom_delta = scroll * 0.1;
        let old_zoom = canvas_state.zoom;
        let new_zoom = (old_zoom + zoom_delta).clamp(0.1, 4.0);
        canvas_state.zoom = new_zoom;

        debug!(
            "Canvas zoom: scroll_y={} ctrl={} old_zoom={} new_zoom={} offset={:?}",
            scroll, ctrl, old_zoom, new_zoom, canvas_state.offset
        );
    }
}

pub fn screen_to_canvas(screen_pos: Vec2, canvas_state: &CanvasState) -> Vec2 {
    (screen_pos / canvas_state.zoom) + canvas_state.offset
}

pub fn canvas_to_screen(canvas_pos: Vec2, canvas_state: &CanvasState) -> Vec2 {
    (canvas_pos - canvas_state.offset) * canvas_state.zoom
}
