use bevy::prelude::*;
use std::collections::HashSet;

// Import the required types
use crate::plugins::input_system::{InputState, InputMappings, InputSource, GameAction};

/// System to update input state based on keyboard inputs  
fn update_input_system(
    mut input_state: ResMut<InputState>,
    mappings: Res<InputMappings>,
    keyboard_input: Res<ButtonInput<KeyCode>>, // Use KeyCode as the generic parameter
) {
    let mut new_active_actions = HashSet::new();

    // Check each action against its mapped inputs
    for (action, sources) in &mappings.action_to_input {
        for source in sources {
            match source {
                InputSource::KeyCode(key_code) => {
                    if keyboard_input.pressed(*key_code) {
                        new_active_actions.insert(*action);
                        break;
                    }
                }
            }
        }
    }

    input_state.active_actions = new_active_actions;
}

/// Plugin for the input system
pub struct InputSystemPlugin;

impl Plugin for InputSystemPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, update_input_system);

        // Initialize resources
        if !app.world_mut().contains_resource::<InputMappings>() {
            app.insert_resource(InputMappings::default());
        }
        if !app.world_mut().contains_resource::<InputState>() {
            app.insert_resource(InputState::default());
        }
    }
}
