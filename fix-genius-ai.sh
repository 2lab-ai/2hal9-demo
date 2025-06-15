#!/bin/bash

# Fix the provider.rs trait definition
cat > /Users/icedac/2lab.ai/2hal9-demo/crates/genius-ai/src/provider.rs << 'EOF'
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
EOF

# Fix collective.rs
sed -i '' 's/_game_state: game_state: &GameStateGameState/_game_state: \&GameState/g' /Users/icedac/2lab.ai/2hal9-demo/crates/genius-ai/src/collective.rs
sed -i '' 's/genius_core::GameError::InvalidAction { reason: "\(.*\)".to_string() }/genius_core::GameError::InvalidAction { reason: \1.to_string() }/g' /Users/icedac/2lab.ai/2hal9-demo/crates/genius-ai/src/collective.rs

# Fix sota.rs
sed -i '' 's/_game_state: game_state: &GameStateGameState/_game_state: \&GameState/g' /Users/icedac/2lab.ai/2hal9-demo/crates/genius-ai/src/sota.rs
sed -i '' 's/_player_id: player_id: &strstr/_player_id: \&str/g' /Users/icedac/2lab.ai/2hal9-demo/crates/genius-ai/src/sota.rs
sed -i '' 's/genius_core::GameError::InvalidAction("No valid actions".to_string())/genius_core::GameError::InvalidAction { reason: "No valid actions".to_string() }/g' /Users/icedac/2lab.ai/2hal9-demo/crates/genius-ai/src/sota.rs

# Fix providers
sed -i '' 's/_game_state: game_state: &GameStateGameState/_game_state: \&GameState/g' /Users/icedac/2lab.ai/2hal9-demo/crates/genius-ai/src/providers/*.rs
sed -i '' 's/_player_id: player_id: &strstr/player_id: \&str/g' /Users/icedac/2lab.ai/2hal9-demo/crates/genius-ai/src/providers/*.rs
sed -i '' 's/__game_state: game_state: &GameStateGameState/_game_state: \&GameState/g' /Users/icedac/2lab.ai/2hal9-demo/crates/genius-ai/src/providers/*.rs

echo "Fixed genius-ai files!"