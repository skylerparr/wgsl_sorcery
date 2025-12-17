use crate::node_graph::model::{InputPin, NodeGraph, NodeId, NodeInstance, OutputPin, PinId};

/// Centralized node factory - single source of truth for all node creation
pub struct NodeFactory;

impl NodeFactory {
    /// Create a new test node with standard configuration
    pub fn create_test_node(
        node_id: NodeId,
        position: bevy::prelude::Vec2,
        next_pin_id: u32,
    ) -> (NodeInstance, u32) {
        let pin_a = PinId(next_pin_id);
        let pin_b = PinId(next_pin_id + 1);
        let pin_out = PinId(next_pin_id + 2);

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

        let layout = crate::node_graph::model::NodeLayout::default();
        let node_instance = NodeInstance {
            node_id,
            position,
            inputs: vec![input_pin1, input_pin2],
            outputs: vec![output_pin],
            title: format!("TestNode {}", node_id.0),
            size: bevy::prelude::Vec2::new(layout.width, layout.min_height),
            header_height: layout.header_height,
            pin_offsets: (vec![], vec![]), // Will be populated by rendering system
        };

        (node_instance, next_pin_id + 3)
    }

    /// Create a basic math operation node (add, subtract, multiply, divide)
    pub fn create_math_node(
        node_id: NodeId,
        position: bevy::prelude::Vec2,
        operation: MathOperation,
        next_pin_id: u32,
    ) -> (NodeInstance, u32) {
        let (title, operation_label) = match operation {
            MathOperation::Add => ("Add", "Add"),
            MathOperation::Subtract => ("Subtract", "Sub"),
            MathOperation::Multiply => ("Multiply", "Mul"),
            MathOperation::Divide => ("Divide", "Div"),
        };

        let pin_a = PinId(next_pin_id);
        let pin_b = PinId(next_pin_id + 1);
        let pin_result = PinId(next_pin_id + 2);

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
            pin_id: pin_result,
            label: operation_label.to_string(),
            parent_node: node_id,
        };

        let layout = crate::node_graph::model::NodeLayout::default();
        let node_instance = NodeInstance {
            node_id,
            position,
            inputs: vec![input_pin1, input_pin2],
            outputs: vec![output_pin],
            title: format!("{} {}", title, node_id.0),
            size: bevy::prelude::Vec2::new(layout.width, layout.min_height),
            header_height: layout.header_height,
            pin_offsets: (vec![], vec![]), // Will be populated by rendering system
        };

        (node_instance, next_pin_id + 3)
    }

    /// Create a constant value node
    pub fn create_constant_node(
        node_id: NodeId,
        position: bevy::prelude::Vec2,
        value: f32,
        next_pin_id: u32,
    ) -> (NodeInstance, u32) {
        let pin_out = PinId(next_pin_id);

        let output_pin = OutputPin {
            pin_id: pin_out,
            label: format!("{:.2}", value),
            parent_node: node_id,
        };

        let layout = crate::node_graph::model::NodeLayout::default();
        let node_instance = NodeInstance {
            node_id,
            position,
            inputs: vec![], // No inputs for constant nodes
            outputs: vec![output_pin],
            title: format!("Const {}", node_id.0),
            size: bevy::prelude::Vec2::new(layout.width, layout.min_height),
            header_height: layout.header_height,
            pin_offsets: (vec![], vec![]), // Will be populated by rendering system
        };

        (node_instance, next_pin_id + 1)
    }

    /// Get the next available node ID from the current graph state
    pub fn get_next_node_id(node_graph: &NodeGraph) -> NodeId {
        NodeId(
            node_graph
                .nodes
                .keys()
                .map(|id| id.0)
                .max()
                .unwrap_or(0)
                .saturating_add(1),
        )
    }

    /// Get the next available pin ID from the current graph state
    pub fn get_next_pin_id(node_graph: &NodeGraph) -> u32 {
        node_graph
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
            .saturating_add(1)
    }
}

/// Supported math operations for math nodes
#[derive(Debug, Clone, Copy)]
pub enum MathOperation {
    Add,
    Subtract,
    Multiply,
    Divide,
}
