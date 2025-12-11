use bevy::prelude::*;
use std::collections::{HashMap, HashSet};

// Import the GameAction enum from actions.rs
use crate::plugins::input_system::GameAction;

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
