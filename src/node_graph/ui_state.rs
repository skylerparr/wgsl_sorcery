use crate::node_graph::model::{NodeId, PinId};
use bevy::prelude::*;
use bevy_egui::egui;

#[derive(Debug, Clone, PartialEq)]
pub struct PendingConnection {
    pub from_pin: PinId,
    pub from_screen_pos: egui::Pos2,
}

#[derive(Debug, Clone, Resource, Default)]
pub struct GraphUiState {
    pub pending_connection: Option<PendingConnection>,
    pub active_drag_node: Option<NodeId>,
    pub drag_origin: Vec2,
    pub drag_offset: Vec2,
}

impl GraphUiState {
    pub fn clear_drag_state(&mut self) {
        self.active_drag_node = None;
        self.drag_origin = Vec2::ZERO;
        self.drag_offset = Vec2::ZERO;
    }

    pub fn clear_pending_connection(&mut self) {
        self.pending_connection = None;
    }
}
