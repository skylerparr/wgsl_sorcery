#[cfg(test)]
mod tests {
    use crate::node_graph::model::{InputPin, NodeGraph, NodeId, NodeInstance, OutputPin, PinId};
    use crate::node_graph::node_factory::{MathOperation, NodeFactory};
    use bevy::prelude::*;

    #[test]
    fn test_create_test_node() {
        let node_id = NodeId(5);
        let position = Vec2::new(100.0, 200.0);
        let next_pin_id = 10;

        let (node, returned_pin_id) = NodeFactory::create_test_node(node_id, position, next_pin_id);

        // Check basic node properties
        assert_eq!(node.node_id, node_id);
        assert_eq!(node.position, position);
        assert_eq!(node.title, "TestNode 5");
        assert_eq!(node.size.x, 220.0); // Default width
        assert_eq!(node.size.y, 100.0); // Default min height
        assert_eq!(node.header_height, 24.0); // Default header height

        // Check pins
        assert_eq!(node.inputs.len(), 2);
        assert_eq!(node.outputs.len(), 1);

        // Check input pins
        assert_eq!(node.inputs[0].pin_id, PinId(10));
        assert_eq!(node.inputs[0].label, "A");
        assert_eq!(node.inputs[0].parent_node, node_id);

        assert_eq!(node.inputs[1].pin_id, PinId(11));
        assert_eq!(node.inputs[1].label, "B");
        assert_eq!(node.inputs[1].parent_node, node_id);

        // Check output pin
        assert_eq!(node.outputs[0].pin_id, PinId(12));
        assert_eq!(node.outputs[0].label, "Out");
        assert_eq!(node.outputs[0].parent_node, node_id);

        // Check returned next pin ID
        assert_eq!(returned_pin_id, 13); // 10 + 3 pins created
    }

    #[test]
    fn test_create_math_node_add() {
        let node_id = NodeId(1);
        let position = Vec2::new(50.0, 75.0);
        let next_pin_id = 5;

        let (node, returned_pin_id) =
            NodeFactory::create_math_node(node_id, position, MathOperation::Add, next_pin_id);

        assert_eq!(node.node_id, node_id);
        assert_eq!(node.position, position);
        assert_eq!(node.title, "Add 1");
        assert_eq!(node.inputs.len(), 2);
        assert_eq!(node.outputs.len(), 1);

        // Check output label for add operation
        assert_eq!(node.outputs[0].label, "Add");

        assert_eq!(returned_pin_id, 8); // 5 + 3 pins created
    }

    #[test]
    fn test_create_math_node_subtract() {
        let node_id = NodeId(2);
        let position = Vec2::new(150.0, 250.0);
        let next_pin_id = 20;

        let (node, returned_pin_id) =
            NodeFactory::create_math_node(node_id, position, MathOperation::Subtract, next_pin_id);

        assert_eq!(node.title, "Subtract 2");
        assert_eq!(node.outputs[0].label, "Sub");
        assert_eq!(returned_pin_id, 23); // 20 + 3 pins created
    }

    #[test]
    fn test_create_math_node_multiply() {
        let node_id = NodeId(3);
        let position = Vec2::new(0.0, 0.0);
        let next_pin_id = 100;

        let (node, returned_pin_id) =
            NodeFactory::create_math_node(node_id, position, MathOperation::Multiply, next_pin_id);

        assert_eq!(node.title, "Multiply 3");
        assert_eq!(node.outputs[0].label, "Mul");
        assert_eq!(returned_pin_id, 103); // 100 + 3 pins created
    }

    #[test]
    fn test_create_math_node_divide() {
        let node_id = NodeId(4);
        let position = Vec2::new(-50.0, -100.0);
        let next_pin_id = 0;

        let (node, returned_pin_id) =
            NodeFactory::create_math_node(node_id, position, MathOperation::Divide, next_pin_id);

        assert_eq!(node.title, "Divide 4");
        assert_eq!(node.outputs[0].label, "Div");
        assert_eq!(returned_pin_id, 3); // 0 + 3 pins created
    }

