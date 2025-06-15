//! Mirror Mind - A game of recursive prediction and theory of mind

use genius_core::{
    Game, GameConfig, GameState, GameResult, PlayerAction, RoundResult,
    RoundOutcome, GameEvent, GameAnalytics, EmergenceEvent, EmergenceType,
    Result, GameError,
};
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// The depth of recursive thinking
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ThinkingLevel {
    Level0, // Direct action
    Level1, // I think you will...
    Level2, // I think you think I will...
    Level3, // I think you think I think you will...
    Level4, // And deeper...
}

/// A prediction about another player's action
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Prediction {
    target_player: String,
    predicted_action: String,
    thinking_level: ThinkingLevel,
    confidence: f32,
}

/// Player's mental model of others
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MentalModel {
    predictions: HashMap<String, Vec<Prediction>>,
    accuracy_history: Vec<f32>,
    model_complexity: f32,
}

pub struct MirrorMindGame {
    round_number: u32,
    player_models: HashMap<String, MentalModel>,
    prediction_matrix: HashMap<(String, String), Vec<Prediction>>,
    emergence_threshold: f32,
    max_thinking_depth: u32,
}

impl MirrorMindGame {
    pub fn new() -> Self {
        Self {
            round_number: 0,
            player_models: HashMap::new(),
            prediction_matrix: HashMap::new(),
            emergence_threshold: 0.75,
            max_thinking_depth: 4,
        }
    }
    
    fn calculate_prediction_accuracy(
        &self,
        prediction: &Prediction,
        actual_actions: &HashMap<String, PlayerAction>
    ) -> f32 {
        if let Some(actual) = actual_actions.get(&prediction.target_player) {
            if actual.action_type == prediction.predicted_action {
                // Higher reward for deeper accurate predictions
                1.0 + (prediction.thinking_level as u32 as f32 * 0.2)
            } else {
                // Penalty for wrong predictions at deep levels
                -(prediction.thinking_level as u32 as f32 * 0.1)
            }
        } else {
            0.0
        }
    }
    
    fn detect_recursive_thinking(&self, actions: &HashMap<String, PlayerAction>) -> Option<EmergenceEvent> {
        // Analyze depth of recursive thinking across players
        let mut thinking_depths = Vec::new();
        
        for action in actions.values() {
            if let Ok(data) = serde_json::from_value::<MirrorMindAction>(action.data.clone()) {
                thinking_depths.push(data.predictions.len() as f32);
            }
        }
        
        if thinking_depths.is_empty() {
            return None;
        }
        
        let avg_depth = thinking_depths.iter().sum::<f32>() / thinking_depths.len() as f32;
        
        if avg_depth > 3.0 {
            Some(EmergenceEvent {
                round: self.round_number,
                event_type: EmergenceType::Custom("RecursiveThinking".to_string()),
                description: format!("Players achieving average thinking depth of {:.1} levels", avg_depth),
                emergence_score: (avg_depth / self.max_thinking_depth as f32).min(1.0),
                involved_players: actions.keys().cloned().collect(),
            })
        } else {
            None
        }
    }
    
    fn calculate_mental_model_convergence(&self) -> f32 {
        // Check if players' mental models are converging
        let models: Vec<_> = self.player_models.values().collect();
        if models.len() < 2 {
            return 0.0;
        }
        
        let mut similarity_sum = 0.0;
        let mut comparison_count = 0;
        
        for i in 0..models.len() {
            for j in i+1..models.len() {
                similarity_sum += self.compare_mental_models(models[i], models[j]);
                comparison_count += 1;
            }
        }
        
        if comparison_count > 0 {
            similarity_sum / comparison_count as f32
        } else {
            0.0
        }
    }
    
