use bevy::prelude::*;

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
