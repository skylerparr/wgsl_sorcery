use crate::node_graph::model::NodeGraph;
use crate::node_graph::pin_manager::PinPositionManager;
use crate::node_graph::ui_state::GraphUiState;
use bevy::prelude::*;

mod node_graph;
mod systems;

// Setup function - spawn the duck sprite with velocity component
fn setup(mut commands: Commands) {
    // Spawn camera
    commands.spawn(Camera2d);
}

// Main application with integrated input system
fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(bevy_egui::EguiPlugin::default())
        // Node graph resources
        .init_resource::<NodeGraph>()
        .init_resource::<GraphUiState>()
        .init_resource::<PinPositionManager>()
        // Systems
        .add_systems(Startup, setup)
        // Canvas systems
        .add_systems(Update, node_graph::canvas::update_canvas_system)
        .add_systems(Update, node_graph::render::render_canvas_background_system)
        // Node rendering systems
        .add_systems(Update, node_graph::render::render_nodes_system)
        .add_systems(Update, node_graph::render::render_connections_system)
        .add_systems(Update, node_graph::render::render_pending_connection_system)
        // Node interaction systems
        .add_systems(Update, node_graph::interactions::handle_node_drag_system)
        .add_systems(
            Update,
            node_graph::interactions::handle_pin_interactions_system,
        )
        // Node creation system
        .add_systems(Update, systems::spawn_node::spawn_test_node_system)
        .run();
}
