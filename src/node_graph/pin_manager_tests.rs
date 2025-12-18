#[cfg(test)]
mod tests {
    use crate::node_graph::model::{
        CanvasState, InputPin, NodeGraph, NodeId, NodeInstance, NodeLayout, OutputPin, PinId,
    };
    use crate::node_graph::pin_manager::PinPositionManager;
    use bevy::prelude::*;

    fn create_test_node_graph() -> NodeGraph {
        let mut graph = NodeGraph::new();

        // Create first test node
        let node_id1 = NodeId(1);
        let position1 = Vec2::new(100.0, 200.0);

        let input_pin1 = InputPin {
            pin_id: PinId(0), // Even = input
            label: "A".to_string(),
            parent_node: node_id1,
        };
        let input_pin2 = InputPin {
            pin_id: PinId(2), // Even = input
            label: "B".to_string(),
            parent_node: node_id1,
        };
        let output_pin1 = OutputPin {
            pin_id: PinId(1), // Odd = output
            label: "Out".to_string(),
            parent_node: node_id1,
        };
        let output_pin2 = OutputPin {
            pin_id: PinId(3), // Odd = output
            label: "Out2".to_string(),
            parent_node: node_id1,
        };

        let node1 = NodeInstance {
            node_id: node_id1,
            position: position1,
            inputs: vec![input_pin1, input_pin2],
            outputs: vec![output_pin1, output_pin2],
            title: "Test Node 1".to_string(),
            size: Vec2::new(220.0, 100.0),
            header_height: 24.0,
            pin_offsets: (vec![], vec![]),
        };

        // Create second test node
        let node_id2 = NodeId(2);
        let position2 = Vec2::new(400.0, 300.0);

        let input_pin3 = InputPin {
            pin_id: PinId(4), // Even = input
            label: "C".to_string(),
            parent_node: node_id2,
        };
        let output_pin3 = OutputPin {
            pin_id: PinId(5), // Odd = output
            label: "Out".to_string(),
            parent_node: node_id2,
        };

        let node2 = NodeInstance {
            node_id: node_id2,
            position: position2,
            inputs: vec![input_pin3],
            outputs: vec![output_pin3],
            title: "Test Node 2".to_string(),
            size: Vec2::new(220.0, 100.0),
            header_height: 24.0,
            pin_offsets: (vec![], vec![]),
        };

        graph.add_node(node1);
        graph.add_node(node2);
        graph
    }

    fn create_test_canvas_state() -> CanvasState {
        CanvasState {
            zoom: 1.0,
            offset: Vec2::new(50.0, 25.0),
        }
    }

    #[test]
    fn test_pin_position_manager_default() {
        let manager = PinPositionManager::default();
        assert!(manager.cached_positions.is_empty());
        assert_eq!(manager.frame_version, 0);
        assert_eq!(manager.current_frame, 0);
        assert_eq!(manager.layout.width, 220.0);
    }

    #[test]
    fn test_is_input_node() {
        assert!(PinPositionManager::is_input_node(PinId(0))); // Even
        assert!(PinPositionManager::is_input_node(PinId(2))); // Even
        assert!(PinPositionManager::is_input_node(PinId(4))); // Even
        assert!(!PinPositionManager::is_input_node(PinId(1))); // Odd
        assert!(!PinPositionManager::is_input_node(PinId(3))); // Odd
    }

    #[test]
    fn test_is_output_node() {
        assert!(PinPositionManager::is_output_node(PinId(1))); // Odd
        assert!(PinPositionManager::is_output_node(PinId(3))); // Odd
        assert!(PinPositionManager::is_output_node(PinId(5))); // Odd
        assert!(!PinPositionManager::is_output_node(PinId(0))); // Even
        assert!(!PinPositionManager::is_output_node(PinId(2))); // Even
    }

    #[test]
    fn test_get_pin_owner_node() {
        let manager = PinPositionManager::default();
        let graph = create_test_node_graph();

        // Test valid pins
        assert_eq!(
            manager.get_pin_owner_node(PinId(0), &graph),
            Some(NodeId(1))
        );
        assert_eq!(
            manager.get_pin_owner_node(PinId(1), &graph),
            Some(NodeId(1))
        );
        assert_eq!(
            manager.get_pin_owner_node(PinId(2), &graph),
            Some(NodeId(1))
        );
        assert_eq!(
            manager.get_pin_owner_node(PinId(3), &graph),
            Some(NodeId(1))
        );

        // Test invalid pin
        assert_eq!(manager.get_pin_owner_node(PinId(99), &graph), None);
    }

    #[test]
    fn test_can_connect_pins_valid() {
        let manager = PinPositionManager::default();
        let graph = create_test_node_graph();

        // Output to input, different nodes - should be valid
        assert!(manager.can_connect_pins(PinId(1), PinId(4), &graph)); // Node 1 output to Node 2 input
    }

