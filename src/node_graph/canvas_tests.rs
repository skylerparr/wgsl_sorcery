#[cfg(test)]
mod tests {
    use crate::node_graph::canvas::{canvas_to_screen, screen_to_canvas, update_canvas_system};
    use crate::node_graph::model::{CanvasState, NodeGraph};
    use bevy::input::mouse::{MouseMotion, MouseWheel};
    use bevy::prelude::*;

    fn create_test_app() -> App {
        let mut app = App::new();
        app.add_plugins((MinimalPlugins, bevy::input::InputPlugin));
        app.insert_resource(NodeGraph::default());
        app.add_systems(Update, update_canvas_system);
        app
    }

    #[test]
    fn test_screen_to_canvas() {
        let canvas_state = CanvasState {
            zoom: 1.0,
            offset: Vec2::new(100.0, 50.0),
        };

        let screen_pos = Vec2::new(200.0, 150.0);
        let canvas_pos = screen_to_canvas(screen_pos, &canvas_state);

        // Expected: (200/1.0) + 100 = 300, (150/1.0) + 50 = 200
        assert_eq!(canvas_pos, Vec2::new(300.0, 200.0));
    }

    #[test]
    fn test_screen_to_canvas_with_zoom() {
        let canvas_state = CanvasState {
            zoom: 2.0,
            offset: Vec2::new(100.0, 50.0),
        };

        let screen_pos = Vec2::new(400.0, 300.0);
        let canvas_pos = screen_to_canvas(screen_pos, &canvas_state);

        // Expected: (400/2.0) + 100 = 300, (300/2.0) + 50 = 200
        assert_eq!(canvas_pos, Vec2::new(300.0, 200.0));
    }

    #[test]
    fn test_canvas_to_screen() {
        let canvas_state = CanvasState {
            zoom: 1.0,
            offset: Vec2::new(100.0, 50.0),
        };

        let canvas_pos = Vec2::new(300.0, 200.0);
        let screen_pos = canvas_to_screen(canvas_pos, &canvas_state);

        // Expected: (300 - 100) * 1.0 = 200, (200 - 50) * 1.0 = 150
        assert_eq!(screen_pos, Vec2::new(200.0, 150.0));
    }

    #[test]
    fn test_canvas_to_screen_with_zoom() {
        let canvas_state = CanvasState {
            zoom: 2.0,
            offset: Vec2::new(100.0, 50.0),
        };

        let canvas_pos = Vec2::new(300.0, 200.0);
        let screen_pos = canvas_to_screen(canvas_pos, &canvas_state);

        // Expected: (300 - 100) * 2.0 = 400, (200 - 50) * 2.0 = 300
        assert_eq!(screen_pos, Vec2::new(400.0, 300.0));
    }

    #[test]
    fn test_coordinate_transform_roundtrip() {
        let canvas_state = CanvasState {
            zoom: 1.5,
            offset: Vec2::new(75.0, 125.0),
        };

        let original_canvas = Vec2::new(250.0, 350.0);
        let screen = canvas_to_screen(original_canvas, &canvas_state);
        let back_to_canvas = screen_to_canvas(screen, &canvas_state);

        assert!((original_canvas.x - back_to_canvas.x).abs() < 0.001);
        assert!((original_canvas.y - back_to_canvas.y).abs() < 0.001);
    }

    #[test]
    fn test_coordinate_transform_roundtrip_with_zoom() {
        let canvas_state = CanvasState {
            zoom: 3.0,
            offset: Vec2::new(50.0, 25.0),
        };

        let original_canvas = Vec2::new(100.0, 200.0);
        let screen = canvas_to_screen(original_canvas, &canvas_state);
        let back_to_canvas = screen_to_canvas(screen, &canvas_state);

        assert!((original_canvas.x - back_to_canvas.x).abs() < 0.001);
        assert!((original_canvas.y - back_to_canvas.y).abs() < 0.001);
    }

    #[test]
    fn test_screen_to_canvas_zero_offset() {
        let canvas_state = CanvasState {
            zoom: 1.0,
            offset: Vec2::ZERO,
        };

        let screen_pos = Vec2::new(150.0, 250.0);
        let canvas_pos = screen_to_canvas(screen_pos, &canvas_state);

        assert_eq!(canvas_pos, screen_pos);
    }

    #[test]
    fn test_canvas_to_screen_zero_offset() {
        let canvas_state = CanvasState {
            zoom: 1.0,
            offset: Vec2::ZERO,
        };

        let canvas_pos = Vec2::new(150.0, 250.0);
        let screen_pos = canvas_to_screen(canvas_pos, &canvas_state);

        assert_eq!(screen_pos, screen_pos);
    }

