use crate::node_graph::model::{CanvasState, NodeGraph, NodeId, NodeLayout, PinId};
use bevy::prelude::*;

/// Centralized pin position manager - single source of truth for all pin positions
#[derive(Resource, Default)]
pub struct PinPositionManager {
    /// Cache of calculated pin positions in screen space for the current frame
    pub cached_positions: std::collections::HashMap<PinId, Vec2>,
    /// Cache invalidation marker
    pub frame_version: u64,
    /// Layout constants
    pub layout: NodeLayout,
}

impl PinPositionManager {
    /// Get pin position in screen space - single point of authority
    pub fn get_pin_screen_position(
        &mut self,
        pin_id: PinId,
        node_graph: &NodeGraph,
        canvas_state: &CanvasState,
    ) -> Option<Vec2> {
        let _zoom = canvas_state.zoom;

        // Check cache first
        if let Some(&cached_pos) = self.cached_positions.get(&pin_id) {
            return Some(cached_pos);
        }

        // Calculate position if not in cache
        let position = self.calculate_pin_position_raw(pin_id, node_graph, canvas_state);

        // Cache the result
        if let Some(pos) = position {
            self.cached_positions.insert(pin_id, pos);
        }

        position
    }

    /// Calculate pin position in screen space (internal calculation)
    pub fn calculate_pin_position_raw(
        &self,
        pin_id: PinId,
        node_graph: &NodeGraph,
        canvas_state: &CanvasState,
    ) -> Option<Vec2> {
        let zoom = canvas_state.zoom;

        for (_, node) in &node_graph.nodes {
            // Check traditional pins - use centralized layout matching render system
            for (i, input_pin) in node.inputs.iter().enumerate() {
                if input_pin.pin_id == pin_id {
                    // Pin position matches render system: outside left margin, below header
                    let pin_offset = Vec2::new(
                        -self.layout.pin_margin,
                        self.layout.header_height
                            + self.layout.pin_spacing
                            + (i as f32 * self.layout.pin_spacing),
                    );
                    let screen_pos = ((node.position + canvas_state.offset) * zoom) + pin_offset;
                    return Some(screen_pos);
                }
            }
            for (i, output_pin) in node.outputs.iter().enumerate() {
                if output_pin.pin_id == pin_id {
                    // Pin position matches render system: outside right margin, below header
                    let pin_offset = Vec2::new(
                        self.layout.width + self.layout.pin_margin,
                        self.layout.header_height
                            + self.layout.pin_spacing
                            + (i as f32 * self.layout.pin_spacing),
                    );
                    let screen_pos = ((node.position + canvas_state.offset) * zoom) + pin_offset;
                    return Some(screen_pos);
                }
            }
        }
        None
    }

    /// Invalidate cache (call this when nodes move or canvas state changes)
    pub fn invalidate_cache(&mut self) {
        self.frame_version += 1;
        // Always clear cache every frame to ensure connections follow nodes
        self.cached_positions.clear();
    }

    /// Get pin position in canvas space (for interactions)
    pub fn get_pin_canvas_position(
        &mut self,
        pin_id: PinId,
        node_graph: &NodeGraph,
        canvas_state: &CanvasState,
    ) -> Option<Vec2> {
        let screen_pos = self.get_pin_screen_position(pin_id, node_graph, canvas_state)?;
        Some(screen_pos / canvas_state.zoom - canvas_state.offset)
    }

    /// Get connection endpoints for drawing (returns screen space positions)
    pub fn get_connection_endpoints(
        &mut self,
        from_pin: PinId,
        to_pin: PinId,
        node_graph: &NodeGraph,
        canvas_state: &CanvasState,
    ) -> Option<(Vec2, Vec2)> {
        let from_pos = self.get_pin_screen_position(from_pin, node_graph, canvas_state)?;
        let to_pos = self.get_pin_screen_position(to_pin, node_graph, canvas_state)?;
        Some((from_pos, to_pos))
    }

    /// Check if pin is an Input Node
    pub fn is_input_node(pin_id: PinId) -> bool {
        pin_id.0 % 2 == 0
    }

    /// Check if pin is an Output Node  
    pub fn is_output_node(pin_id: PinId) -> bool {
        pin_id.0 % 2 == 1
    }

    /// Get the node that owns this pin
    pub fn get_pin_owner_node(&self, pin_id: PinId, node_graph: &NodeGraph) -> Option<NodeId> {
        for (node_id, node) in &node_graph.nodes {
            if node.inputs.iter().any(|p| p.pin_id == pin_id)
                || node.outputs.iter().any(|p| p.pin_id == pin_id)
            {
                return Some(*node_id);
            }
        }
        None
    }

    /// Check if two pins can connect (Output->Input only, cross-window only)
    pub fn can_connect_pins(&self, from_pin: PinId, to_pin: PinId, node_graph: &NodeGraph) -> bool {
        // Must be Output->Input
        if !Self::is_output_node(from_pin) || !Self::is_input_node(to_pin) {
            return false;
        }

        // Must be cross-window (different nodes)
        let from_node = match self.get_pin_owner_node(from_pin, node_graph) {
            Some(node) => node,
            None => return false,
        };
        let to_node = match self.get_pin_owner_node(to_pin, node_graph) {
            Some(node) => node,
            None => return false,
        };

        from_node != to_node
    }
}