    #[test]
    fn test_can_connect_pins_invalid_direction() {
        let manager = PinPositionManager::default();
        let graph = create_test_node_graph();

        // Input to output - should be invalid
        assert!(!manager.can_connect_pins(PinId(0), PinId(1), &graph));
        assert!(!manager.can_connect_pins(PinId(2), PinId(3), &graph));

        // Input to input - should be invalid
        assert!(!manager.can_connect_pins(PinId(0), PinId(2), &graph));

        // Output to output - should be invalid
        assert!(!manager.can_connect_pins(PinId(1), PinId(3), &graph));
    }

    #[test]
    fn test_can_connect_pins_same_node() {
        let manager = PinPositionManager::default();
        let graph = create_test_node_graph();

        // Same node connections - should be invalid
        assert!(!manager.can_connect_pins(PinId(1), PinId(0), &graph)); // Both Node 1
        assert!(!manager.can_connect_pins(PinId(3), PinId(2), &graph)); // Both Node 1
    }

    #[test]
    fn test_can_connect_pins_nonexistent() {
        let manager = PinPositionManager::default();
        let graph = create_test_node_graph();

        // Non-existent pins - should be invalid
        assert!(!manager.can_connect_pins(PinId(99), PinId(100), &graph));
    }

    #[test]
    fn test_invalidate_cache() {
        let mut manager = PinPositionManager::default();

        // Add some cached positions
        manager
            .cached_positions
            .insert(PinId(1), Vec2::new(100.0, 200.0));
        manager
            .cached_positions
            .insert(PinId(2), Vec2::new(300.0, 400.0));

        assert_eq!(manager.cached_positions.len(), 2);
        assert_eq!(manager.frame_version, 0);

        manager.invalidate_cache();

        assert!(manager.cached_positions.is_empty());
        assert_eq!(manager.frame_version, 1);
    }

    #[test]
    fn test_get_pin_screen_position_cached() {
        let mut manager = PinPositionManager::default();
        let graph = create_test_node_graph();
        let canvas_state = create_test_canvas_state();

        // Pre-populate cache
        let expected_pos = Vec2::new(999.0, 888.0);
        manager.cached_positions.insert(PinId(1), expected_pos);

        let result = manager.get_pin_screen_position(PinId(1), &graph, &canvas_state);

        assert_eq!(result, Some(expected_pos));
    }

    #[test]
    fn test_get_pin_screen_position_calculated() {
        let mut manager = PinPositionManager::default();
        let graph = create_test_node_graph();
        let canvas_state = create_test_canvas_state();

        let result = manager.get_pin_screen_position(PinId(1), &graph, &canvas_state);

        assert!(result.is_some());
        // Position should be calculated based on node position, canvas offset, and pin offset
        let pos = result.unwrap();
        assert!(pos.x > 0.0);
        assert!(pos.y > 0.0);
    }

    #[test]
    fn test_get_pin_screen_position_nonexistent() {
        let mut manager = PinPositionManager::default();
        let graph = create_test_node_graph();
        let canvas_state = create_test_canvas_state();

        let result = manager.get_pin_screen_position(PinId(999), &graph, &canvas_state);

        assert_eq!(result, None);
    }

    #[test]
    fn test_get_pin_canvas_position() {
        let mut manager = PinPositionManager::default();
        let graph = create_test_node_graph();
        let mut canvas_state = create_test_canvas_state();
        canvas_state.zoom = 2.0;

        let screen_pos = manager.get_pin_screen_position(PinId(1), &graph, &canvas_state);
        let canvas_pos = manager.get_pin_canvas_position(PinId(1), &graph, &canvas_state);

        assert!(screen_pos.is_some());
        assert!(canvas_pos.is_some());

        // Canvas position should be screen position divided by zoom, minus offset
        let expected_canvas = screen_pos.unwrap() / canvas_state.zoom - canvas_state.offset;
        assert!((canvas_pos.unwrap().x - expected_canvas.x).abs() < 0.001);
        assert!((canvas_pos.unwrap().y - expected_canvas.y).abs() < 0.001);
    }

    #[test]
    fn test_get_connection_endpoints() {
        let mut manager = PinPositionManager::default();
        let graph = create_test_node_graph();
        let canvas_state = create_test_canvas_state();

        let result = manager.get_connection_endpoints(PinId(1), PinId(2), &graph, &canvas_state);

        assert!(result.is_some());
        let (from_pos, to_pos) = result.unwrap();
        assert!(from_pos.x > 0.0);
        assert!(from_pos.y > 0.0);
        assert!(to_pos.x > 0.0);
        assert!(to_pos.y > 0.0);
    }

