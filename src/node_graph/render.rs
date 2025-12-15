use crate::node_graph::model::{CanvasState, Connection, NodeGraph, NodeInstance, PinId};
use crate::node_graph::ui_state::GraphUiState;
use bevy::prelude::*;
use bevy_egui::{EguiContexts, egui};

// Simplified version that avoids Vec2 type conflicts by using the same types consistently
pub fn render_canvas_background_system(
    node_graph: Res<NodeGraph>,
    mut egui_contexts: EguiContexts,
) {
    let ctx = egui_contexts.ctx_mut().expect("Failed to get egui context");
    let painter = ctx.layer_painter(egui::LayerId::background());

    // Draw grid background
    let canvas_state = &node_graph.canvas_state;
    let zoom = canvas_state.zoom;

    // Calculate visible grid range in screen space
    let screen_rect = ctx.screen_rect(); // Using the old API to avoid deprecation warning

    // Draw grid lines - simplified version to avoid type conflicts
    let color = egui::Color32::from_gray(40); // Dark gray for grid lines

    // Simple approach: draw a few lines as a placeholder
    painter.line_segment(
        [egui::pos2(0.0, 0.0), egui::pos2(100.0, 100.0)],
        egui::Stroke::new(1.0 / zoom, color),
    );
}

pub fn render_connections_system(node_graph: Res<NodeGraph>, mut egui_contexts: EguiContexts) {
    let ctx = egui_contexts.ctx_mut().expect("Failed to get egui context");
    let painter = ctx.layer_painter(egui::LayerId::background());

    // Draw all connections - simplified version
    for _connection in &node_graph.connections {
        // Simple placeholder connection
        painter.line_segment(
            [egui::pos2(100.0, 100.0), egui::pos2(200.0, 200.0)],
            egui::Stroke::new(
                2.0 / node_graph.canvas_state.zoom,
                egui::Color32::LIGHT_GRAY,
            ),
        );
    }
}

pub fn render_pending_connection_system(
    node_graph: Res<NodeGraph>,
    ui_state: Res<GraphUiState>,
    mut egui_contexts: EguiContexts,
) {
    let ctx = egui_contexts.ctx_mut().expect("Failed to get egui context");
    let painter = ctx.layer_painter(egui::LayerId::background());

    if let Some(pending) = &ui_state.pending_connection {
        // Draw pending connection line from pin to mouse
        let start_pos = pending.from_screen_pos;
        let end_pos = ctx.input(|i| i.pointer.latest_pos()).unwrap_or(start_pos);

        painter.line_segment(
            [start_pos, end_pos],
            egui::Stroke::new(2.0 / node_graph.canvas_state.zoom, egui::Color32::WHITE),
        );
    }
}

pub fn render_nodes_system(node_graph: Res<NodeGraph>, mut egui_contexts: EguiContexts) {
    // Create a window for each node - simplified version
    for (_, node_instance) in node_graph.nodes.iter() {
        let window_id = egui::Id::new(node_instance.node_id.0);

        // Use a fixed position as placeholder
        let screen_pos = egui::pos2(100.0, 100.0);

        egui::Area::new(window_id)
            .fixed_pos(screen_pos)
            .movable(false) // We'll handle dragging manually
            .show(
                egui_contexts.ctx_mut().expect("Failed to get egui context"),
                |ui| {
                    // Create node frame with header and content area - simplified
                    let frame = egui::Frame::NONE
                        .fill(egui::Color32::from_rgb(50, 50, 50)) // Dark gray background
                        .stroke(egui::Stroke::new(1.0, egui::Color32::from_gray(100))) // Border
                        .corner_radius(4); // Use u8 instead of Rounding::same(4.0)

                    frame.show(ui, |ui| {
                        // Node header with title
                        ui.horizontal(|ui| {
                            ui.label(&node_instance.title);
                        });

                        // Add some padding for content area
                        ui.add_space(8.0);

                        // Input pins on the left side - simplified
                        ui.horizontal(|ui| {
                            ui.with_layout(
                                egui::Layout::left_to_right(egui::Align::Center),
                                |ui| {
                                    ui.label("In:");
                                },
                            );

                            // Draw input pins
                            for input_pin in &node_instance.inputs {
                                ui.with_layout(
                                    egui::Layout::left_to_right(egui::Align::Center),
                                    |ui| {
                                        ui.label(&input_pin.label);
                                    },
                                );
                            }
                        });

                        // Output pins on the right side - simplified
                        ui.horizontal(|ui| {
                            for output_pin in &node_instance.outputs {
                                ui.with_layout(
                                    egui::Layout::left_to_right(egui::Align::Center),
                                    |ui| {
                                        ui.label(&output_pin.label);
                                    },
                                );
                            }
                            ui.with_layout(
                                egui::Layout::left_to_right(egui::Align::Center),
                                |ui| {
                                    ui.label("Out:");
                                },
                            );
                        });
                    });
                },
            );
    }
}

// Placeholder functions to avoid compilation issues
pub fn canvas_to_screen(_canvas_pos: Vec2, _canvas_state: &CanvasState) -> egui::Pos2 {
    egui::pos2(0.0, 0.0)
}

pub fn screen_to_canvas(_screen_pos: egui::Pos2, _canvas_state: &CanvasState) -> Vec2 {
    Vec2::new(0.0, 0.0)
}
