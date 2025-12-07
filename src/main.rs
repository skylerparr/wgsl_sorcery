use bevy::prelude::*;

// Import our custom input system
mod input_system;
use input_system::{GameAction, InputMappings, InputSource, InputSystemPlugin};

// Velocity component for movement
#[derive(Component)]
struct Velocity {
    x: f32,
    y: f32,
}

// Movement speed
const MOVE_SPEED: f32 = 200.0;

// Setup function - spawn the duck sprite with velocity component
fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    // Spawn camera
    commands.spawn(Camera2d);

    // Spawn duck sprite with velocity component for movement
    commands.spawn((
        Sprite {
            image: asset_server.load("ducky.png"),
            ..Default::default()
        },
        Velocity { x: 0.0, y: 0.0 },
    ));
}

// Movement system that responds to input actions
fn handle_movement(
    mut query: Query<(&mut Transform, &mut Velocity), With<Sprite>>,
    time: Res<Time>,
    input_state: Res<input_system::InputState>,
) {
    for (mut transform, mut velocity) in query.iter_mut() {
        // Reset velocity each frame
        velocity.x = 0.0;

        // Check input state and set velocities accordingly
        if input_state.is_action_active(GameAction::MoveLeft) {
            velocity.x -= MOVE_SPEED * time.delta_secs();
        }
        if input_state.is_action_active(GameAction::MoveRight) {
            velocity.x += MOVE_SPEED * time.delta_secs();
        }
        if input_state.is_action_active(GameAction::MoveUp) {
            transform.translation.y += MOVE_SPEED * 0.5 * time.delta_secs();
        }
        if input_state.is_action_active(GameAction::MoveDown) {
            transform.translation.y -= MOVE_SPEED * 0.5 * time.delta_secs();
        }

        // Apply velocity to position
        transform.translation.x += velocity.x;
    }
}

// Rotation system - now controlled by gamepad inputs
fn handle_rotation(
    mut query: Query<&mut Transform, With<Sprite>>,
    input_state: Res<input_system::InputState>,
) {
    for mut transform in query.iter_mut() {
        if input_state.is_action_active(GameAction::RotateLeft) {
            transform.rotation = Quat::from_rotation_z(transform.rotation.z - 0.1);
        }
        if input_state.is_action_active(GameAction::RotateRight) {
            transform.rotation = Quat::from_rotation_z(transform.rotation.z + 0.1);
        }
    }
}

// Speed boost system
fn handle_speed_boost(
    _query: Query<&mut Transform, With<Sprite>>,
    input_state: Res<input_system::InputState>,
) {
    // This could modify movement speed or rotation speed when active
    if input_state.is_action_active(GameAction::SpeedBoost) {
        // Example: make duck glow or change color (you'd need to add color components)
    }
}

// Function to demonstrate dynamic remapping of inputs - now only runs once at startup
fn remap_inputs(mut mappings: ResMut<InputMappings>) {
    println!("=== Current Input Mappings ===");
    for (action, sources) in mappings.get_current_mappings() {
        let sources_str = sources
            .iter()
            .map(|s| s.to_string())
            .collect::<Vec<_>>()
            .join(", ");
        println!("{:?}: {}", action, sources_str);
    }

    // Example: Add a new mapping - map Space key to SpeedBoost
    mappings.add_mapping(GameAction::SpeedBoost, InputSource::KeyCode(KeyCode::Space));

    println!("\nAdded Space key as additional SpeedBoost input!");
}

// Main application with integrated input system
fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        // Add our custom input system plugin
        .add_plugins(InputSystemPlugin)
        // Systems
        .add_systems(Startup, (setup, remap_inputs))
        .add_systems(Update, (handle_movement, handle_rotation))
        .run();
}
