//! AI provider traits and abstractions

use async_trait::async_trait;
use genius_core::{PlayerAction, GameState, Result};
use serde::{Deserialize, Serialize};

/// Decision made by an AI provider
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AIDecision {
    pub action: PlayerAction,
    pub reasoning: String,
    pub confidence: f32,
}

/// Trait for AI providers
#[async_trait]
pub trait AIProvider: Send + Sync {
    /// Get the provider name
    fn name(&self) -> &str;
    
    /// Make a decision based on game state
    async fn make_decision(
        &self,
        game_state: &GameState,
        player_id: &str,
        valid_actions: Vec<String>,
    ) -> Result<AIDecision>;
    
    /// Get provider capabilities
    fn capabilities(&self) -> ProviderCapabilities {
        ProviderCapabilities::default()
    }
}

/// Capabilities of an AI provider
#[derive(Debug, Clone, Default)]
pub struct ProviderCapabilities {
    pub supports_reasoning: bool,
    pub supports_confidence: bool,
    pub max_context_length: usize,
    pub supports_streaming: bool,
}
