//! Ollama AI provider implementation

use crate::provider::{AIProvider, AIDecision, ProviderCapabilities};
use async_trait::async_trait;
use genius_core::{PlayerAction, GameState, Result};
use serde::{Deserialize, Serialize};

pub struct OllamaProvider {
    model_name: String,
    endpoint: String,
    client: reqwest::Client,
}

impl OllamaProvider {
    pub fn new(model_name: String) -> Self {
        Self {
            model_name,
            endpoint: "http://localhost:11434".to_string(),
            client: reqwest::Client::new(),
        }
    }
    
    pub fn with_endpoint(model_name: String, endpoint: String) -> Self {
        Self {
            model_name,
            endpoint,
            client: reqwest::Client::new(),
        }
    }
}

#[derive(Serialize)]
struct OllamaRequest {
    model: String,
    prompt: String,
    stream: bool,
    options: OllamaOptions,
}

#[derive(Serialize)]
struct OllamaOptions {
    temperature: f32,
    num_predict: u32,
}

#[derive(Deserialize)]
struct OllamaResponse {
    response: String,
}

#[async_trait]
impl AIProvider for OllamaProvider {
    fn name(&self) -> &str {
        "Ollama"
    }
    
    async fn make_decision(
        &self,
        game_state: &GameState,
        player_id: &str,
        valid_actions: Vec<String>,
    ) -> Result<AIDecision> {
        // Build prompt based on game state
        let prompt = format!(
            "You are playing {} as player {}.\n\
             Current round: {}\n\
             Your score: {:?}\n\
             Valid actions: {:?}\n\n\
             What is your next action? Choose from the valid actions and explain briefly.\n\
             Respond in this format:\n\
             Action: <your chosen action>\n\
             Reasoning: <brief explanation>",
            game_state.game_type.display_name(),
            player_id,
            game_state.round,
            game_state.scores.get(player_id).unwrap_or(&0.0),
            valid_actions
        );
        
        // Create Ollama request
        let request = OllamaRequest {
            model: self.model_name.clone(),
            prompt,
            stream: false,
            options: OllamaOptions {
                temperature: 0.7,
                num_predict: 100,
            },
        };
        
        // Send request to Ollama
        let url = format!("{}/api/generate", self.endpoint);
        match self.client.post(&url).json(&request).send().await {
            Ok(response) => {
                if response.status().is_success() {
                    match response.json::<OllamaResponse>().await {
                        Ok(ollama_resp) => {
                            // Parse response
                            let response_text = ollama_resp.response.to_lowercase();
                            
                            // Find action in response
                            let mut chosen_action = valid_actions[0].clone();
                            for action in &valid_actions {
                                if response_text.contains(&action.to_lowercase()) {
                                    chosen_action = action.clone();
                                    break;
                                }
                            }
                            
                            // Extract reasoning
                            let reasoning = if let Some(idx) = response_text.find("reasoning:") {
                                response_text[idx + 10..].trim().to_string()
                            } else {
                                "Ollama AI decision".to_string()
                            };
                            
                            let action = PlayerAction::new(
                                player_id.to_string(),
                                chosen_action,
                                serde_json::json!({"llm_response": ollama_resp.response}),
                            );
                            
                            Ok(AIDecision {
                                action,
                                reasoning,
                                confidence: 0.8,
                            })
                        }
                        Err(e) => {
                            // Fallback on parse error
                            let action = PlayerAction::new(
                                player_id.to_string(),
                                valid_actions[0].clone(),
                                serde_json::json!({"error": e.to_string()}),
                            );
                            
                            Ok(AIDecision {
                                action,
                                reasoning: "Parse error, using default action".to_string(),
                                confidence: 0.3,
                            })
                        }
                    }
                } else {
                    // Fallback on HTTP error
                    let action = PlayerAction::new(
                        player_id.to_string(),
                        valid_actions[0].clone(),
                        serde_json::json!({"error": "HTTP error"}),
                    );
                    
                    Ok(AIDecision {
                        action,
                        reasoning: format!("Ollama API error: {}", response.status()),
                        confidence: 0.1,
                    })
                }
            }
            Err(e) => {
                // Fallback on connection error
                let action = PlayerAction::new(
                    player_id.to_string(),
                    valid_actions[0].clone(),
                    serde_json::json!({"error": e.to_string()}),
                );
                
                Ok(AIDecision {
                    action,
                    reasoning: "Ollama connection failed, using default action".to_string(),
                    confidence: 0.1,
                })
            }
        }
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