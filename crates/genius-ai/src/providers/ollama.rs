//! Ollama AI provider implementation

use crate::provider::{AIProvider, AIDecision, ProviderCapabilities};
use async_trait::async_trait;
use genius_core::{PlayerAction, GameState, Result};

pub struct OllamaProvider {
    model_name: String,
    endpoint: String,
}

impl OllamaProvider {
    pub fn new(model_name: String) -> Self {
        Self {
            model_name,
            endpoint: "http://localhost:11434".to_string(),
        }
    }
}

#[async_trait]
impl AIProvider for OllamaProvider {
    fn name(&self) -> &str {
        "Ollama"
    }
    
    async fn make_decision(
        &self,
        _game_state: &GameState,
        player_id: &str,
        valid_actions: Vec<String>,
    ) -> Result<AIDecision> {
        // Placeholder implementation
        let action = PlayerAction::new(
            player_id.to_string(),
            valid_actions.first().unwrap_or(&"pass".to_string()).clone(),
            serde_json::json!({}),
        );
        
        Ok(AIDecision {
            action,
            reasoning: "Ollama reasoning placeholder".to_string(),
            confidence: 0.7,
        })
    }
    
    fn capabilities(&self) -> ProviderCapabilities {
        ProviderCapabilities {
            supports_reasoning: true,
            supports_confidence: true,
            max_context_length: 4096,
            supports_streaming: true,
        }
    }
}