//! Main client interface

use genius_core::{GameType, PlayerAction, GameState, Result};
use std::time::Duration;

/// Configuration for the Genius client
#[derive(Debug, Clone)]
pub struct ClientConfig {
    pub server_url: String,
    pub timeout: Duration,
    pub player_id: String,
}

/// Main client for interacting with Genius server
pub struct GeniusClient {
    config: ClientConfig,
}

impl GeniusClient {
    pub fn new(config: ClientConfig) -> Self {
        Self { config }
    }
    
    pub async fn connect(&mut self) -> Result<()> {
        // Implementation would connect to server
        Ok(())
    }
    
    pub async fn join_game(&mut self, game_type: GameType) -> Result<String> {
        // Implementation would join a game
        Ok("game_id".to_string())
    }
    
    pub async fn submit_action(&mut self, action: PlayerAction) -> Result<()> {
        // Implementation would submit an action
        Ok(())
    }
    
    pub async fn get_game_state(&self, game_id: &str) -> Result<GameState> {
        // Implementation would fetch game state
        unimplemented!("Client implementation pending")
    }
}