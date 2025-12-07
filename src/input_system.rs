use bevy::prelude::*;
use std::collections::{HashMap, HashSet};

/// Actions that can be triggered by inputs
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum GameAction {
    MoveLeft,
    MoveRight,
    MoveUp,
    MoveDown,
    RotateLeft,
    RotateRight,
    SpeedBoost,
}

/// Simple input source type - just key codes for now to keep it working
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum InputSource {
    KeyCode(KeyCode),
}

impl std::fmt::Display for InputSource {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::KeyCode(key_code) => write!(f, "Keyboard({:?})", key_code),
        }
    }
}

/// Input mapping configuration
#[derive(Resource)]
pub struct InputMappings {
    /// Maps actions to their input sources
    pub action_to_input: HashMap<GameAction, Vec<InputSource>>,
}

impl Default for InputMappings {
    fn default() -> Self {
        let mut mappings = HashMap::new();

        // Use key codes that actually exist in Bevy 0.17.3

        mappings.insert(
            GameAction::MoveLeft,
            vec![InputSource::KeyCode(KeyCode::ArrowLeft)],
        );

        mappings.insert(
            GameAction::MoveRight,
            vec![InputSource::KeyCode(KeyCode::ArrowRight)],
        );

        mappings.insert(
            GameAction::MoveUp,
            vec![InputSource::KeyCode(KeyCode::ArrowUp)],
        );

        mappings.insert(
            GameAction::MoveDown,
            vec![InputSource::KeyCode(KeyCode::ArrowDown)],
        );

        Self {
            action_to_input: mappings,
        }
    }
}

/// Input state tracking - what actions are currently active
#[derive(Resource, Default)]
pub struct InputState {
    pub active_actions: HashSet<GameAction>,
}

impl InputState {
    /// Check if an action is currently being performed
    pub fn is_action_active(&self, action: GameAction) -> bool {
        self.active_actions.contains(&action)
    }

    /// Get all actions that are currently active
    pub fn get_active_actions(&self) -> &HashSet<GameAction> {
        &self.active_actions
    }
}

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

/// Helper functions for adding/removing mappings
impl InputMappings {
    /// Add a new input mapping
    pub fn add_mapping(&mut self, action: GameAction, source: InputSource) {
        self.action_to_input
            .entry(action)
            .or_insert_with(Vec::new)
            .push(source);
    }

    /// Remove all existing mappings for an action and replace with new ones
    pub fn set_mappings_for_action(&mut self, action: GameAction, sources: Vec<InputSource>) {
        self.action_to_input.insert(action, sources);
    }

    /// Get all current mappings as a reference
    pub fn get_current_mappings(&self) -> &HashMap<GameAction, Vec<InputSource>> {
        &self.action_to_input
    }
}
