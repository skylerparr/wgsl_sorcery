use crate::node_graph::model::{
    InputPin, NodeGraph, NodeId, NodeInstance, NodeLayout, OutputPin, PinId,
};
use bevy::prelude::*;

pub fn spawn_test_node_system(mut node_graph: ResMut<NodeGraph>, input: Res<ButtonInput<KeyCode>>) {
    // Check if 'N' key was pressed
    if input.just_pressed(KeyCode::KeyN) {
        warn!("DEBUG: N key pressed detected");

        // Allocate unique ids based on current graph contents.
        // (This is deterministic and avoids needing extra counters.)
        let next_node_id = node_graph
            .nodes
            .keys()
            .map(|id| id.0)
            .max()
            .unwrap_or(0)
            .saturating_add(1);

        let next_pin_id = node_graph
            .nodes
            .values()
            .flat_map(|n| {
                n.inputs
                    .iter()
                    .map(|p| p.pin_id.0)
                    .chain(n.outputs.iter().map(|p| p.pin_id.0))
            })
            .max()
            .unwrap_or(0)
            .saturating_add(1);

        let node_id = NodeId(next_node_id);
        let pin_a = PinId(next_pin_id);
        let pin_b = PinId(next_pin_id + 1);
        let pin_out = PinId(next_pin_id + 2);

        // Spawn in front of the camera center in canvas space so it’s visible.
        // For now: offset each new node a bit so they don’t stack perfectly.
        let spawn_pos = Vec2::new((next_node_id as f32) * 40.0, (next_node_id as f32) * 20.0);

        let input_pin1 = InputPin {
            pin_id: pin_a,
            label: "A".to_string(),
            parent_node: node_id,
        };
        let input_pin2 = InputPin {
            pin_id: pin_b,
            label: "B".to_string(),
            parent_node: node_id,
        };
        let output_pin = OutputPin {
            pin_id: pin_out,
            label: "Out".to_string(),
            parent_node: node_id,
        };

        let layout = NodeLayout::default();
        let node_instance = NodeInstance {
            node_id,
            position: spawn_pos,
            inputs: vec![input_pin1, input_pin2],
            outputs: vec![output_pin],
            title: format!("TestNode {}", next_node_id),
            size: Vec2::new(layout.width, layout.min_height),
            header_height: layout.header_height,
            pin_offsets: (vec![], vec![]), // Will be populated by rendering system
        };

        warn!(
            "DEBUG: Adding node {:?} at {:?} with pins A={:?} B={:?} Out={:?}",
            node_id, spawn_pos, pin_a, pin_b, pin_out
        );
        node_graph.add_node(node_instance);
        warn!(
            "DEBUG: Node added successfully, node count: {}",
            node_graph.nodes.len()
        );
    }
}
