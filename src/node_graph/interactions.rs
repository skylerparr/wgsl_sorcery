use crate::node_graph::model::{CanvasState, Connection, NodeGraph, NodeLayout, PinId};
use crate::node_graph::pin_manager::PinPositionManager;
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
    let layout = NodeLayout::default();

    // Handle dragging logic - simplified approach using pointer state directly
    let pointer_pos = ctx.input(|i| i.pointer.latest_pos()).unwrap_or_default();
    let is_primary_pressed = ctx.input(|i| i.pointer.primary_down());
    let is_primary_released = ctx.input(|i| i.pointer.primary_released());

    // Stop dragging if primary button was released
    if is_primary_released && ui_state.active_drag_node.is_some() {
        info!(
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
            // Convert screen delta to canvas delta
            let canvas_delta = Vec2::new(drag_delta.x, drag_delta.y) / node_graph.canvas_state.zoom;

            // Update node position
            if let Some(node) = node_graph.nodes.get_mut(&active_drag_node_id) {
                let new_position = node.position + canvas_delta;
                info!(
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
        info!("Primary button pressed at position: {:?}", pointer_pos);

        // Check if pointer is over any node header
        for (_, node_instance) in node_graph.nodes.iter() {
            let canvas_state = &node_graph.canvas_state;
            let node_screen_pos =
                vec2_to_pos2((node_instance.position + canvas_state.offset) * canvas_state.zoom);

            // Check header area using centralized layout constants
            let header_rect = egui::Rect::from_min_size(
                node_screen_pos,
                egui::vec2(layout.width, layout.header_height),
            );

            if header_rect.contains(pointer_pos) {
                info!(
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
    pin_manager: Res<PinPositionManager>,
    mut egui_contexts: EguiContexts,
) {
    let ctx = egui_contexts.ctx_mut().expect("Failed to get egui context");
    let layout = NodeLayout::default();

    // First check for mouse release on connection completion
    if ctx.input(|i| i.pointer.any_released()) {
        let pointer_pos = ctx.input(|i| i.pointer.latest_pos()).unwrap_or_default();

        if let Some(pending) = &ui_state.pending_connection {
            info!(
                "INTERACTION: Mouse released, checking for connection completion at {:?}",
                pointer_pos
            );

            // Check for input pins at release position
            for (_, node_instance) in node_graph.nodes.iter() {
                let canvas_state = &node_graph.canvas_state;
                let node_screen_pos = vec2_to_pos2(
                    (node_instance.position + canvas_state.offset) * canvas_state.zoom,
                );

                // Calculate the Area rectangle using centralized layout constants
                let header_rect = egui::Rect::from_min_size(
                    node_screen_pos,
                    egui::vec2(layout.width, node_instance.header_height + 70.0),
                );

                // Check input pins - use centralized layout constants matching render system
                for (i, input_pin) in node_instance.inputs.iter().enumerate() {
                    let pin_radius = layout.pin_radius; // 6px radius from NodeLayout
                    let pin_x = -layout.pin_margin; // Slightly outside the node border
                    let pin_y =
                        layout.header_height + layout.pin_spacing + (i as f32 * layout.pin_spacing); // Below header

                    let pin_center = header_rect.min + egui::vec2(pin_x, pin_y);
                    let pin_rect = egui::Rect::from_center_size(
                        pin_center,
                        egui::vec2(pin_radius * 2.0, pin_radius * 2.0),
                    );

                    if pin_rect.contains(pointer_pos) {
                        info!(
                            "INTERACTION: Pointer released over input pin {:?} on node {:?}",
                            input_pin.pin_id, node_instance.node_id
                        );

                        // Validate connection rules
                        if pin_manager.can_connect_pins(
                            pending.from_pin,
                            input_pin.pin_id,
                            &node_graph,
                        ) {
                            let new_connection = Connection {
                                from_pin: pending.from_pin,
                                to_pin: input_pin.pin_id,
                            };

                            info!(
                                "INTERACTION: Creating connection from pin {:?} to pin {:?}",
                                pending.from_pin, input_pin.pin_id
                            );
                            node_graph.add_connection(new_connection);
                            info!(
                                "INTERACTION: Connection created successfully. Total connections: {}",
                                node_graph.connections.len()
                            );
                            ui_state.pending_connection = None;
                            return;
                        } else {
                            info!("INTERACTION: Invalid connection - rules not satisfied");
                        }
                    }
                }
            }
        }
    }

    if ctx.input(|i| i.pointer.any_pressed()) {
        let pointer_pos = ctx.input(|i| i.pointer.latest_pos()).unwrap_or_default();
        info!("INTERACTION: Mouse pressed at {:?}", pointer_pos);

        // Check for pin clicks (matching the render system exactly)
        let mut clicked_pin: Option<(PinId, bool)> = None; // (pin_id, is_input)

        for (_, node_instance) in node_graph.nodes.iter() {
            let canvas_state = &node_graph.canvas_state;
            let node_screen_pos =
                vec2_to_pos2((node_instance.position + canvas_state.offset) * canvas_state.zoom);

            // Calculate the Area rectangle using centralized layout constants
            let header_rect = egui::Rect::from_min_size(
                node_screen_pos,
                egui::vec2(layout.width, node_instance.header_height + 70.0),
            );

            // Check input pins - use centralized layout constants
            for (i, input_pin) in node_instance.inputs.iter().enumerate() {
                let pin_radius = layout.pin_radius; // 6px radius from NodeLayout
                let pin_x = -layout.pin_margin; // Slightly outside the node border
                let pin_y =
                    layout.header_height + layout.pin_spacing + (i as f32 * layout.pin_spacing); // Below header

                let pin_center = header_rect.min + egui::vec2(pin_x, pin_y);
                let pin_rect = egui::Rect::from_center_size(
                    pin_center,
                    egui::vec2(pin_radius * 2.0, pin_radius * 2.0),
                );

                if pin_rect.contains(pointer_pos) {
                    info!(
                        "INTERACTION: Clicked input pin {:?} on node {:?}",
                        input_pin.pin_id, node_instance.node_id
                    );
                    clicked_pin = Some((input_pin.pin_id, true));
                    break;
                }
            }

            // Check output pins - use centralized layout constants
            for (i, output_pin) in node_instance.outputs.iter().enumerate() {
                let pin_radius = layout.pin_radius; // 6px radius from NodeLayout
                let pin_x = layout.width + layout.pin_margin; // Slightly outside the node border
                let pin_y =
                    layout.header_height + layout.pin_spacing + (i as f32 * layout.pin_spacing); // Below header

                let pin_center = header_rect.min + egui::vec2(pin_x, pin_y);
                let pin_rect = egui::Rect::from_center_size(
                    pin_center,
                    egui::vec2(pin_radius * 2.0, pin_radius * 2.0),
                );

                if pin_rect.contains(pointer_pos) {
                    info!(
                        "INTERACTION: Clicked output pin {:?} on node {:?}",
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

                    info!(
                        "INTERACTION: Creating connection from pin {:?} to pin {:?}",
                        pending.from_pin, pin_id
                    );
                    node_graph.add_connection(new_connection);
                    info!(
                        "INTERACTION: Connection created successfully. Total connections: {}",
                        node_graph.connections.len()
                    );
                    ui_state.pending_connection = None;
                } else {
                    info!(
                        "INTERACTION: Clicked input pin {:?} but no pending connection",
                        pin_id
                    );
                }
            } else {
                // Clicked on output pin - start a new connection
                info!(
                    "INTERACTION: Starting connection from output pin {:?}",
                    pin_id
                );
                ui_state.pending_connection = Some(PendingConnection {
                    from_pin: pin_id,
                    from_screen_pos: pointer_pos,
                });
            }
        } else {
            // Clicked on empty space - cancel any pending connection
            if ui_state.pending_connection.is_some() {
                info!("INTERACTION: Clicked empty space, canceling pending connection");
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
