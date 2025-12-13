use crate::node_graph::model::{CanvasState, NodeGraph};
use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts};

pub fn render_nodes_system(
    node_graph: Res<NodeGraph>,
    mut egui_contexts: EguiContexts,
) {
    // Create a window for each node
    for (_, node_instance) in node_graph.nodes.iter() {
        let window_id = egui::Id::new(node_instance.node_id.0);
        
        egui::Window::new(&node_instance.title)
            .id(window_id)
            .default_pos(egui::pos2(node_instance.position.x, node_instance.position.y))
            .resizable(false)
            .show(egui_contexts.ctx_mut().expect("Failed to get egui context"), |ui| {
                // Layout input pins on the left side
                ui.horizontal(|ui| {
                    ui.label("In:");
                    for input_pin in &node_instance.inputs {
                        ui.label(&input_pin.label);
                    }
                });
                
                // Layout output pins on the right side  
                ui.horizontal(|ui| {
                    for output_pin in &node_instance.outputs {
                        ui.label(&output_pin.label);
                    }
                    ui.label("Out:");
                });
            });
    }
}

pub fn render_connections_system(
    _node_graph: Res<NodeGraph>,
    mut egui_contexts: EguiContexts,
) {
    // Draw connections between nodes
    let ctx = egui_contexts.ctx_mut().expect("Failed to get egui context");
    let painter = ctx.layer_painter(egui::LayerId::background());
    
    // For now, draw a simple placeholder line - in a real implementation
    // this would connect actual pins from input/output nodes
    let start_pos = egui::pos2(100.0, 100.0);
    let end_pos = egui::pos2(200.0, 200.0);
    
    painter.line_segment(
        [start_pos, end_pos],
        egui::Stroke::new(2.0, egui::Color32::WHITE),
    );
}

pub fn canvas_to_screen(canvas_pos: Vec2, canvas_state: &CanvasState) -> egui::Pos2 {
    let screen_pos = (canvas_pos - canvas_state.offset) * canvas_state.zoom;
    egui::pos2(screen_pos.x, screen_pos.y)
}
