//! Player-related types and abstractions

use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Unique identifier for a player
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct PlayerId(pub String);

impl PlayerId {
    /// Create a new random player ID
    pub fn new() -> Self {
        Self(Uuid::new_v4().to_string())
    }
    
    /// Create from an existing string
    pub fn from_string(id: String) -> Self {
        Self(id)
    }
}

impl Default for PlayerId {
    fn default() -> Self {
        Self::new()
    }
}

impl std::fmt::Display for PlayerId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// Represents a player in the game system
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Player {
    pub id: PlayerId,
    pub name: String,
    pub player_type: PlayerType,
    pub metadata: serde_json::Value,
}

/// Type of player (human or various AI types)
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum PlayerType {
    Human,
    AI {
        provider: String,
        model: String,
    },
    Collective {
        num_agents: usize,
        strategy: String,
    },
}

/// Action taken by a player
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlayerAction {
    pub player_id: String,
    pub action_type: String,
    pub data: serde_json::Value,
    pub reasoning: Option<String>,
    pub confidence: Option<f32>,
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

impl PlayerAction {
    /// Create a new player action
    pub fn new(player_id: String, action_type: String, data: serde_json::Value) -> Self {
        Self {
            player_id,
            action_type,
            data,
            reasoning: None,
            confidence: None,
            timestamp: chrono::Utc::now(),
        }
    }
    
    /// Add reasoning to the action
    pub fn with_reasoning(mut self, reasoning: String) -> Self {
        self.reasoning = Some(reasoning);
        self
    }
    
    /// Add confidence score to the action
    pub fn with_confidence(mut self, confidence: f32) -> Self {
        self.confidence = Some(confidence.clamp(0.0, 1.0));
        self
    }
}