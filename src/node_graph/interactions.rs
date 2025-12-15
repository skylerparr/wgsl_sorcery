use crate::node_graph::model::{CanvasState, Connection, NodeGraph, NodeId, NodeInstance, PinId};
use crate::node_graph::ui_state::{GraphUiState, PendingConnection};
use bevy::prelude::*;
use bevy_egui::{EguiContexts, egui};

pub fn handle_node_drag_system(
    mut node_graph: ResMut<NodeGraph>,
    mut ui_state: ResMut<GraphUiState>,
    mut egui_contexts: EguiContexts,
) {
    let ctx = egui_contexts.ctx_mut().expect("Failed to get egui context");

    // Handle dragging logic
    if let Some(active_drag_node_id) = ui_state.active_drag_node {
        // Debug logging for active dragging
        debug!("Dragging node: {:?}", active_drag_node_id);

        if ctx.input(|i| i.pointer.any_pressed()) {
            // If we're already dragging, update the drag offset
            let pointer_pos = ctx.input(|i| i.pointer.latest_pos()).unwrap_or_default();
            debug!("Pointer pressed during drag at position: {:?}", pointer_pos);

            let canvas_pos = screen_to_canvas(pointer_pos, &node_graph.canvas_state);
            debug!("Converted pointer to canvas position: {:?}", canvas_pos);

            // Update node position
            if let Some(node) = node_graph.nodes.get_mut(&active_drag_node_id) {
                let new_position = canvas_pos - ui_state.drag_offset;
                debug!(
                    "Updating node {:?} position from {:?} to {:?}",
                    active_drag_node_id, node.position, new_position
                );
                node.position = new_position;
            }
        } else if ctx.input(|i| i.pointer.any_released()) {
            // Stop dragging
            debug!("Dragging released for node: {:?}", active_drag_node_id);
            ui_state.active_drag_node = None;
        }
    } else {
        // Check for drag start on node header
        let pointer_pos = ctx.input(|i| i.pointer.latest_pos()).unwrap_or_default();
        debug!(
            "Checking for drag start at pointer position: {:?}",
            pointer_pos
        );

        let canvas_pos = screen_to_canvas(pointer_pos, &node_graph.canvas_state);
        debug!("Converted to canvas position: {:?}", canvas_pos);

        // Check if pointer is over a node header (simplified check)
        for (_, node_instance) in node_graph.nodes.iter() {
            let node_screen_pos =
                canvas_to_screen(node_instance.position, &node_graph.canvas_state);

            // Simple header area check
            let header_rect = egui::Rect::from_min_size(
                node_screen_pos,
                egui::vec2(200.0, 30.0), // Approximate header size
            );

            if header_rect.contains(pointer_pos) {
                debug!(
                    "Pointer is over node header for node: {:?}",
                    node_instance.node_id
                );
                if ctx.input(|i| i.pointer.primary_down()) {
                    // Start dragging
                    debug!("Starting drag for node: {:?}", node_instance.node_id);
                    ui_state.active_drag_node = Some(node_instance.node_id);
                    ui_state.drag_origin = canvas_pos;
                    ui_state.drag_offset = canvas_pos - node_instance.position;
                    break;
                }
            } else {
                debug!(
                    "Pointer not over node header for node: {:?}",
                    node_instance.node_id
                );
            }
        }
    }
}

pub fn handle_pin_interactions_system(
    mut node_graph: ResMut<NodeGraph>,
    mut ui_state: ResMut<GraphUiState>,
    mut egui_contexts: EguiContexts,
) {
    let ctx = egui_contexts.ctx_mut().expect("Failed to get egui context");

    if ctx.input(|i| i.pointer.any_pressed()) {
        let pointer_pos = ctx.input(|i| i.pointer.latest_pos()).unwrap_or_default();

        // Check if we clicked on a pin
        let canvas_pos = screen_to_canvas(pointer_pos, &node_graph.canvas_state);

        // Check for input/output pins
        let mut clicked_pin: Option<(PinId, bool)> = None; // (pin_id, is_input)

        for (_, node_instance) in node_graph.nodes.iter() {
            let node_screen_pos =
                canvas_to_screen(node_instance.position, &node_graph.canvas_state);

            // Check input pins
            for input_pin in &node_instance.inputs {
                // Simplified pin hit detection - would normally use stored pin positions
                let pin_radius = 6.0;
                let pin_x = node_screen_pos.x - pin_radius; // Left side pins
                let pin_y = node_screen_pos.y + 30.0; // Start from header height

                let pin_rect = egui::Rect::from_min_size(
                    egui::pos2(pin_x, pin_y),
                    egui::vec2(pin_radius * 2.0, pin_radius * 2.0),
                );

                if pin_rect.contains(pointer_pos) {
                    clicked_pin = Some((input_pin.pin_id, true));
                    break;
                }
            }

            // Check output pins
            for output_pin in &node_instance.outputs {
                // Simplified pin hit detection - would normally use stored pin positions
                let pin_radius = 6.0;
                let pin_x = node_screen_pos.x + 200.0; // Right side pins
                let pin_y = node_screen_pos.y + 30.0; // Start from header height

                let pin_rect = egui::Rect::from_min_size(
                    egui::pos2(pin_x, pin_y),
                    egui::vec2(pin_radius * 2.0, pin_radius * 2.0),
                );

                if pin_rect.contains(pointer_pos) {
                    clicked_pin = Some((output_pin.pin_id, false));
                    break;
                }
            }

            if clicked_pin.is_some() {
                break;
            }
        }

        // Handle pin click
        if let Some((pin_id, is_input)) = clicked_pin {
            if is_input {
                // Clicked on input pin - check if we have a pending connection
                if let Some(pending) = &ui_state.pending_connection {
                    // Connect to the input pin
                    let new_connection = Connection {
                        from_pin: pending.from_pin,
                        to_pin: pin_id,
                    };

                    node_graph.add_connection(new_connection);
                    ui_state.pending_connection = None;
                }
            } else {
                // Clicked on output pin - start a new connection
                ui_state.pending_connection = Some(PendingConnection {
                    from_pin: pin_id,
                    from_screen_pos: pointer_pos,
                });
            }
        } else {
            // Clicked on empty space - cancel any pending connection
            if ui_state.pending_connection.is_some() {
                ui_state.pending_connection = None;
            }
        }
    }
}

// Helper functions that avoid Vec2 type conflicts by using explicit conversions
pub fn screen_to_canvas(screen_pos: egui::Pos2, canvas_state: &CanvasState) -> Vec2 {
    // Use explicit scalar operations to prevent type confusion
    let screen_x = screen_pos.x;
    let screen_y = screen_pos.y;

    // Perform calculations with scalar values directly
    let result_x = (screen_x / canvas_state.zoom) + canvas_state.offset.x;
    let result_y = (screen_y / canvas_state.zoom) + canvas_state.offset.y;

    Vec2::new(result_x, result_y)
}

pub fn canvas_to_screen(canvas_pos: Vec2, canvas_state: &CanvasState) -> egui::Pos2 {
    // Use explicit scalar operations to prevent type confusion
    let canvas_x = canvas_pos.x;
    let canvas_y = canvas_pos.y;

    // Perform calculations with scalar values directly
    let screen_x = (canvas_x - canvas_state.offset.x) * canvas_state.zoom;
    let screen_y = (canvas_y - canvas_state.offset.y) * canvas_state.zoom;

    egui::pos2(screen_x, screen_y)
}
