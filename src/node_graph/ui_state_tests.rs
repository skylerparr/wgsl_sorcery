#[cfg(test)]
mod tests {
    use crate::node_graph::model::{NodeId, PinId};
    use crate::node_graph::ui_state::{GraphUiState, PendingConnection};
    use bevy::prelude::*;
    use bevy_egui::egui;

    #[test]
    fn test_graph_ui_state_default() {
        let ui_state = GraphUiState::default();

        assert_eq!(ui_state.active_drag_node, None);
        assert_eq!(ui_state.pending_connection, None);
        assert_eq!(ui_state.drag_origin, Vec2::ZERO);
        assert_eq!(ui_state.drag_offset, Vec2::ZERO);
    }

    #[test]
    fn test_graph_ui_state_dragging() {
        let mut ui_state = GraphUiState::default();

        // Start dragging a node
        ui_state.active_drag_node = Some(NodeId(1));
        ui_state.drag_origin = Vec2::new(300.0, 400.0);
        ui_state.drag_offset = Vec2::new(50.0, 60.0);

        assert_eq!(ui_state.active_drag_node, Some(NodeId(1)));
        assert_eq!(ui_state.drag_origin, Vec2::new(300.0, 400.0));
        assert_eq!(ui_state.drag_offset, Vec2::new(50.0, 60.0));
    }

    #[test]
    fn test_pending_connection_creation() {
        let pin_id = PinId(1);
        let from_pos = egui::pos2(100.0, 200.0);

        let pending = PendingConnection {
            from_pin: pin_id,
            from_screen_pos: from_pos,
        };

        assert_eq!(pending.from_pin, pin_id);
        assert_eq!(pending.from_screen_pos, from_pos);
    }

    #[test]
    fn test_pending_connection_clone() {
        let pin_id = PinId(1);
        let from_pos = egui::pos2(100.0, 200.0);

        let pending = PendingConnection {
            from_pin: pin_id,
            from_screen_pos: from_pos,
        };

        let cloned = pending.clone();
        assert_eq!(cloned.from_pin, pending.from_pin);
        assert_eq!(cloned.from_screen_pos, pending.from_screen_pos);
    }

    #[test]
    fn test_graph_ui_state_with_pending_connection() {
        let mut ui_state = GraphUiState::default();

        let pin_id = PinId(1);
        let from_pos = egui::pos2(100.0, 200.0);

        let pending = PendingConnection {
            from_pin: pin_id,
            from_screen_pos: from_pos,
        };

        ui_state.pending_connection = Some(pending.clone());

        assert_eq!(ui_state.active_drag_node, None);
        assert_eq!(ui_state.pending_connection, Some(pending));
        assert_eq!(ui_state.drag_origin, Vec2::ZERO);
        assert_eq!(ui_state.drag_offset, Vec2::ZERO);
    }

    #[test]
    fn test_clear_drag_state() {
        let mut ui_state = GraphUiState::default();

        // Set up a dragging state
        ui_state.active_drag_node = Some(NodeId(1));
        ui_state.drag_origin = Vec2::new(300.0, 400.0);
        ui_state.drag_offset = Vec2::new(50.0, 60.0);

        // Clear drag state
        ui_state.clear_drag_state();

        assert_eq!(ui_state.active_drag_node, None);
        assert_eq!(ui_state.drag_origin, Vec2::ZERO);
        assert_eq!(ui_state.drag_offset, Vec2::ZERO);
    }

    #[test]
    fn test_clear_pending_connection() {
        let mut ui_state = GraphUiState::default();

        // Set up a pending connection
        let pin_id = PinId(1);
        let from_pos = egui::pos2(100.0, 200.0);

        let pending = PendingConnection {
            from_pin: pin_id,
            from_screen_pos: from_pos,
        };

        ui_state.pending_connection = Some(pending);

        // Clear pending connection
        ui_state.clear_pending_connection();

        assert_eq!(ui_state.active_drag_node, None);
        assert_eq!(ui_state.pending_connection, None);
        assert_eq!(ui_state.drag_origin, Vec2::ZERO);
        assert_eq!(ui_state.drag_offset, Vec2::ZERO);
    }
}