    fn compare_mental_models(&self, model1: &MentalModel, model2: &MentalModel) -> f32 {
        // Simplified comparison - real implementation would be more sophisticated
        let complexity_diff = (model1.model_complexity - model2.model_complexity).abs();
        1.0 - (complexity_diff / 10.0).min(1.0)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct MirrorMindAction {
    base_action: String,
    predictions: Vec<Prediction>,
    reasoning_chain: Vec<String>,
}

impl Default for MirrorMindGame {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl Game for MirrorMindGame {
    async fn initialize(&mut self, config: GameConfig) -> Result<GameState> {
        let mut state = GameState::new(config.game_type.clone());
        
        // Initialize with metadata about the game
        state.metadata.insert(
            "description".to_string(),
            serde_json::json!("Predict what others predict about your predictions"),
        );
        state.metadata.insert(
            "thinking_levels".to_string(),
            serde_json::json!(self.max_thinking_depth),
        );
        
        Ok(state)
    }
    
    async fn process_round(
        &mut self,
        state: &GameState,
        actions: HashMap<String, PlayerAction>
    ) -> Result<RoundResult> {
        self.round_number = state.round;
        let mut scores_delta = HashMap::new();
        let mut events = Vec::new();
        
        // Parse actions and predictions
        let mut parsed_actions = HashMap::new();
        for (player_id, action) in &actions {
            if let Ok(mirror_action) = serde_json::from_value::<MirrorMindAction>(action.data.clone()) {
                parsed_actions.insert(player_id.clone(), mirror_action);
            }
        }
        
        // Calculate accuracy of predictions
        for (player_id, mirror_action) in &parsed_actions {
            let mut round_score = 0.0;
            
            // Score each prediction
            for prediction in &mirror_action.predictions {
                let accuracy = self.calculate_prediction_accuracy(prediction, &actions);
                round_score += accuracy;
                
                // Update mental model
                if let Some(model) = self.player_models.get_mut(player_id) {
                    model.accuracy_history.push(accuracy.max(0.0));
                    model.model_complexity = mirror_action.predictions.len() as f32;
                }
            }
            
            // Bonus for theory of mind depth
            let depth_bonus = mirror_action.reasoning_chain.len() as f32 * 0.5;
            round_score += depth_bonus;
            
            scores_delta.insert(player_id.clone(), round_score as i32);
        }
        
        // Check for emergence
        if let Some(emergence) = self.detect_recursive_thinking(&actions) {
            events.push(GameEvent {
                event_type: "emergence".to_string(),
                description: emergence.description.clone(),
                affected_players: emergence.involved_players.clone(),
                data: serde_json::json!({ "emergence_score": emergence.emergence_score }),
            });
        }
        
        // Check for mental model convergence
        let convergence = self.calculate_mental_model_convergence();
        if convergence > self.emergence_threshold {
            events.push(GameEvent {
                event_type: "convergence".to_string(),
                description: format!("Mental models converging at {:.0}% similarity", convergence * 100.0),
                affected_players: state.players().into_iter().cloned().collect(),
                data: serde_json::json!({ "convergence_score": convergence }),
            });
        }
        
        // Determine round outcome
        let winners = scores_delta.iter()
            .filter(|(_, &score)| score > 0)
            .map(|(player, _)| player.clone())
            .collect();
            
        let outcome = RoundOutcome {
            winners,
            losers: vec![],
            special_events: events.iter().map(|e| e.description.clone()).collect(),
            emergence_detected: convergence > self.emergence_threshold,
        };
        
        Ok(RoundResult {
            round: state.round,
            actions: actions.clone(),
            outcome,
            scores_delta,
            events,
            timestamp: chrono::Utc::now(),
        })
    }
    
    async fn is_game_over(&self, state: &GameState) -> bool {
        // Game ends when maximum rounds reached or perfect convergence
        state.round >= 50 || self.calculate_mental_model_convergence() > 0.95
    }
    
    async fn calculate_final_result(&self, state: &GameState) -> GameResult {
        let winner = state.scores.iter()
            .max_by_key(|(_, &score)| score)
            .map(|(player, _)| player.clone())
            .unwrap_or_default();
            
        let convergence = self.calculate_mental_model_convergence();
        let avg_thinking_depth = self.player_models.values()
            .map(|m| m.model_complexity)
            .sum::<f32>() / self.player_models.len().max(1) as f32;
            
        GameResult {
            game_id: state.game_id,
            winner,
            final_scores: state.scores.clone(),
            total_rounds: state.round,
            duration_ms: 0, // Would be calculated from timestamps
            emergence_events: vec![],
            analytics: GameAnalytics {
                collective_coordination_score: convergence,
                decision_diversity_index: 1.0 - convergence,
                strategic_depth: avg_thinking_depth / self.max_thinking_depth as f32,
                emergence_frequency: 0.0, // Would be calculated from events
                performance_differential: 0.0, // Would be calculated from scores
                custom_metrics: {
                    let mut metrics = HashMap::new();
                    metrics.insert("average_thinking_depth".to_string(), avg_thinking_depth);
                    metrics.insert("mental_model_convergence".to_string(), convergence);
                    metrics
                },
            },
        }
    }
    
    async fn get_valid_actions(&self, _state: &GameState, _player_id: &str) -> Vec<String> {
        vec![
            "predict_action".to_string(),
            "predict_prediction".to_string(),
            "model_theory_of_mind".to_string(),
        ]
    }
    
    async fn get_visualization_data(&self, _state: &GameState) -> serde_json::Value {
        serde_json::json!({
            "prediction_matrix": self.prediction_matrix,
            "mental_models": self.player_models,
            "convergence": self.calculate_mental_model_convergence(),
        })
    }
}