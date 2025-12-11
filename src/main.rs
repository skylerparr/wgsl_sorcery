use bevy::prelude::*;

// Setup function - spawn the duck sprite with velocity component
fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    // Spawn camera
    commands.spawn(Camera2d);

    // Spawn duck sprite with velocity component for movement
    commands.spawn(
        (Sprite {
            image: asset_server.load("ducky.png"),
            ..Default::default()
        }),
    );
}

// Main application with integrated input system
fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        // Systems
        .add_systems(Startup, (setup))
        .run();
}
