use crate::node_graph::model::{NodeGraph, NodeId, PinId};
use bevy::prelude::*;
use bevy_egui::egui;

#[derive(Debug, Clone, Resource, Default)]
pub struct GraphUiState {
    pub pending_connection: Option<PendingConnection>,
    pub active_drag_node: Option<NodeId>,
    pub drag_origin: Vec2,
    pub drag_offset: Vec2,
}

#[derive(Debug, Clone)]
pub struct PendingConnection {
    pub from_pin: PinId,
    pub from_screen_pos: egui::Pos2,
}
