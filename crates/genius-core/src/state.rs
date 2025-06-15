//! Game state and result types

use crate::game::GameType;
use crate::player::PlayerAction;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

/// Current state of a game
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameState {
    pub game_id: Uuid,
    pub game_type: GameType,
    pub round: u32,
    pub scores: HashMap<String, i32>,
    pub history: Vec<RoundResult>,
    pub metadata: HashMap<String, serde_json::Value>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

impl GameState {
    /// Create a new game state
    pub fn new(game_type: GameType) -> Self {
        let now = chrono::Utc::now();
        Self {
            game_id: Uuid::new_v4(),
            game_type,
            round: 0,
            scores: HashMap::new(),
            history: Vec::new(),
            metadata: HashMap::new(),
            created_at: now,
            updated_at: now,
        }
    }
    
    /// Get list of active players
    pub fn players(&self) -> Vec<&String> {
        self.scores.keys().collect()
    }
    
    /// Add a player to the game
    pub fn add_player(&mut self, player_id: String) {
        self.scores.insert(player_id, 0);
    }
    
    /// Update scores based on deltas
    pub fn apply_score_deltas(&mut self, deltas: &HashMap<String, i32>) {
        for (player, delta) in deltas {
            if let Some(score) = self.scores.get_mut(player) {
                *score += delta;
            }
        }
    }
}

/// Result of a single game round
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RoundResult {
    pub round: u32,
    pub actions: HashMap<String, PlayerAction>,
    pub outcome: RoundOutcome,
    pub scores_delta: HashMap<String, i32>,
    pub events: Vec<GameEvent>,
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

/// Outcome of a round
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RoundOutcome {
    pub winners: Vec<String>,
    pub losers: Vec<String>,
    pub special_events: Vec<String>,
    pub emergence_detected: bool,
}

/// Events that can occur during gameplay
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameEvent {
    pub event_type: String,
    pub description: String,
    pub affected_players: Vec<String>,
    pub data: serde_json::Value,
}

/// Final result of a completed game
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameResult {
    pub game_id: Uuid,
    pub winner: String,
    pub final_scores: HashMap<String, i32>,
    pub total_rounds: u32,
    pub duration_ms: u64,
    pub emergence_events: Vec<EmergenceEvent>,
    pub analytics: GameAnalytics,
}

/// Emergence event detected during gameplay
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmergenceEvent {
    pub round: u32,
    pub event_type: EmergenceType,
    pub description: String,
    pub emergence_score: f32,
    pub involved_players: Vec<String>,
}

/// Types of emergence that can be detected
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EmergenceType {
    CollectiveStrategy,
    SwarmIntelligence,
    NashEquilibrium,
    PhaseTransition,
    SpontaneousCoordination,
    Custom(String),
}

/// Analytics calculated for a completed game
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameAnalytics {
    pub collective_coordination_score: f32,
    pub decision_diversity_index: f32,
    pub strategic_depth: f32,
    pub emergence_frequency: f32,
    pub performance_differential: f32,
    pub custom_metrics: HashMap<String, f32>,
}

impl Default for GameAnalytics {
    fn default() -> Self {
        Self {
            collective_coordination_score: 0.0,
            decision_diversity_index: 0.0,
            strategic_depth: 0.0,
            emergence_frequency: 0.0,
            performance_differential: 0.0,
            custom_metrics: HashMap::new(),
        }
    }
}