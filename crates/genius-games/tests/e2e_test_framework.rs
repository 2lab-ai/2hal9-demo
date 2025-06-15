//! End-to-end test framework for games

use genius_core::{
    Game, GameConfig, GameType, Player, PlayerType, PlayerAction, 
    GameState, RoundResult, GameResult,
};
use genius_games::create_game;
use genius_ai::providers::mock::MockProvider;
use genius_ai::provider::AIProvider;
use std::collections::HashMap;
use std::time::Duration;
use tokio::time::timeout;

/// Test configuration for e2e tests
pub struct E2ETestConfig {
    pub game_type: GameType,
    pub num_players: usize,
    pub max_rounds: u32,
    pub timeout_per_round: Duration,
    pub enable_emergence_tracking: bool,
    pub seed: Option<u64>,
}

impl Default for E2ETestConfig {
    fn default() -> Self {
        Self {
            game_type: GameType::MinorityGame,
            num_players: 4,
            max_rounds: 20,
            timeout_per_round: Duration::from_secs(5),
            enable_emergence_tracking: true,
            seed: None,
        }
    }
}

/// Test runner for e2e game tests
pub struct E2ETestRunner {
    config: E2ETestConfig,
    ai_provider: Box<dyn AIProvider>,
}

impl E2ETestRunner {
    pub fn new(config: E2ETestConfig) -> Self {
        Self {
            config,
            ai_provider: Box::new(MockProvider::new()),
        }
    }
    
    pub fn with_deterministic_ai(mut self) -> Self {
        self.ai_provider = Box::new(MockProvider::deterministic());
        self
    }
    
    /// Run a complete game and return the result
    pub async fn run_game(&self) -> anyhow::Result<GameTestResult> {
        let mut game = create_game(self.config.game_type.clone())?;
        
        // Create players
        let players = self.create_test_players();
        
        // Initialize game
        let game_config = GameConfig {
            game_type: self.config.game_type.clone(),
            rounds: self.config.max_rounds,
            time_limit_ms: self.config.timeout_per_round.as_millis() as u64,
            special_rules: HashMap::new(),
            initial_players: players.clone(),
        };
        
        let mut state = game.initialize(game_config).await?;
        
        // Add players to state
        for player in &players {
            state.add_player(player.id.to_string());
        }
        
        let mut round_results = Vec::new();
        let mut emergence_events = Vec::new();
        
        // Run game rounds
        while !game.is_game_over(&state).await && state.round < self.config.max_rounds {
            state.round += 1;
            
            // Get valid actions for each player
            let mut actions = HashMap::new();
            
            for player in &players {
                let valid_actions = game.get_valid_actions(&state, &player.id.to_string()).await;
                
                // Use AI to decide action
                let decision = timeout(
                    self.config.timeout_per_round,
                    self.ai_provider.make_decision(&state, &player.id.to_string(), valid_actions)
                ).await??;
                
                actions.insert(player.id.to_string(), decision.action);
            }
            
            // Process round
            let round_result = game.process_round(&state, actions).await?;
            
            // Update state
            state.apply_score_deltas(&round_result.scores_delta);
            state.history.push(round_result.clone());
            
            // Track emergence
            if self.config.enable_emergence_tracking {
                for event in &round_result.events {
                    if event.event_type == "emergence" {
                        emergence_events.push(event.clone());
                    }
                }
            }
            
            round_results.push(round_result);
        }
        
        // Calculate final result
        let final_result = game.calculate_final_result(&state).await;
        
        Ok(GameTestResult {
            game_type: self.config.game_type.clone(),
            final_result,
            round_results,
            emergence_events,
            total_rounds: state.round,
            final_state: state,
        })
    }
    
    /// Run multiple games and collect statistics
    pub async fn run_multiple_games(&self, num_games: usize) -> anyhow::Result<GameTestStats> {
        let mut results = Vec::new();
        
        for _ in 0..num_games {
            let result = self.run_game().await?;
            results.push(result);
        }
        
        Ok(GameTestStats::from_results(results))
    }
    
    fn create_test_players(&self) -> Vec<Player> {
        (0..self.config.num_players)
            .map(|i| Player {
                id: genius_core::player::PlayerId::from_string(format!("test_player_{}", i)),
                name: format!("Test Player {}", i),
                player_type: PlayerType::AI {
                    provider: "mock".to_string(),
                    model: "test".to_string(),
                },
                metadata: serde_json::json!({}),
            })
            .collect()
    }
}

/// Result of a single game test
#[derive(Debug)]
pub struct GameTestResult {
    pub game_type: GameType,
    pub final_result: GameResult,
    pub round_results: Vec<RoundResult>,
    pub emergence_events: Vec<genius_core::GameEvent>,
    pub total_rounds: u32,
    pub final_state: GameState,
}

