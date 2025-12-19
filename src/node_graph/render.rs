use crate::node_graph::model::{NodeGraph, NodeLayout};
use crate::node_graph::pin_manager::PinPositionManager;
use crate::node_graph::ui_state::GraphUiState;
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
    let screen_rect = ctx.viewport_rect();

    // Draw grid lines using canvas->screen transforms
    let grid_spacing = 48.0; // Base grid spacing in canvas space
    let visible_min = pos2_to_vec2(screen_rect.min);
    let visible_max = pos2_to_vec2(screen_rect.max);

    // Convert screen bounds to canvas bounds
    let canvas_min = (visible_min / zoom) - canvas_state.offset;
    let canvas_max = (visible_max / zoom) - canvas_state.offset;

    // Find grid lines in visible range
    let start_x = (canvas_min.x / grid_spacing).floor() * grid_spacing;
    let end_x = (canvas_max.x / grid_spacing).ceil() * grid_spacing;
    let start_y = (canvas_min.y / grid_spacing).floor() * grid_spacing;
    let end_y = (canvas_max.y / grid_spacing).ceil() * grid_spacing;

    // Draw vertical grid lines
    for x in (start_x as i32..=end_x as i32).step_by(grid_spacing as usize) {
        let canvas_pos = Vec2::new(x as f32, start_y);
        let screen_pos = vec2_to_pos2((canvas_pos + canvas_state.offset) * zoom);
        let screen_pos2 = vec2_to_pos2((Vec2::new(x as f32, end_y) + canvas_state.offset) * zoom);

        painter.line_segment(
            [screen_pos, screen_pos2],
            egui::Stroke::new(1.0 / zoom, egui::Color32::from_gray(40)),
        );
    }

    // Draw horizontal grid lines
    for y in (start_y as i32..=end_y as i32).step_by(grid_spacing as usize) {
        let canvas_pos = Vec2::new(start_x, y as f32);
        let screen_pos = vec2_to_pos2((canvas_pos + canvas_state.offset) * zoom);
        let screen_pos2 = vec2_to_pos2((Vec2::new(end_x, y as f32) + canvas_state.offset) * zoom);

        painter.line_segment(
            [screen_pos, screen_pos2],
            egui::Stroke::new(1.0 / zoom, egui::Color32::from_gray(40)),
        );
    }
}

pub fn render_connections_system(
    node_graph: Res<NodeGraph>,
    mut pin_manager: ResMut<PinPositionManager>,
    mut egui_contexts: EguiContexts,
) {
    let ctx = egui_contexts.ctx_mut().expect("Failed to get egui context");
    let painter = ctx.layer_painter(egui::LayerId::background());

    let canvas_state = &node_graph.canvas_state;

    // Draw all connections as smooth bezier curves using single point of authority
    for (connection_index, connection) in node_graph.connections.iter().enumerate() {
        if let Some((from_pos, to_pos)) = pin_manager.get_connection_endpoints(
            connection.from_pin,
            connection.to_pin,
            &node_graph,
            canvas_state,
        ) {
            // Convert to screen positions
            let from_screen = vec2_to_pos2(from_pos);
            let to_screen = vec2_to_pos2(to_pos);

            // Draw smooth bezier curve using proper control points
            let distance = (to_screen - from_screen).length();
            let ctrl_offset = distance * 0.3; // 30% of the distance between points

            // Calculate control points for smooth bezier curve
            let direction = (to_screen - from_screen).normalized();
            let perpendicular = egui::vec2(-direction.y, direction.x);

            let ctrl1 = from_screen + perpendicular * 20.0 + direction * ctrl_offset;
            let ctrl2 = to_screen - perpendicular * 20.0 - direction * ctrl_offset;

            // Draw smooth bezier curve using multiple line segments
            let segments = 12;
            let mut prev_point = from_screen;

            for i in 1..=segments {
                let t = i as f32 / segments as f32;

                // Cubic bezier interpolation
                let u = 1.0 - t;
                let tt = t * t;
                let uu = u * u;
                let uuu = uu * u;
                let ttt = tt * t;

                let current_point = egui::pos2(
                    uuu * from_screen.x
                        + 3.0 * uu * t * ctrl1.x
                        + 3.0 * u * tt * ctrl2.x
                        + ttt * to_screen.x,
                    uuu * from_screen.y
                        + 3.0 * uu * t * ctrl1.y
                        + 3.0 * u * tt * ctrl2.y
                        + ttt * to_screen.y,
                );

                painter.line_segment(
                    [prev_point, current_point],
                    egui::Stroke::new(3.0, egui::Color32::LIGHT_GRAY),
                );

                prev_point = current_point;
            }
        } else {
            warn!(
                "RENDER: Could not find endpoints for connection {}: from pin {:?} to pin {:?}",
                connection_index, connection.from_pin, connection.to_pin
            );
        }
    }
}