    #[test]
    fn test_create_constant_node() {
        let node_id = NodeId(10);
        let position = Vec2::new(300.0, 400.0);
        let value = 3.14159;
        let next_pin_id = 50;

        let (node, returned_pin_id) =
            NodeFactory::create_constant_node(node_id, position, value, next_pin_id);

        assert_eq!(node.node_id, node_id);
        assert_eq!(node.position, position);
        assert_eq!(node.title, "Const 10");
        assert_eq!(node.size.x, 220.0); // Default width
        assert_eq!(node.size.y, 100.0); // Default min height
        assert_eq!(node.header_height, 24.0); // Default header height

        // Constant nodes should have no inputs
        assert_eq!(node.inputs.len(), 0);

        // Should have one output
        assert_eq!(node.outputs.len(), 1);
        assert_eq!(node.outputs[0].pin_id, PinId(50));
        assert_eq!(node.outputs[0].label, "3.14"); // Formatted to 2 decimal places
        assert_eq!(node.outputs[0].parent_node, node_id);

        assert_eq!(returned_pin_id, 51); // 50 + 1 pin created
    }

    #[test]
    fn test_create_constant_node_negative_value() {
        let node_id = NodeId(15);
        let position = Vec2::new(0.0, 0.0);
        let value = -2.5;
        let next_pin_id = 75;

        let (node, returned_pin_id) =
            NodeFactory::create_constant_node(node_id, position, value, next_pin_id);

        assert_eq!(node.outputs[0].label, "-2.50");
        assert_eq!(returned_pin_id, 76);
    }

    #[test]
    fn test_create_constant_node_zero_value() {
        let node_id = NodeId(20);
        let position = Vec2::new(0.0, 0.0);
        let value = 0.0;
        let next_pin_id = 80;

        let (node, returned_pin_id) =
            NodeFactory::create_constant_node(node_id, position, value, next_pin_id);

        assert_eq!(node.outputs[0].label, "0.00");
        assert_eq!(returned_pin_id, 81);
    }

    #[test]
    fn test_get_next_node_id_empty_graph() {
        let graph = NodeGraph::new();
        let next_id = NodeFactory::get_next_node_id(&graph);

        assert_eq!(next_id, NodeId(1)); // First ID should be 1
    }

    #[test]
    fn test_get_next_node_id_with_existing_nodes() {
        let mut graph = NodeGraph::new();

        // Add some nodes with specific IDs
        let node1 = create_dummy_node(NodeId(5));
        let node2 = create_dummy_node(NodeId(10));
        let node3 = create_dummy_node(NodeId(3));

        graph.add_node(node1);
        graph.add_node(node2);
        graph.add_node(node3);

        let next_id = NodeFactory::get_next_node_id(&graph);
        assert_eq!(next_id, NodeId(11)); // Should be max(5,10,3) + 1 = 11
    }

    #[test]
    fn test_get_next_node_id_single_node() {
        let mut graph = NodeGraph::new();

        let node1 = create_dummy_node(NodeId(1));
        graph.add_node(node1);

        let next_id = NodeFactory::get_next_node_id(&graph);
        assert_eq!(next_id, NodeId(2)); // Should be 1 + 1 = 2
    }

    #[test]
    fn test_get_next_pin_id_empty_graph() {
        let graph = NodeGraph::new();
        let next_id = NodeFactory::get_next_pin_id(&graph);

        assert_eq!(next_id, 1); // First pin ID should be 1
    }

    #[test]
    fn test_get_next_pin_id_with_existing_pins() {
        let mut graph = NodeGraph::new();

        // Create nodes with various pin IDs
        let mut node1 = create_dummy_node(NodeId(1));
        node1.inputs = vec![
            InputPin {
                pin_id: PinId(0),
                label: "A".to_string(),
                parent_node: NodeId(1),
            },
            InputPin {
                pin_id: PinId(2),
                label: "B".to_string(),
                parent_node: NodeId(1),
            },
        ];
        node1.outputs = vec![OutputPin {
            pin_id: PinId(1),
            label: "Out".to_string(),
            parent_node: NodeId(1),
        }];

        let mut node2 = create_dummy_node(NodeId(2));
        node2.inputs = vec![InputPin {
            pin_id: PinId(4),
            label: "C".to_string(),
            parent_node: NodeId(2),
        }];
        node2.outputs = vec![
            OutputPin {
                pin_id: PinId(5),
                label: "Out2".to_string(),
                parent_node: NodeId(2),
            },
            OutputPin {
                pin_id: PinId(7),
                label: "Out3".to_string(),
                parent_node: NodeId(2),
            },
        ];

        graph.add_node(node1);
        graph.add_node(node2);

        let next_id = NodeFactory::get_next_pin_id(&graph);
        assert_eq!(next_id, 8); // Should be max(0,2,1,4,5,7) + 1 = 8
    }

