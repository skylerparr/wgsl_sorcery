use crate::node_graph::model::{CanvasState, Connection, NodeGraph, NodeId, NodeInstance, PinId};
use crate::node_graph::ui_state::{GraphUiState, PendingConnection};
use bevy::prelude::*;
use bevy_egui::{EguiContexts, egui};

// Convert from bevy Vec2 to egui Pos2
fn vec2_to_pos2(vec: Vec2) -> egui::Pos2 {
    egui::pos2(vec.x, vec.y)
}

// Convert from egui Pos2 to bevy Vec2
fn pos2_to_vec2(pos: egui::Pos2) -> Vec2 {
    Vec2::new(pos.x, pos.y)
}

pub fn handle_node_drag_system(
    mut node_graph: ResMut<NodeGraph>,
    mut ui_state: ResMut<GraphUiState>,
    mut egui_contexts: EguiContexts,
) {
    let ctx = egui_contexts.ctx_mut().expect("Failed to get egui context");

    // Handle dragging logic - simplified approach using pointer state directly
    let pointer_pos = ctx.input(|i| i.pointer.latest_pos()).unwrap_or_default();
    let is_primary_pressed = ctx.input(|i| i.pointer.primary_down());
    let is_primary_released = ctx.input(|i| i.pointer.primary_released());

    // Stop dragging if primary button was released
    if is_primary_released && ui_state.active_drag_node.is_some() {
        debug!(
            "Primary button released, stopping drag for node: {:?}",
            ui_state.active_drag_node
        );
        ui_state.active_drag_node = None;
        return;
    }

    // If we're actively dragging, update position based on mouse movement
    if let Some(active_drag_node_id) = ui_state.active_drag_node {
        let drag_delta = ctx.input(|i| i.pointer.delta());
        if drag_delta != egui::Vec2::ZERO {
            debug!(
                "Dragging node {:?} with delta {:?}",
                active_drag_node_id, drag_delta
            );

            // Convert screen delta to canvas delta
            let canvas_delta = Vec2::new(drag_delta.x, drag_delta.y) / node_graph.canvas_state.zoom;

            // Update node position
            if let Some(node) = node_graph.nodes.get_mut(&active_drag_node_id) {
                let new_position = node.position + canvas_delta;
                debug!(
                    "Moving node {:?} from {:?} to {:?}",
                    active_drag_node_id, node.position, new_position
                );
                node.position = new_position;
            }
        }
        return;
    }

    // Check for drag start - if primary button is pressed and we're over a node header
    if is_primary_pressed {
        debug!("Primary button pressed at position: {:?}", pointer_pos);

        // Check if pointer is over any node header
        for (_, node_instance) in node_graph.nodes.iter() {
            let canvas_state = &node_graph.canvas_state;
            let node_screen_pos =
                vec2_to_pos2((node_instance.position + canvas_state.offset) * canvas_state.zoom);

            // Check header area
            let header_rect = egui::Rect::from_min_size(
                node_screen_pos,
                egui::vec2(220.0, node_instance.header_height), // Node width and header height
            );

            if header_rect.contains(pointer_pos) {
                debug!(
                    "Starting drag for node: {:?} at screen pos {:?}",
                    node_instance.node_id, pointer_pos
                );
                ui_state.active_drag_node = Some(node_instance.node_id);
                // Store the current position as the drag origin
                ui_state.drag_origin = node_instance.position;
                break;
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

    // First check for mouse release on connection dots (for completing node-to-node connections)
    if ctx.input(|i| i.pointer.any_released()) {
        let pointer_pos = ctx.input(|i| i.pointer.latest_pos()).unwrap_or_default();

        if let Some(pending) = &ui_state.pending_connection {
            // Check if we released over a blue or red connection dot
            for (_, node_instance) in node_graph.nodes.iter() {
                let canvas_state = &node_graph.canvas_state;
                let node_screen_pos = vec2_to_pos2(
                    (node_instance.position + canvas_state.offset) * canvas_state.zoom,
                );

                let dot_radius = 12.0;

                // Calculate the Area rectangle (same as used in render system)
                let header_rect = egui::Rect::from_min_size(
                    node_screen_pos,
                    egui::vec2(220.0, node_instance.header_height + 70.0),
                );

                // Check blue connection dot - positioned at (50.0, 70.0) relative to header_rect.min
                let blue_dot_center = header_rect.min + egui::vec2(50.0, 70.0);
                let blue_dot_rect = egui::Rect::from_center_size(
                    blue_dot_center,
                    egui::vec2(dot_radius * 2.0, dot_radius * 2.0),
                );

                // Check red connection dot - positioned at (170.0, 70.0) relative to header_rect.min
                let red_dot_center = header_rect.min + egui::vec2(170.0, 70.0);
                let red_dot_rect = egui::Rect::from_center_size(
                    red_dot_center,
                    egui::vec2(dot_radius * 2.0, dot_radius * 2.0),
                );

                let from_node_id = NodeId(pending.from_pin.0);

                if blue_dot_rect.contains(pointer_pos) || red_dot_rect.contains(pointer_pos) {
                    if node_instance.node_id != from_node_id {
                        // Create node-to-node connection
                        let new_connection = Connection {
                            from_pin: pending.from_pin,
                            to_pin: PinId(node_instance.node_id.0),
                        };

                        debug!(
                            "Creating node-to-node connection from node {:?} to node {:?}",
                            from_node_id, node_instance.node_id
                        );
                        node_graph.add_connection(new_connection);
                        debug!(
                            "Node-to-node connection created successfully. Total connections: {}",
                            node_graph.connections.len()
                        );
                        ui_state.pending_connection = None;
                        return;
                    }
                }
            }
        }
    }

    if ctx.input(|i| i.pointer.any_pressed()) {
        let pointer_pos = ctx.input(|i| i.pointer.latest_pos()).unwrap_or_default();

        // Check for connection dots on nodes (higher priority)
        let mut clicked_node_dot: Option<(NodeId, bool)> = None; // (node_id, is_blue_dot)

        for (_, node_instance) in node_graph.nodes.iter() {
            let canvas_state = &node_graph.canvas_state;
            let node_screen_pos =
                vec2_to_pos2((node_instance.position + canvas_state.offset) * canvas_state.zoom);

            let dot_radius = 12.0;

            // Calculate the Area rectangle (same as used in render system)
            let header_rect = egui::Rect::from_min_size(
                node_screen_pos,
                egui::vec2(220.0, node_instance.header_height + 70.0),
            );

            // Check blue connection dot - positioned at (50.0, 70.0) relative to header_rect.min
            let blue_dot_center = header_rect.min + egui::vec2(50.0, 70.0);
            let blue_dot_rect = egui::Rect::from_center_size(
                blue_dot_center,
                egui::vec2(dot_radius * 2.0, dot_radius * 2.0),
            );

            debug!(
                "Checking blue dot rect: {:?} for pointer at: {:?}",
                blue_dot_rect, pointer_pos
            );

            if blue_dot_rect.contains(pointer_pos) {
                debug!(
                    "Clicked blue connection dot on node {:?}",
                    node_instance.node_id
                );
                clicked_node_dot = Some((node_instance.node_id, true)); // true = blue dot
                break;
            }

            // Check red connection dot - positioned at (170.0, 70.0) relative to header_rect.min
            let red_dot_center = header_rect.min + egui::vec2(170.0, 70.0);
            let red_dot_rect = egui::Rect::from_center_size(
                red_dot_center,
                egui::vec2(dot_radius * 2.0, dot_radius * 2.0),
            );

            debug!(
                "Checking red dot rect: {:?} for pointer at: {:?}",
                red_dot_rect, pointer_pos
            );

            if red_dot_rect.contains(pointer_pos) {
                debug!(
                    "Clicked red connection dot on node {:?}",
                    node_instance.node_id
                );
                clicked_node_dot = Some((node_instance.node_id, false)); // false = red dot
                break;
            }
        }

        // If we clicked a node connection dot, handle that instead of pins
        if let Some((node_id, _is_blue)) = clicked_node_dot {
            debug!("Starting node-to-node connection from node {:?}", node_id);
            ui_state.pending_connection = Some(PendingConnection {
                from_pin: PinId(node_id.0), // Use node ID as pin ID for simplicity
                from_screen_pos: pointer_pos,
            });
            return;
        }

        // Check for input/output pins (fallback)
        let mut clicked_pin: Option<(PinId, bool)> = None; // (pin_id, is_input)

        // Check for traditional pin connections (matching the render system)
        for (_, node_instance) in node_graph.nodes.iter() {
            let canvas_state = &node_graph.canvas_state;
            let node_screen_pos =
                vec2_to_pos2((node_instance.position + canvas_state.offset) * canvas_state.zoom);

            // Calculate the Area rectangle (same as used in render system)
            let header_rect = egui::Rect::from_min_size(
                node_screen_pos,
                egui::vec2(220.0, node_instance.header_height + 70.0),
            );

            // Check input pins - positioned relative to header_rect.min (matching render system)
            for (i, input_pin) in node_instance.inputs.iter().enumerate() {
                let pin_radius = 6.0;
                let pin_x = -6.0; // Slightly outside the node border (matching render system)
                let pin_y = node_instance.header_height + 20.0 + (i as f32 * 20.0); // Below header

                let pin_center = header_rect.min + egui::vec2(pin_x, pin_y);
                let pin_rect = egui::Rect::from_center_size(
                    pin_center,
                    egui::vec2(pin_radius * 2.0, pin_radius * 2.0),
                );

                if pin_rect.contains(pointer_pos) {
                    debug!(
                        "Clicked input pin {:?} on node {:?}",
                        input_pin.pin_id, node_instance.node_id
                    );
                    clicked_pin = Some((input_pin.pin_id, true));
                    break;
                }
            }

            // Check output pins - positioned relative to header_rect.min (matching render system)
            for (i, output_pin) in node_instance.outputs.iter().enumerate() {
                let pin_radius = 6.0;
                let pin_x = 226.0; // Slightly outside the node border (matching render system)
                let pin_y = node_instance.header_height + 20.0 + (i as f32 * 20.0); // Below header

                let pin_center = header_rect.min + egui::vec2(pin_x, pin_y);
                let pin_rect = egui::Rect::from_center_size(
                    pin_center,
                    egui::vec2(pin_radius * 2.0, pin_radius * 2.0),
                );

                if pin_rect.contains(pointer_pos) {
                    debug!(
                        "Clicked output pin {:?} on node {:?}",
                        output_pin.pin_id, node_instance.node_id
                    );
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

                    debug!(
                        "Creating connection from pin {:?} to pin {:?}",
                        pending.from_pin, pin_id
                    );
                    node_graph.add_connection(new_connection);
                    debug!(
                        "Connection created successfully. Total connections: {}",
                        node_graph.connections.len()
                    );
                    ui_state.pending_connection = None;
                } else {
                    debug!("Clicked input pin {:?} but no pending connection", pin_id);
                }
            } else {
                // Clicked on output pin - start a new connection
                debug!("Starting connection from output pin {:?}", pin_id);
                ui_state.pending_connection = Some(PendingConnection {
                    from_pin: pin_id,
                    from_screen_pos: pointer_pos,
                });
            }
        } else {
            // Clicked on empty space - cancel any pending connection
            if ui_state.pending_connection.is_some() {
                debug!("Clicked empty space, canceling pending connection");
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
