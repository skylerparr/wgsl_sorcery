#[cfg(test)]
mod tests {
    use crate::node_graph::model::{
        CanvasState, Connection, InputPin, NodeGraph, NodeId, NodeInstance, NodeLayout, OutputPin,
        PinId,
    };
    use bevy::prelude::*;

    #[test]
    fn test_node_layout_default() {
        let layout = NodeLayout::default();
        assert_eq!(layout.width, 220.0);
        assert_eq!(layout.min_height, 100.0);
        assert_eq!(layout.header_height, 24.0);
        assert_eq!(layout.pin_radius, 6.0);
        assert_eq!(layout.input_node_offset, Vec2::new(50.0, 70.0));
        assert_eq!(layout.output_node_offset, Vec2::new(170.0, 70.0));
        assert_eq!(layout.pin_spacing, 20.0);
        assert_eq!(layout.pin_margin, 6.0);
    }

    #[test]
    fn test_node_layout_clone() {
        let layout = NodeLayout::default();
        let cloned = layout.clone();
        assert_eq!(layout.width, cloned.width);
        assert_eq!(layout.min_height, cloned.min_height);
        assert_eq!(layout.header_height, cloned.header_height);
    }

    #[test]
    fn test_node_id_equality() {
        let id1 = NodeId(1);
        let id2 = NodeId(1);
        let id3 = NodeId(2);

        assert_eq!(id1, id2);
        assert_ne!(id1, id3);
    }

    #[test]
    fn test_node_id_hash() {
        use std::hash::{Hash, Hasher};
        let mut hasher1 = std::collections::hash_map::DefaultHasher::new();
        let mut hasher2 = std::collections::hash_map::DefaultHasher::new();

        let id1 = NodeId(42);
        let id2 = NodeId(42);

        id1.hash(&mut hasher1);
        id2.hash(&mut hasher2);

        assert_eq!(hasher1.finish(), hasher2.finish());
    }

    #[test]
    fn test_pin_id_equality() {
        let id1 = PinId(1);
        let id2 = PinId(1);
        let id3 = PinId(2);

        assert_eq!(id1, id2);
        assert_ne!(id1, id3);
    }

    #[test]
    fn test_pin_id_hash() {
        use std::hash::{Hash, Hasher};
        let mut hasher1 = std::collections::hash_map::DefaultHasher::new();
        let mut hasher2 = std::collections::hash_map::DefaultHasher::new();

        let id1 = PinId(42);
        let id2 = PinId(42);

        id1.hash(&mut hasher1);
        id2.hash(&mut hasher2);

        assert_eq!(hasher1.finish(), hasher2.finish());
    }

    #[test]
    fn test_input_pin_creation() {
        let pin_id = PinId(1);
        let node_id = NodeId(10);
        let input_pin = InputPin {
            pin_id,
            label: "Test Input".to_string(),
            parent_node: node_id,
        };

        assert_eq!(input_pin.pin_id, pin_id);
        assert_eq!(input_pin.label, "Test Input");
        assert_eq!(input_pin.parent_node, node_id);
    }

    #[test]
    fn test_input_pin_clone() {
        let pin_id = PinId(1);
        let node_id = NodeId(10);
        let input_pin = InputPin {
            pin_id,
            label: "Test Input".to_string(),
            parent_node: node_id,
        };

        let cloned = input_pin.clone();
        assert_eq!(input_pin.pin_id, cloned.pin_id);
        assert_eq!(input_pin.label, cloned.label);
        assert_eq!(input_pin.parent_node, cloned.parent_node);
    }

    #[test]
    fn test_output_pin_creation() {
        let pin_id = PinId(2);
        let node_id = NodeId(11);
        let output_pin = OutputPin {
            pin_id,
            label: "Test Output".to_string(),
            parent_node: node_id,
        };

        assert_eq!(output_pin.pin_id, pin_id);
        assert_eq!(output_pin.label, "Test Output");
        assert_eq!(output_pin.parent_node, node_id);
    }

    #[test]
    fn test_output_pin_clone() {
        let pin_id = PinId(2);
        let node_id = NodeId(11);
        let output_pin = OutputPin {
            pin_id,
            label: "Test Output".to_string(),
            parent_node: node_id,
        };

        let cloned = output_pin.clone();
        assert_eq!(output_pin.pin_id, cloned.pin_id);
        assert_eq!(output_pin.label, cloned.label);
        assert_eq!(output_pin.parent_node, cloned.parent_node);
    }