    #[test]
    fn test_get_next_pin_id_single_pin() {
        let mut graph = NodeGraph::new();

        let mut node1 = create_dummy_node(NodeId(1));
        node1.outputs = vec![OutputPin {
            pin_id: PinId(3),
            label: "Out".to_string(),
            parent_node: NodeId(1),
        }];

        graph.add_node(node1);

        let next_id = NodeFactory::get_next_pin_id(&graph);
        assert_eq!(next_id, 4); // Should be 3 + 1 = 4
    }

    #[test]
    fn test_math_operation_labels() {
        let position = Vec2::new(0.0, 0.0);

        let (add_node, _) =
            NodeFactory::create_math_node(NodeId(1), position, MathOperation::Add, 0);
        assert_eq!(add_node.outputs[0].label, "Add");

        let (sub_node, _) =
            NodeFactory::create_math_node(NodeId(2), position, MathOperation::Subtract, 0);
        assert_eq!(sub_node.outputs[0].label, "Sub");

        let (mul_node, _) =
            NodeFactory::create_math_node(NodeId(3), position, MathOperation::Multiply, 0);
        assert_eq!(mul_node.outputs[0].label, "Mul");

        let (div_node, _) =
            NodeFactory::create_math_node(NodeId(4), position, MathOperation::Divide, 0);
        assert_eq!(div_node.outputs[0].label, "Div");
    }

    #[test]
    fn test_node_factory_consistency() {
        // Test that all factory methods create nodes with consistent properties
        let position = Vec2::new(100.0, 200.0);

        let (test_node, _) = NodeFactory::create_test_node(NodeId(1), position, 0);
        let (math_node, _) =
            NodeFactory::create_math_node(NodeId(2), position, MathOperation::Add, 3);
        let (const_node, _) = NodeFactory::create_constant_node(NodeId(3), position, 1.0, 6);

        // All nodes should have the same basic size properties
        assert_eq!(test_node.size, math_node.size);
        assert_eq!(test_node.size, const_node.size);
        assert_eq!(test_node.header_height, math_node.header_height);
        assert_eq!(test_node.header_height, const_node.header_height);
        assert_eq!(test_node.pin_offsets, math_node.pin_offsets);
        assert_eq!(test_node.pin_offsets, const_node.pin_offsets);
    }

    #[test]
    fn test_pin_id_sequence() {
        let position = Vec2::new(0.0, 0.0);
        let mut next_pin_id = 0;

        // Create multiple nodes and verify pin IDs are sequential
        let (test_node, next_pin_id) =
            NodeFactory::create_test_node(NodeId(1), position, next_pin_id);
        assert_eq!(next_pin_id, 3);

        let (math_node, next_pin_id) =
            NodeFactory::create_math_node(NodeId(2), position, MathOperation::Add, next_pin_id);
        assert_eq!(next_pin_id, 6);

        let (const_node, next_pin_id) =
            NodeFactory::create_constant_node(NodeId(3), position, 2.5, next_pin_id);
        assert_eq!(next_pin_id, 7);

        // Verify that pin IDs are correct
        assert_eq!(test_node.inputs[0].pin_id, PinId(0));
        assert_eq!(test_node.inputs[1].pin_id, PinId(1));
        assert_eq!(test_node.outputs[0].pin_id, PinId(2));

        assert_eq!(math_node.inputs[0].pin_id, PinId(3));
        assert_eq!(math_node.inputs[1].pin_id, PinId(4));
        assert_eq!(math_node.outputs[0].pin_id, PinId(5));

        assert_eq!(const_node.outputs[0].pin_id, PinId(6));
    }

    // Helper function to create a dummy node for testing
    fn create_dummy_node(node_id: NodeId) -> NodeInstance {
        NodeInstance {
            node_id,
            position: Vec2::new(0.0, 0.0),
            inputs: vec![],
            outputs: vec![],
            title: "Dummy".to_string(),
            size: Vec2::new(220.0, 100.0),
            header_height: 24.0,
            pin_offsets: (vec![], vec![]),
        }
    }
}
