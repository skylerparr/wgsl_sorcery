use crate::node_graph::model::NodeGraph;
use crate::node_graph::pin_manager::PinPositionManager;
use crate::node_graph::ui_state::GraphUiState;
use crate::shader_view::{
    ShaderView, apply_shader, hot_reload_shaders, render_shader_preview, setup_shader_view,
};
use bevy::prelude::*;
use bevy_egui::EguiPlugin;
use shadplay::plugin::ShadPlayPlugin;

mod node_graph;
mod shader_view;
mod systems;

// Setup function - spawn the duck sprite with velocity component
fn setup(mut commands: Commands) {
    // Spawn camera
    commands.spawn(Camera2d);
}

// Cache invalidation system - runs first each frame to ensure fresh pin positions
fn invalidate_pin_cache_system(mut pin_manager: ResMut<PinPositionManager>) {
    pin_manager.invalidate_cache();
}

// Main application with integrated input system
fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                resolution: (1920, 1080).into(),
                ..default()
            }),
            ..default()
        }))
        .add_plugins(EguiPlugin::default())
        // .add_plugins(ShadPlayPlugin)
        // Node graph resources
        .init_resource::<NodeGraph>()
        .init_resource::<GraphUiState>()
        .init_resource::<PinPositionManager>()
        // Shader view resources
        .init_resource::<ShaderView>()
        // Systems
        .add_systems(Startup, setup)
        .add_systems(Startup, setup_shader_view)
        // Canvas systems
        .add_systems(Update, node_graph::canvas::update_canvas_system)
        // Cache invalidation (run first)
        .add_systems(Update, invalidate_pin_cache_system)
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
        // Shader view systems
        .add_systems(Update, apply_shader)
        .add_systems(Update, hot_reload_shaders)
        .add_systems(Update, render_shader_preview)
        .run();
}
