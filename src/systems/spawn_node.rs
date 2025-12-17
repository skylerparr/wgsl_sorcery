use crate::node_graph::model::NodeGraph;
use crate::node_graph::node_factory::{MathOperation, NodeFactory};
use bevy::prelude::*;

/// Centralized node spawning system - single source of truth for all node creation
pub fn spawn_test_node_system(mut node_graph: ResMut<NodeGraph>, input: Res<ButtonInput<KeyCode>>) {
    // Check if 'N' key was pressed - spawn test node
    if input.just_pressed(KeyCode::KeyN) {
        info!("SPAWN: Creating test node");

        let node_id = NodeFactory::get_next_node_id(&node_graph);
        let next_pin_id = NodeFactory::get_next_pin_id(&node_graph);

        // Spawn in canvas space with offset to prevent stacking
        let spawn_pos = Vec2::new((node_id.0 as f32) * 40.0, (node_id.0 as f32) * 20.0);

        let (node_instance, _next_pin_id) =
            NodeFactory::create_test_node(node_id, spawn_pos, next_pin_id);

        info!(
            "SPAWN: Added test node {:?} at {:?}",
            node_instance.node_id, node_instance.position
        );
        node_graph.add_node(node_instance);
        info!(
            "SPAWN: Node added successfully, total nodes: {}",
            node_graph.nodes.len()
        );
    }

    // Check if 'M' key was pressed - spawn math node
    if input.just_pressed(KeyCode::KeyM) {
        info!("SPAWN: Creating math node (add)");

        let node_id = NodeFactory::get_next_node_id(&node_graph);
        let next_pin_id = NodeFactory::get_next_pin_id(&node_graph);

        // Spawn in canvas space with offset to prevent stacking
        let spawn_pos = Vec2::new(
            (node_id.0 as f32) * 40.0 + 200.0,
            (node_id.0 as f32) * 20.0 + 100.0,
        );

        let (node_instance, _next_pin_id) =
            NodeFactory::create_math_node(node_id, spawn_pos, MathOperation::Add, next_pin_id);

        info!(
            "SPAWN: Added math node {:?} at {:?}",
            node_instance.node_id, node_instance.position
        );
        node_graph.add_node(node_instance);
        info!(
            "SPAWN: Math node added successfully, total nodes: {}",
            node_graph.nodes.len()
        );
    }

    // Check if 'C' key was pressed - spawn constant node
    if input.just_pressed(KeyCode::KeyC) {
        info!("SPAWN: Creating constant node");

        let node_id = NodeFactory::get_next_node_id(&node_graph);
        let next_pin_id = NodeFactory::get_next_pin_id(&node_graph);

        // Spawn in canvas space with offset to prevent stacking
        let spawn_pos = Vec2::new(
            (node_id.0 as f32) * 40.0 + 400.0,
            (node_id.0 as f32) * 20.0 + 200.0,
        );

        let (node_instance, _next_pin_id) =
            NodeFactory::create_constant_node(node_id, spawn_pos, 3.14, next_pin_id);

        info!(
            "SPAWN: Added constant node {:?} at {:?}",
            node_instance.node_id, node_instance.position
        );
        node_graph.add_node(node_instance);
        info!(
            "SPAWN: Constant node added successfully, total nodes: {}",
            node_graph.nodes.len()
        );
    }
}
