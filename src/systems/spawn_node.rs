use crate::node_graph::model::{InputPin, NodeGraph, NodeId, NodeInstance, OutputPin, PinId};
use bevy::prelude::*;

pub fn spawn_test_node_system(mut node_graph: ResMut<NodeGraph>, input: Res<ButtonInput<KeyCode>>) {
    // Check if 'N' key was pressed
    if input.just_pressed(KeyCode::KeyN) {
        warn!("DEBUG: N key pressed detected");
        // Create a simple test node
        let node_id = NodeId(0);
        let input_pin1 = InputPin {
            pin_id: PinId(0),
            label: "A".to_string(),
            parent_node: node_id,
        };
        let input_pin2 = InputPin {
            pin_id: PinId(1),
            label: "B".to_string(),
            parent_node: node_id,
        };
        let output_pin = OutputPin {
            pin_id: PinId(2),
            label: "Out".to_string(),
            parent_node: node_id,
        };

        let node_instance = NodeInstance {
            node_id,
            position: Vec2::ZERO,
            inputs: vec![input_pin1, input_pin2],
            outputs: vec![output_pin],
            title: "TestNode".to_string(),
            size: Vec2::new(220.0, 100.0),
            header_height: 24.0,
            pin_offsets: (vec![], vec![]), // Will be populated by rendering system
        };

        warn!("DEBUG: Adding node to graph");
        node_graph.add_node(node_instance);
        warn!(
            "DEBUG: Node added successfully, node count: {}",
            node_graph.nodes.len()
        );
    }
}
