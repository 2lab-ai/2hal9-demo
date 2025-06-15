//! Error types for the game platform

use thiserror::Error;

/// Main error type for game operations
#[derive(Error, Debug)]
pub enum GameError {
    #[error("Game not found: {id}")]
    GameNotFound { id: String },
    
    #[error("Player not found: {id}")]
    PlayerNotFound { id: String },
    
    #[error("Invalid action: {reason}")]
    InvalidAction { reason: String },
    
    #[error("Game already started")]
    GameAlreadyStarted,
    
    #[error("Game not started")]
    GameNotStarted,
    
    #[error("Game already ended")]
    GameAlreadyEnded,
    
    #[error("Maximum players reached: {max}")]
    MaxPlayersReached { max: usize },
    
    #[error("Minimum players not met: {required}")]
    MinPlayersNotMet { required: usize },
    
    #[error("Turn timeout: {player_id}")]
    TurnTimeout { player_id: String },
    
    #[error("Invalid game state: {reason}")]
    InvalidState { reason: String },
    
    #[error("Configuration error: {reason}")]
    ConfigError { reason: String },
    
    #[error("AI provider error: {0}")]
    AIProviderError(String),
    
    #[error("Serialization error: {0}")]
    SerializationError(#[from] serde_json::Error),
    
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
    
    #[error("Other error: {0}")]
    Other(#[from] anyhow::Error),
}

/// Result type alias using GameError
pub type Result<T> = std::result::Result<T, GameError>;