use bevy::prelude::*;
use std::collections::HashMap;

/// Centralized layout constants for node rendering - single source of truth
#[derive(Debug, Clone)]
pub struct NodeLayout {
    pub width: f32,
    pub min_height: f32,
    pub header_height: f32,
    pub pin_radius: f32,
    pub input_node_offset: Vec2,
    pub output_node_offset: Vec2,
    pub pin_spacing: f32,
    pub pin_margin: f32,
}

impl Default for NodeLayout {
    fn default() -> Self {
        Self {
            width: 220.0,
            min_height: 100.0,
            header_height: 24.0,
            pin_radius: 6.0,
            input_node_offset: Vec2::new(50.0, 70.0),
            output_node_offset: Vec2::new(170.0, 70.0),
            pin_spacing: 20.0,
            pin_margin: 6.0,
        }
    }
}

// Unique identifiers for nodes and pins
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct NodeId(pub u32);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct PinId(pub u32);

#[derive(Debug, Clone)]
pub struct InputPin {
    pub pin_id: PinId,
    pub label: String,
    pub parent_node: NodeId,
}

#[derive(Debug, Clone)]
pub struct OutputPin {
    pub pin_id: PinId,
    pub label: String,
    pub parent_node: NodeId,
}

#[derive(Debug, Clone, Component)]
pub struct NodeInstance {
    pub node_id: NodeId,
    pub position: Vec2,
    pub inputs: Vec<InputPin>,
    pub outputs: Vec<OutputPin>,
    pub title: String,
    pub size: Vec2,
    pub header_height: f32,
    pub pin_offsets: (Vec<(PinId, Vec2)>, Vec<(PinId, Vec2)>), // (input_pin_offsets, output_pin_offsets)
}

#[derive(Debug, Clone, PartialEq)]
pub struct Connection {
    pub from_pin: PinId,
    pub to_pin: PinId,
}

#[derive(Debug, Clone, Resource, Default)]
pub struct NodeGraph {
    pub nodes: HashMap<NodeId, NodeInstance>,
    pub connections: Vec<Connection>,
    pub canvas_state: CanvasState,
}

#[derive(Debug, Clone)]
pub struct CanvasState {
    pub zoom: f32,
    pub offset: Vec2,
}

impl Default for CanvasState {
    fn default() -> Self {
        Self {
            zoom: 1.0,
            offset: Vec2::ZERO,
        }
    }
}

impl NodeGraph {
    pub fn new() -> Self {
        Self {
            nodes: HashMap::new(),
            connections: Vec::new(),
            canvas_state: CanvasState::default(),
        }
    }

    pub fn add_node(&mut self, node: NodeInstance) {
        self.nodes.insert(node.node_id, node);
    }

    pub fn remove_node(&mut self, node_id: NodeId) {
        self.nodes.remove(&node_id);
        // Remove connections related to this node
        self.connections.retain(|_conn| {
            // Check if the nodes connected to this connection still exist
            true // Placeholder - actual implementation would check node existence
        });
    }

    pub fn add_connection(&mut self, connection: Connection) {
        self.connections.push(connection);
    }

    pub fn remove_connection(&mut self, connection: &Connection) {
        self.connections.retain(|c| c != connection);
    }
}