    #[test]
    fn test_node_instance_creation() {
        let node_id = NodeId(1);
        let position = Vec2::new(100.0, 200.0);
        let input_pin = InputPin {
            pin_id: PinId(1),
            label: "Input".to_string(),
            parent_node: node_id,
        };
        let output_pin = OutputPin {
            pin_id: PinId(2),
            label: "Output".to_string(),
            parent_node: node_id,
        };

        let node = NodeInstance {
            node_id,
            position,
            inputs: vec![input_pin],
            outputs: vec![output_pin],
            title: "Test Node".to_string(),
            size: Vec2::new(220.0, 100.0),
            header_height: 24.0,
            pin_offsets: (vec![], vec![]),
        };

        assert_eq!(node.node_id, node_id);
        assert_eq!(node.position, position);
        assert_eq!(node.inputs.len(), 1);
        assert_eq!(node.outputs.len(), 1);
        assert_eq!(node.title, "Test Node");
        assert_eq!(node.size, Vec2::new(220.0, 100.0));
        assert_eq!(node.header_height, 24.0);
    }

    #[test]
    fn test_node_instance_clone() {
        let node_id = NodeId(1);
        let position = Vec2::new(100.0, 200.0);
        let input_pin = InputPin {
            pin_id: PinId(1),
            label: "Input".to_string(),
            parent_node: node_id,
        };
        let output_pin = OutputPin {
            pin_id: PinId(2),
            label: "Output".to_string(),
            parent_node: node_id,
        };

        let node = NodeInstance {
            node_id,
            position,
            inputs: vec![input_pin],
            outputs: vec![output_pin],
            title: "Test Node".to_string(),
            size: Vec2::new(220.0, 100.0),
            header_height: 24.0,
            pin_offsets: (vec![], vec![]),
        };

        let cloned = node.clone();
        assert_eq!(node.node_id, cloned.node_id);
        assert_eq!(node.position, cloned.position);
        assert_eq!(node.inputs.len(), cloned.inputs.len());
        assert_eq!(node.outputs.len(), cloned.outputs.len());
        assert_eq!(node.title, cloned.title);
        assert_eq!(node.size, cloned.size);
        assert_eq!(node.header_height, cloned.header_height);
    }

    #[test]
    fn test_connection_creation() {
        let from_pin = PinId(1);
        let to_pin = PinId(2);
        let connection = Connection { from_pin, to_pin };

        assert_eq!(connection.from_pin, from_pin);
        assert_eq!(connection.to_pin, to_pin);
    }

    #[test]
    fn test_connection_equality() {
        let conn1 = Connection {
            from_pin: PinId(1),
            to_pin: PinId(2),
        };
        let conn2 = Connection {
            from_pin: PinId(1),
            to_pin: PinId(2),
        };
        let conn3 = Connection {
            from_pin: PinId(2),
            to_pin: PinId(1),
        };

        assert_eq!(conn1, conn2);
        assert_ne!(conn1, conn3);
    }

    #[test]
    fn test_connection_clone() {
        let connection = Connection {
            from_pin: PinId(1),
            to_pin: PinId(2),
        };

        let cloned = connection.clone();
        assert_eq!(connection.from_pin, cloned.from_pin);
        assert_eq!(connection.to_pin, cloned.to_pin);
    }

    #[test]
    fn test_canvas_state_default() {
        let canvas_state = CanvasState::default();
        assert_eq!(canvas_state.zoom, 1.0);
        assert_eq!(canvas_state.offset, Vec2::ZERO);
    }

    #[test]
    fn test_canvas_state_creation() {
        let canvas_state = CanvasState {
            zoom: 2.0,
            offset: Vec2::new(100.0, 50.0),
        };

        assert_eq!(canvas_state.zoom, 2.0);
        assert_eq!(canvas_state.offset, Vec2::new(100.0, 50.0));
    }

    #[test]
    fn test_canvas_state_clone() {
        let canvas_state = CanvasState {
            zoom: 2.0,
            offset: Vec2::new(100.0, 50.0),
        };

        let cloned = canvas_state.clone();
        assert_eq!(canvas_state.zoom, cloned.zoom);
        assert_eq!(canvas_state.offset, cloned.offset);
    }

    #[test]
    fn test_node_graph_new() {
        let graph = NodeGraph::new();
        assert!(graph.nodes.is_empty());
        assert!(graph.connections.is_empty());
        assert_eq!(graph.canvas_state.zoom, 1.0);
        assert_eq!(graph.canvas_state.offset, Vec2::ZERO);
    }

    #[test]
    fn test_node_graph_default() {
        let graph = NodeGraph::default();
        assert!(graph.nodes.is_empty());
        assert!(graph.connections.is_empty());
        assert_eq!(graph.canvas_state.zoom, 1.0);
        assert_eq!(graph.canvas_state.offset, Vec2::ZERO);
    }

    #[test]
    fn test_add_node() {
        let mut graph = NodeGraph::new();
        let node_id = NodeId(1);
        let input_pin = InputPin {
            pin_id: PinId(1),
            label: "Input".to_string(),
            parent_node: node_id,
        };
        let output_pin = OutputPin {
            pin_id: PinId(2),
            label: "Output".to_string(),
            parent_node: node_id,
        };

        let node = NodeInstance {
            node_id,
            position: Vec2::new(0.0, 0.0),
            inputs: vec![input_pin],
            outputs: vec![output_pin],
            title: "Test Node".to_string(),
            size: Vec2::new(220.0, 100.0),
            header_height: 24.0,
            pin_offsets: (vec![], vec![]),
        };

        graph.add_node(node);
        assert_eq!(graph.nodes.len(), 1);
        assert!(graph.nodes.contains_key(&node_id));
    }

