use crate::node_graph::model::NodeGraph;
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
        // Systems
        .add_systems(Startup, setup)
        // Canvas systems
        .add_systems(Update, node_graph::canvas::update_canvas_system)
        .add_systems(Update, node_graph::canvas::render_canvas_background_system)
        // Node rendering systems
        .add_systems(Update, node_graph::render::render_nodes_system)
        .add_systems(Update, node_graph::render::render_connections_system)
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
