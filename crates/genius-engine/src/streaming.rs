//! Game event streaming infrastructure

use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::{broadcast, RwLock};
use uuid::Uuid;

use genius_core::{GameState, GameResult, GameEvent};

/// Messages that can be streamed during game execution
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum StreamMessage {
    GameStarted {
        game_id: Uuid,
        players: Vec<String>,
    },
    GameStateUpdate {
        game_id: Uuid,
        state: GameState,
    },
    RoundUpdate {
        game_id: Uuid,
        round: u32,
        events: Vec<GameEvent>,
    },
    GameEnded {
        game_id: Uuid,
        result: GameResult,
    },
    AnalyticsUpdate {
        game_id: Uuid,
        metrics: RealTimeMetrics,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RealTimeMetrics {
    pub average_response_time_ms: f64,
    pub active_players: u32,
    pub emergence_score: f32,
    pub rounds_per_minute: f32,
}

/// Manages streaming of game events
pub struct GameEventStreamer {
    sender: broadcast::Sender<StreamMessage>,
    metrics: Arc<RwLock<RealTimeMetrics>>,
}

impl GameEventStreamer {
    pub fn new() -> Self {
        let (sender, _) = broadcast::channel(100);
        Self {
            sender,
            metrics: Arc::new(RwLock::new(RealTimeMetrics {
                average_response_time_ms: 0.0,
                active_players: 0,
                emergence_score: 0.0,
                rounds_per_minute: 0.0,
            })),
        }
    }
    
    pub fn subscribe(&self) -> broadcast::Receiver<StreamMessage> {
        self.sender.subscribe()
    }
    
    pub async fn broadcast_game_started(&self, game_id: Uuid, players: Vec<String>) {
        let _ = self.sender.send(StreamMessage::GameStarted { game_id, players });
    }
    
    pub async fn broadcast_state_update(&self, game_id: Uuid, state: GameState) {
        let _ = self.sender.send(StreamMessage::GameStateUpdate { game_id, state });
    }
    
    pub async fn broadcast_round_update(&self, game_id: Uuid, round: u32, events: Vec<GameEvent>) {
        let _ = self.sender.send(StreamMessage::RoundUpdate { game_id, round, events });
    }
    
    pub async fn broadcast_game_ended(&self, game_id: Uuid, result: GameResult) {
        let _ = self.sender.send(StreamMessage::GameEnded { game_id, result });
    }
    
    pub async fn update_metrics(&self, metrics: RealTimeMetrics) {
        *self.metrics.write().await = metrics.clone();
        // Broadcast metrics update
        let _ = self.sender.send(StreamMessage::AnalyticsUpdate {
            game_id: Uuid::new_v4(), // Would be game-specific in real implementation
            metrics,
        });
    }
}

impl Default for GameEventStreamer {
    fn default() -> Self {
        Self::new()
    }
}