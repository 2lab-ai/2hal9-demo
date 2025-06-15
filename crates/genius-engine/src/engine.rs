//! Core game engine implementation

use genius_core::{
    Game, GameConfig, GameState, GameType, RoundResult, GameResult,
    PlayerAction, GameError, Result,
};
use dashmap::DashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use uuid::Uuid;
use std::collections::HashMap;

/// The main game engine that manages all active games
pub struct GameEngine {
    /// Active game instances
    games: DashMap<Uuid, Arc<RwLock<GameInstance>>>,
    /// Game factory function
    game_factory: Arc<dyn Fn(GameType) -> Result<Box<dyn Game>> + Send + Sync>,
}

/// A single game instance with its state
struct GameInstance {
    game: Box<dyn Game>,
    state: GameState,
    config: GameConfig,
}

impl GameEngine {
    /// Create a new game engine
    pub fn new<F>(game_factory: F) -> Self 
    where 
        F: Fn(GameType) -> Result<Box<dyn Game>> + Send + Sync + 'static
    {
        Self {
            games: DashMap::new(),
            game_factory: Arc::new(game_factory),
        }
    }
    
    /// Create a new game instance
    pub async fn create_game(&self, config: GameConfig) -> Result<GameState> {
        // Create game instance
        let mut game = (self.game_factory)(config.game_type.clone())?;
        
        // Initialize game
        let state = game.initialize(config.clone()).await?;
        let game_id = state.game_id;
        
        // Store instance
        let instance = GameInstance {
            game,
            state: state.clone(),
            config,
        };
        
        self.games.insert(game_id, Arc::new(RwLock::new(instance)));
        
        Ok(state)
    }
    
    /// Process a turn for a game
    pub async fn process_turn(
        &self, 
        game_id: Uuid, 
        actions: HashMap<String, PlayerAction>
    ) -> Result<RoundResult> {
        let game_arc = self.games.get(&game_id)
            .ok_or_else(|| GameError::GameNotFound { id: game_id.to_string() })?
            .clone();
            
        let mut instance = game_arc.write().await;
        
        // Check if game is over
        if instance.game.is_game_over(&instance.state).await {
            return Err(GameError::GameAlreadyEnded);
        }
        
        // Process round (clone state to avoid borrow checker issues)
        let state_clone = instance.state.clone();
        let round_result = instance.game.process_round(&state_clone, actions).await?;
        
        // Update state
        instance.state.round += 1;
        instance.state.apply_score_deltas(&round_result.scores_delta);
        instance.state.history.push(round_result.clone());
        instance.state.updated_at = chrono::Utc::now();
        
        Ok(round_result)
    }
    
    /// Get current game state
    pub async fn get_game_state(&self, game_id: Uuid) -> Result<GameState> {
        let game_arc = self.games.get(&game_id)
            .ok_or_else(|| GameError::GameNotFound { id: game_id.to_string() })?;
            
        let instance = game_arc.read().await;
        Ok(instance.state.clone())
    }
    
    /// Finalize a game and get results
    pub async fn finalize_game(&self, game_id: Uuid) -> Result<GameResult> {
        let game_arc = self.games.get(&game_id)
            .ok_or_else(|| GameError::GameNotFound { id: game_id.to_string() })?
            .clone();
            
        let instance = game_arc.read().await;
        let result = instance.game.calculate_final_result(&instance.state).await;
        
        // Remove from active games
        drop(instance);
        self.games.remove(&game_id);
        
        Ok(result)
    }
    
    /// Get list of active games
    pub fn active_games(&self) -> Vec<Uuid> {
        self.games.iter().map(|entry| *entry.key()).collect()
    }
    
    /// Get valid actions for a player
    pub async fn get_valid_actions(
        &self, 
        game_id: Uuid, 
        player_id: &str
    ) -> Result<Vec<String>> {
        let game_arc = self.games.get(&game_id)
            .ok_or_else(|| GameError::GameNotFound { id: game_id.to_string() })?;
            
        let instance = game_arc.read().await;
        Ok(instance.game.get_valid_actions(&instance.state, player_id).await)
    }
}