    #[test]
    fn test_screen_to_canvas_zero_zoom() {
        let canvas_state = CanvasState {
            zoom: 0.0, // This shouldn't happen in practice but test edge case
            offset: Vec2::new(100.0, 50.0),
        };

        let screen_pos = Vec2::new(200.0, 150.0);
        let canvas_pos = screen_to_canvas(screen_pos, &canvas_state);

        // Division by zero should give infinity, but we expect the offset
        assert!(canvas_pos.x.is_infinite());
        assert!(canvas_pos.y.is_infinite());
    }

    #[test]
    fn test_negative_coordinates() {
        let canvas_state = CanvasState {
            zoom: 1.0,
            offset: Vec2::new(-50.0, -25.0),
        };

        let screen_pos = Vec2::new(-100.0, -150.0);
        let canvas_pos = screen_to_canvas(screen_pos, &canvas_state);

        assert_eq!(canvas_pos, Vec2::new(-150.0, -175.0));

        let back_to_screen = canvas_to_screen(canvas_pos, &canvas_state);
        assert_eq!(back_to_screen, screen_pos);
    }

    #[test]
    fn test_large_coordinates() {
        let canvas_state = CanvasState {
            zoom: 0.5,
            offset: Vec2::new(1000.0, 2000.0),
        };

        let screen_pos = Vec2::new(5000.0, 8000.0);
        let canvas_pos = screen_to_canvas(screen_pos, &canvas_state);

        // Expected: (5000/0.5) + 1000 = 11000, (8000/0.5) + 2000 = 18000
        assert_eq!(canvas_pos, Vec2::new(11000.0, 18000.0));
    }

    #[test]
    fn test_update_canvas_system_no_input() {
        let mut app = create_test_app();

        // Run the system without any input events
        app.update();

        // Check that canvas state remains unchanged
        let node_graph = app.world().resource::<NodeGraph>();
        assert_eq!(node_graph.canvas_state.zoom, 1.0);
        assert_eq!(node_graph.canvas_state.offset, Vec2::ZERO);
    }

    #[test]
    fn test_update_canvas_system_zoom_in() {
        let mut app = create_test_app();

        // Send a mouse wheel event (positive Y = zoom in)
        app.world_mut().write_message(MouseWheel {
            y: 1.0,
            x: 0.0,
            unit: bevy::input::mouse::MouseScrollUnit::Line,
            window: Entity::PLACEHOLDER,
        });

        app.update();

        let node_graph = app.world().resource::<NodeGraph>();
        assert!(node_graph.canvas_state.zoom > 1.0);
        assert!(node_graph.canvas_state.zoom <= 4.0); // Should be clamped
    }

    #[test]
    fn test_update_canvas_system_zoom_out() {
        let mut app = create_test_app();

        // Send a mouse wheel event (negative Y = zoom out)
        app.world_mut().write_message(MouseWheel {
            y: -1.0,
            x: 0.0,
            unit: bevy::input::mouse::MouseScrollUnit::Line,
            window: Entity::PLACEHOLDER,
        });

        app.update();

        let node_graph = app.world().resource::<NodeGraph>();
        assert!(node_graph.canvas_state.zoom < 1.0);
        assert!(node_graph.canvas_state.zoom >= 0.1); // Should be clamped
    }

    #[test]
    fn test_update_canvas_system_zoom_clamping() {
        let mut app = create_test_app();

        // Try to zoom way in (beyond max)
        app.world_mut().write_message(MouseWheel {
            y: 100.0,
            x: 0.0,
            unit: bevy::input::mouse::MouseScrollUnit::Line,
            window: Entity::PLACEHOLDER,
        });
        app.update();

        let node_graph = app.world().resource::<NodeGraph>();
        assert_eq!(node_graph.canvas_state.zoom, 4.0); // Should be clamped to max

        // Try to zoom way out (beyond min)
        app.world_mut().write_message(MouseWheel {
            y: -100.0,
            x: 0.0,
            unit: bevy::input::mouse::MouseScrollUnit::Line,
            window: Entity::PLACEHOLDER,
        });
        app.update();

        let node_graph = app.world().resource::<NodeGraph>();
        assert_eq!(node_graph.canvas_state.zoom, 0.1); // Should be clamped to min
    }