pub fn render_pending_connection_system(
    node_graph: Res<NodeGraph>,
    ui_state: Res<GraphUiState>,
    mut pin_manager: ResMut<PinPositionManager>,
    mut egui_contexts: EguiContexts,
) {
    let ctx = egui_contexts.ctx_mut().expect("Failed to get egui context");
    let painter = ctx.layer_painter(egui::LayerId::background());

    if let Some(pending) = &ui_state.pending_connection {
        let canvas_state = &node_graph.canvas_state;

        // Find the source pin position using single point of authority
        if let Some(from_pos) =
            pin_manager.get_pin_screen_position(pending.from_pin, &node_graph, canvas_state)
        {
            let from_screen = vec2_to_pos2(from_pos);
            let end_pos = ctx.input(|i| i.pointer.latest_pos()).unwrap_or(from_screen);

            // Draw temporary bezier curve using line segments
            let ctrl_offset = Vec2::new(80.0, 0.0);
            let ctrl1 = from_screen + egui::vec2(ctrl_offset.x, ctrl_offset.y);
            let ctrl2 = end_pos - egui::vec2(ctrl_offset.x, ctrl_offset.y);

            // Simple line approximation for now
            painter.line_segment(
                [from_screen, ctrl1],
                egui::Stroke::new(2.0 / canvas_state.zoom, egui::Color32::WHITE),
            );
            painter.line_segment(
                [ctrl1, ctrl2],
                egui::Stroke::new(2.0 / canvas_state.zoom, egui::Color32::WHITE),
            );
            painter.line_segment(
                [ctrl2, end_pos],
                egui::Stroke::new(2.0 / canvas_state.zoom, egui::Color32::WHITE),
            );
        }
    }
}

pub fn render_nodes_system(node_graph: Res<NodeGraph>, mut egui_contexts: EguiContexts) {
    let ctx = egui_contexts.ctx_mut().expect("Failed to get egui context");
    let canvas_state = &node_graph.canvas_state;
    let layout = NodeLayout::default();

    // Create a window for each node using proper canvas->screen transforms
    for (_, node_instance) in node_graph.nodes.iter() {
        let window_id = egui::Id::new(node_instance.node_id.0);

        // Convert node position from canvas to screen space
        let screen_pos =
            vec2_to_pos2((node_instance.position + canvas_state.offset) * canvas_state.zoom);

        egui::Area::new(window_id)
            .fixed_pos(screen_pos)
            .movable(false) // We'll handle dragging manually
            .show(ctx, |ui| {
                // Create node frame with header and content area
                let frame = egui::Frame::NONE
                    .fill(egui::Color32::from_rgb(50, 50, 50)) // Dark gray background
                    .stroke(egui::Stroke::new(1.0, egui::Color32::from_gray(100))) // Border
                    .corner_radius(4.0);

                frame.show(ui, |ui| {
                    // Set up layout for node content using centralized constants
                    ui.set_min_size(egui::vec2(layout.width, layout.min_height));

                    // Node header with drag area using centralized constants
                    let (header_response, painter) = ui.allocate_painter(
                        egui::vec2(layout.width, layout.header_height),
                        egui::Sense::drag(),
                    );

                    // Draw header background
                    painter.rect_filled(
                        header_response.rect,
                        4.0,
                        egui::Color32::from_rgb(60, 60, 60),
                    );

                    // Center the title
                    let title_pos = header_response.rect.center();
                    painter.text(
                        title_pos,
                        egui::Align2::CENTER_CENTER,
                        &node_instance.title,
                        egui::FontId::default(),
                        egui::Color32::WHITE,
                    );

                    // Handle dragging
                    if header_response.dragged() {
                        // We'll handle this in the interaction system
                        debug!("Header dragged for node: {:?}", node_instance.node_id);
                    }

                    ui.add_space(layout.header_height);

                    // Create a painter for the current content area
                    let content_painter = ui.painter();

                    // Draw input pins on the left with labels
                    for (i, input_pin) in node_instance.inputs.iter().enumerate() {
                        // Calculate pin position using centralized layout constants
                        let pin_x = -layout.pin_margin; // Slightly outside of node border
                        let pin_y = layout.header_height
                            + layout.pin_spacing
                            + (i as f32 * layout.pin_spacing);

                        let pin_pos = header_response.rect.min + egui::vec2(pin_x, pin_y);

                        // Draw pin circle (6px radius as per SPEC.md)
                        content_painter.circle_filled(
                            pin_pos,
                            layout.pin_radius, // 6px radius from NodeLayout
                            egui::Color32::from_gray(200), // Neutral tone
                        );

                        // Draw pin label next to pin
                        let label_pos =
                            pin_pos + egui::vec2(layout.pin_radius + 4.0, -layout.pin_radius / 2.0);
                        content_painter.text(
                            label_pos,
                            egui::Align2::LEFT_CENTER,
                            &input_pin.label,
                            egui::FontId::default(),
                            egui::Color32::WHITE,
                        );
                    }

                    // Draw output pins on the right with labels
                    for (i, output_pin) in node_instance.outputs.iter().enumerate() {
                        // Calculate pin position using centralized layout constants
                        let pin_x = layout.width + layout.pin_margin; // Slightly outside of node border
                        let pin_y = layout.header_height
                            + layout.pin_spacing
                            + (i as f32 * layout.pin_spacing);

                        let pin_pos = header_response.rect.min + egui::vec2(pin_x, pin_y);

                        // Draw pin circle (6px radius as per SPEC.md)
                        content_painter.circle_filled(
                            pin_pos,
                            layout.pin_radius, // 6px radius from NodeLayout
                            egui::Color32::from_gray(200), // Neutral tone
                        );

                        // Draw pin label next to pin (right-aligned)
                        let label_pos = pin_pos
                            + egui::vec2(-layout.pin_radius - 4.0, -layout.pin_radius / 2.0);
                        content_painter.text(
                            label_pos,
                            egui::Align2::RIGHT_CENTER,
                            &output_pin.label,
                            egui::FontId::default(),
                            egui::Color32::WHITE,
                        );
                    }

                    // Reserve space for internal content area (empty for now as per SPEC.md)
                    let content_height =
                        (node_instance.inputs.len().max(node_instance.outputs.len()) as f32
                            * layout.pin_spacing)
                            + 20.0;
                    ui.add_space(content_height);
                });
            });
    }
}