impl GameTestResult {
    /// Check if the game completed successfully
    pub fn is_successful(&self) -> bool {
        !self.final_result.winner.is_empty() && self.total_rounds > 0
    }
    
    /// Get emergence frequency
    pub fn emergence_frequency(&self) -> f32 {
        self.emergence_events.len() as f32 / self.total_rounds.max(1) as f32
    }
    
    /// Check if specific emergence type occurred
    pub fn has_emergence_type(&self, emergence_type: &str) -> bool {
        self.emergence_events.iter().any(|e| {
            e.data.get("emergence_type")
                .and_then(|v| v.as_str())
                .map(|s| s == emergence_type)
                .unwrap_or(false)
        })
    }
}

/// Statistics from multiple game runs
#[derive(Debug)]
pub struct GameTestStats {
    pub total_games: usize,
    pub successful_games: usize,
    pub avg_rounds: f32,
    pub avg_emergence_frequency: f32,
    pub winner_distribution: HashMap<String, usize>,
    pub emergence_type_counts: HashMap<String, usize>,
}

impl GameTestStats {
    fn from_results(results: Vec<GameTestResult>) -> Self {
        let total_games = results.len();
        let successful_games = results.iter().filter(|r| r.is_successful()).count();
        
        let avg_rounds = results.iter()
            .map(|r| r.total_rounds as f32)
            .sum::<f32>() / total_games.max(1) as f32;
            
        let avg_emergence_frequency = results.iter()
            .map(|r| r.emergence_frequency())
            .sum::<f32>() / total_games.max(1) as f32;
            
        let mut winner_distribution = HashMap::new();
        for result in &results {
            *winner_distribution.entry(result.final_result.winner.clone()).or_insert(0) += 1;
        }
        
        let mut emergence_type_counts = HashMap::new();
        for result in &results {
            for event in &result.emergence_events {
                if let Some(etype) = event.data.get("emergence_type").and_then(|v| v.as_str()) {
                    *emergence_type_counts.entry(etype.to_string()).or_insert(0) += 1;
                }
            }
        }
        
        Self {
            total_games,
            successful_games,
            avg_rounds,
            avg_emergence_frequency,
            winner_distribution,
            emergence_type_counts,
        }
    }
    
    /// Print a summary of the statistics
    pub fn print_summary(&self) {
        println!("=== Game Test Statistics ===");
        println!("Total games: {}", self.total_games);
        println!("Successful games: {} ({:.1}%)", 
            self.successful_games, 
            self.successful_games as f32 / self.total_games as f32 * 100.0);
        println!("Average rounds: {:.1}", self.avg_rounds);
        println!("Average emergence frequency: {:.2}", self.avg_emergence_frequency);
        
        println!("\nWinner distribution:");
        for (winner, count) in &self.winner_distribution {
            println!("  {}: {} ({:.1}%)", 
                winner, 
                count, 
                *count as f32 / self.total_games as f32 * 100.0);
        }
        
        if !self.emergence_type_counts.is_empty() {
            println!("\nEmergence types observed:");
            for (etype, count) in &self.emergence_type_counts {
                println!("  {}: {}", etype, count);
            }
        }
    }
}

/// Assertion helpers for tests
pub mod assertions {
    use super::*;
    
    /// Assert that a game completes successfully
    pub fn assert_game_completes(result: &GameTestResult) {
        assert!(result.is_successful(), "Game did not complete successfully");
        assert!(result.total_rounds > 0, "Game ended with 0 rounds");
    }
    
    /// Assert that emergence occurs with minimum frequency
    pub fn assert_emergence_frequency(result: &GameTestResult, min_frequency: f32) {
        let actual = result.emergence_frequency();
        assert!(
            actual >= min_frequency,
            "Emergence frequency {} is below minimum {}",
            actual,
            min_frequency
        );
    }
    
    /// Assert that game ends within round limit
    pub fn assert_game_ends_within_rounds(result: &GameTestResult, max_rounds: u32) {
        assert!(
            result.total_rounds <= max_rounds,
            "Game took {} rounds, exceeding limit of {}",
            result.total_rounds,
            max_rounds
        );
    }
    
    /// Assert fair winner distribution
    pub fn assert_fair_winner_distribution(stats: &GameTestStats, tolerance: f32) {
        let expected_win_rate = 1.0 / stats.winner_distribution.len() as f32;
        
        for (winner, count) in &stats.winner_distribution {
            let actual_rate = *count as f32 / stats.total_games as f32;
            let deviation = (actual_rate - expected_win_rate).abs();
            
            assert!(
                deviation <= tolerance,
                "Player {} has win rate {:.2} (expected {:.2} ± {:.2})",
                winner,
                actual_rate,
                expected_win_rate,
                tolerance
            );
        }
    }
}