    #[test]
    fn test_update_canvas_system_pan() {
        let mut app = create_test_app();

        // Press right mouse button
        let mut button_input = ButtonInput::<MouseButton>::default();
        button_input.press(MouseButton::Right);
        app.world_mut().insert_resource(button_input);

        // Send mouse motion event
        app.world_mut().write_message(MouseMotion {
            delta: Vec2::new(10.0, -5.0),
        });

        app.update();

        let node_graph = app.world().resource::<NodeGraph>();
        // Offset should be adjusted (negative because we move canvas opposite to mouse)
        assert!(node_graph.canvas_state.offset.x < 0.0);
        assert!(node_graph.canvas_state.offset.y > 0.0);
    }

    #[test]
    fn test_update_canvas_system_pan_with_middle_button() {
        let mut app = create_test_app();

        // Press middle mouse button
        let mut button_input = ButtonInput::<MouseButton>::default();
        button_input.press(MouseButton::Middle);
        app.world_mut().insert_resource(button_input);

        // Send mouse motion event
        app.world_mut().write_message(MouseMotion {
            delta: Vec2::new(20.0, 15.0),
        });

        app.update();

        let node_graph = app.world().resource::<NodeGraph>();
        // Offset should be adjusted
        assert!(node_graph.canvas_state.offset.x < 0.0);
        assert!(node_graph.canvas_state.offset.y < 0.0);
    }

    #[test]
    fn test_update_canvas_system_no_pan_without_button() {
        let mut app = create_test_app();

        // Don't press any mouse button
        let button_input = ButtonInput::<MouseButton>::default();
        app.world_mut().insert_resource(button_input);

        // Send mouse motion event
        app.world_mut().write_message(MouseMotion {
            delta: Vec2::new(10.0, 10.0),
        });

        app.update();

        let node_graph = app.world().resource::<NodeGraph>();
        // Offset should remain unchanged
        assert_eq!(node_graph.canvas_state.offset, Vec2::ZERO);
    }

    #[test]
    fn test_update_canvas_system_pan_with_zoom() {
        let mut app = create_test_app();

        // Set initial zoom
        {
            let mut node_graph = app.world_mut().resource_mut::<NodeGraph>();
            node_graph.canvas_state.zoom = 2.0;
        }

        // Press right mouse button
        let mut button_input = ButtonInput::<MouseButton>::default();
        button_input.press(MouseButton::Right);
        app.world_mut().insert_resource(button_input);

        // Send mouse motion event
        app.world_mut().write_message(MouseMotion {
            delta: Vec2::new(10.0, -10.0),
        });

        app.update();

        let node_graph = app.world().resource::<NodeGraph>();
        // With zoom=2.0, the pan should be half the mouse delta, and negative because we subtract
        assert!((node_graph.canvas_state.offset.x + 5.0).abs() < 0.001);
        assert!((node_graph.canvas_state.offset.y - 5.0).abs() < 0.001);
    }

    #[test]
    fn test_update_canvas_system_combined_operations() {
        let mut app = create_test_app();

        // Press right mouse button for panning
        let mut button_input = ButtonInput::<MouseButton>::default();
        button_input.press(MouseButton::Right);
        app.world_mut().insert_resource(button_input);

        // Send both pan and zoom events
        app.world_mut().write_message(MouseMotion {
            delta: Vec2::new(5.0, 5.0),
        });
        app.world_mut().write_message(MouseWheel {
            y: 0.5,
            x: 0.0,
            unit: bevy::input::mouse::MouseScrollUnit::Line,
            window: Entity::PLACEHOLDER,
        });

        app.update();

        let node_graph = app.world().resource::<NodeGraph>();
        // Both zoom and offset should be affected
        assert!(node_graph.canvas_state.zoom > 1.0);
        assert!(node_graph.canvas_state.offset.x < 0.0);
        assert!(node_graph.canvas_state.offset.y < 0.0);
    }

    #[test]
    fn test_coordinate_precision() {
        let canvas_state = CanvasState {
            zoom: 0.3333333,
            offset: Vec2::new(0.1234567, 0.7654321),
        };

        let original_canvas = Vec2::new(123.456789, 987.654321);
        let screen_pos = canvas_to_screen(original_canvas, &canvas_state);
        let back_to_canvas = screen_to_canvas(screen_pos, &canvas_state);

        // Allow for small floating point errors
        assert!((original_canvas.x - back_to_canvas.x).abs() < 0.0001);
        assert!((original_canvas.y - back_to_canvas.y).abs() < 0.0001);
    }
}
