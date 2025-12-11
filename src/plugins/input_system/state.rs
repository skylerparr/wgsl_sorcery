use bevy::prelude::*;
use std::collections::HashSet;

// Import the GameAction enum from actions.rs
use crate::plugins::input_system::GameAction;

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