    #[test]
    fn test_remove_node() {
        let mut graph = NodeGraph::new();
        let node_id = NodeId(1);
        let input_pin = InputPin {
            pin_id: PinId(1),
            label: "Input".to_string(),
            parent_node: node_id,
        };
        let output_pin = OutputPin {
            pin_id: PinId(2),
            label: "Output".to_string(),
            parent_node: node_id,
        };

        let node = NodeInstance {
            node_id,
            position: Vec2::new(0.0, 0.0),
            inputs: vec![input_pin],
            outputs: vec![output_pin],
            title: "Test Node".to_string(),
            size: Vec2::new(220.0, 100.0),
            header_height: 24.0,
            pin_offsets: (vec![], vec![]),
        };

        graph.add_node(node);
        assert_eq!(graph.nodes.len(), 1);

        graph.remove_node(node_id);
        assert_eq!(graph.nodes.len(), 0);
        assert!(!graph.nodes.contains_key(&node_id));
    }

    #[test]
    fn test_add_connection() {
        let mut graph = NodeGraph::new();
        let connection = Connection {
            from_pin: PinId(1),
            to_pin: PinId(2),
        };

        graph.add_connection(connection.clone());
        assert_eq!(graph.connections.len(), 1);
        assert_eq!(graph.connections[0], connection);
    }

    #[test]
    fn test_remove_connection() {
        let mut graph = NodeGraph::new();
        let connection = Connection {
            from_pin: PinId(1),
            to_pin: PinId(2),
        };

        graph.add_connection(connection.clone());
        assert_eq!(graph.connections.len(), 1);

        graph.remove_connection(&connection);
        assert_eq!(graph.connections.len(), 0);
    }

    #[test]
    fn test_remove_nonexistent_connection() {
        let mut graph = NodeGraph::new();
        let connection1 = Connection {
            from_pin: PinId(1),
            to_pin: PinId(2),
        };
        let connection2 = Connection {
            from_pin: PinId(3),
            to_pin: PinId(4),
        };

        graph.add_connection(connection1);
        assert_eq!(graph.connections.len(), 1);

        graph.remove_connection(&connection2);
        assert_eq!(graph.connections.len(), 1); // Should not remove anything
    }

    #[test]
    fn test_node_graph_with_multiple_nodes() {
        let mut graph = NodeGraph::new();

        // Add first node
        let node_id1 = NodeId(1);
        let input_pin1 = InputPin {
            pin_id: PinId(1),
            label: "Input1".to_string(),
            parent_node: node_id1,
        };
        let output_pin1 = OutputPin {
            pin_id: PinId(2),
            label: "Output1".to_string(),
            parent_node: node_id1,
        };

        let node1 = NodeInstance {
            node_id: node_id1,
            position: Vec2::new(0.0, 0.0),
            inputs: vec![input_pin1],
            outputs: vec![output_pin1],
            title: "Node 1".to_string(),
            size: Vec2::new(220.0, 100.0),
            header_height: 24.0,
            pin_offsets: (vec![], vec![]),
        };

        // Add second node
        let node_id2 = NodeId(2);
        let input_pin2 = InputPin {
            pin_id: PinId(3),
            label: "Input2".to_string(),
            parent_node: node_id2,
        };
        let output_pin2 = OutputPin {
            pin_id: PinId(4),
            label: "Output2".to_string(),
            parent_node: node_id2,
        };

        let node2 = NodeInstance {
            node_id: node_id2,
            position: Vec2::new(300.0, 0.0),
            inputs: vec![input_pin2],
            outputs: vec![output_pin2],
            title: "Node 2".to_string(),
            size: Vec2::new(220.0, 100.0),
            header_height: 24.0,
            pin_offsets: (vec![], vec![]),
        };

        graph.add_node(node1);
        graph.add_node(node2);

        assert_eq!(graph.nodes.len(), 2);
        assert!(graph.nodes.contains_key(&node_id1));
        assert!(graph.nodes.contains_key(&node_id2));
    }

    #[test]
    fn test_node_graph_with_multiple_connections() {
        let mut graph = NodeGraph::new();

        let connection1 = Connection {
            from_pin: PinId(1),
            to_pin: PinId(2),
        };
        let connection2 = Connection {
            from_pin: PinId(3),
            to_pin: PinId(4),
        };
        let connection3 = Connection {
            from_pin: PinId(5),
            to_pin: PinId(6),
        };

        graph.add_connection(connection1.clone());
        graph.add_connection(connection2.clone());
        graph.add_connection(connection3.clone());

        assert_eq!(graph.connections.len(), 3);
        assert!(graph.connections.contains(&connection1));
        assert!(graph.connections.contains(&connection2));
        assert!(graph.connections.contains(&connection3));
    }
}
