//! Mock AI provider for testing

use crate::provider::{AIProvider, AIDecision, ProviderCapabilities};
use async_trait::async_trait;
use genius_core::{PlayerAction, GameState, Result};
use rand::Rng;

pub struct MockProvider {
    deterministic: bool,
}

impl MockProvider {
    pub fn new() -> Self {
        Self {
            deterministic: false,
        }
    }
    
    pub fn deterministic() -> Self {
        Self {
            deterministic: true,
        }
    }
}

impl Default for MockProvider {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl AIProvider for MockProvider {
    fn name(&self) -> &str {
        "Mock Provider"
    }
    
    async fn make_decision(
        &self,
        _game_state: &GameState,
        player_id: &str,
        valid_actions: Vec<String>,
    ) -> Result<AIDecision> {
        let action_type = if self.deterministic {
            valid_actions.first().unwrap_or(&"pass".to_string()).clone()
        } else {
            let mut rng = rand::rng();
            let idx = rng.random_range(0..valid_actions.len());
            valid_actions[idx].clone()
        };
        
        let action = PlayerAction::new(
            player_id.to_string(),
            action_type,
            serde_json::json!({}),
        );
        
        Ok(AIDecision {
            action,
            reasoning: "Mock decision based on randomness".to_string(),
            confidence: 0.5,
        })
    }
    
    fn capabilities(&self) -> ProviderCapabilities {
        ProviderCapabilities {
            supports_reasoning: false,
            supports_confidence: false,
            max_context_length: 1024,
            supports_streaming: false,
        }
    }
}