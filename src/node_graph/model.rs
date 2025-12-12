use bevy::prelude::*;
use std::collections::HashMap;

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