    #[test]
    fn test_get_connection_endpoints_nonexistent() {
        let mut manager = PinPositionManager::default();
        let graph = create_test_node_graph();
        let canvas_state = create_test_canvas_state();

        let result =
            manager.get_connection_endpoints(PinId(999), PinId(1000), &graph, &canvas_state);

        assert_eq!(result, None);
    }

    #[test]
    fn test_pin_position_calculation_input_pin() {
        let manager = PinPositionManager::default();
        let graph = create_test_node_graph();
        let canvas_state = create_test_canvas_state();

        // Calculate input pin position (PinId 0 - first input pin)
        let position = manager.calculate_pin_position_raw(PinId(0), &graph, &canvas_state);

        assert!(position.is_some());
        let pos = position.unwrap();

        // Should be on the left side of the node
        let expected_x = (100.0 + 50.0) * 1.0 - 6.0; // node.x + offset.x - margin
        assert!((pos.x - expected_x).abs() < 0.1);

        // Should be below header with spacing
        let expected_y = (200.0 + 25.0) * 1.0 + 24.0 + 20.0 + 0.0 * 20.0; // node.y + offset.y + header + spacing + index*spacing
        assert!((pos.y - expected_y).abs() < 0.1);
    }

    #[test]
    fn test_pin_position_calculation_output_pin() {
        let manager = PinPositionManager::default();
        let graph = create_test_node_graph();
        let canvas_state = create_test_canvas_state();

        // Calculate output pin position (PinId 1 - first output pin)
        let position = manager.calculate_pin_position_raw(PinId(1), &graph, &canvas_state);

        assert!(position.is_some());
        let pos = position.unwrap();

        // Should be on the right side of the node
        let expected_x = (100.0 + 50.0) * 1.0 + 220.0 + 6.0; // node.x + offset.x + width + margin
        assert!((pos.x - expected_x).abs() < 0.1);

        // Should be below header with spacing
        let expected_y = (200.0 + 25.0) * 1.0 + 24.0 + 20.0 + 0.0 * 20.0; // node.y + offset.y + header + spacing + index*spacing
        assert!((pos.y - expected_y).abs() < 0.1);
    }

    #[test]
    fn test_pin_position_calculation_with_zoom() {
        let manager = PinPositionManager::default();
        let graph = create_test_node_graph();
        let mut canvas_state = create_test_canvas_state();
        canvas_state.zoom = 2.0;

        let position = manager.calculate_pin_position_raw(PinId(0), &graph, &canvas_state);

        assert!(position.is_some());
        let pos = position.unwrap();

        // Position should be scaled by zoom
        let expected_x = ((100.0 + 50.0) * 2.0) - 6.0; // (node.x + offset.x) * zoom - margin
        assert!((pos.x - expected_x).abs() < 0.1);
    }

    #[test]
    fn test_multiple_pin_positions() {
        let mut manager = PinPositionManager::default();
        let graph = create_test_node_graph();
        let canvas_state = create_test_canvas_state();

        // Get positions for all pins
        let pos0 = manager.get_pin_screen_position(PinId(0), &graph, &canvas_state);
        let pos1 = manager.get_pin_screen_position(PinId(1), &graph, &canvas_state);
        let pos2 = manager.get_pin_screen_position(PinId(2), &graph, &canvas_state);
        let pos3 = manager.get_pin_screen_position(PinId(3), &graph, &canvas_state);

        assert!(pos0.is_some());
        assert!(pos1.is_some());
        assert!(pos2.is_some());
        assert!(pos3.is_some());

        // Input pins should be on the left, output pins on the right
        assert!(pos0.unwrap().x < pos1.unwrap().x);
        assert!(pos2.unwrap().x < pos3.unwrap().x);

        // Pins should be vertically spaced
        assert!(pos0.unwrap().y < pos2.unwrap().y); // First input above second input
        assert!(pos1.unwrap().y < pos3.unwrap().y); // First output above second output
    }

    #[test]
    fn test_frame_version_increment() {
        let mut manager = PinPositionManager::default();

        assert_eq!(manager.frame_version, 0);

        manager.invalidate_cache();
        assert_eq!(manager.frame_version, 1);

        manager.invalidate_cache();
        assert_eq!(manager.frame_version, 2);
    }

    #[test]
    fn test_cache_hit_miss() {
        let mut manager = PinPositionManager::default();
        let graph = create_test_node_graph();
        let canvas_state = create_test_canvas_state();

        // First call should be a cache miss and calculate position
        let pos1 = manager.get_pin_screen_position(PinId(1), &graph, &canvas_state);
        assert!(pos1.is_some());

        // Second call should be a cache hit
        let pos2 = manager.get_pin_screen_position(PinId(1), &graph, &canvas_state);
        assert!(pos2.is_some());

        assert_eq!(pos1, pos2);
        assert!(manager.cached_positions.contains_key(&PinId(1)));
    }
}
