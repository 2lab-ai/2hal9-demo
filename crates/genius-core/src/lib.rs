//! Core types and traits for the Genius Game Platform
//! 
//! This crate provides the fundamental abstractions used throughout the system.

pub mod game;
pub mod player;
pub mod state;
pub mod error;

pub use game::*;
pub use player::*;
pub use state::*;
pub use error::*;

/// Re-export commonly used types
pub mod prelude {
    pub use crate::{
        game::{Game, GameConfig, GameType},
        player::{Player, PlayerId, PlayerAction},
        state::{GameState, RoundResult, GameResult},
        error::{GameError, Result},
    };
}