//! Core game trait and types

use crate::{state::*, player::{Player, PlayerAction}, error::Result};
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Configuration for a game instance
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameConfig {
    pub game_type: GameType,
    pub rounds: u32,
    pub time_limit_ms: u64,
    pub special_rules: HashMap<String, String>,
    pub initial_players: Vec<Player>,
}

/// Available game types in the platform
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum GameType {
    // Strategic Games
    MinorityGame,
    ByzantineGenerals,
    MiniGo,
    MiniHoldem,
    VoidWalker,
    ObserverGame,
    QuantumDreamer,
    
    // Collective Intelligence Games
    CollectiveMaze,
    SwarmOptimization,
    RecursiveReasoning,
    QuantumConsensus,
    MirrorMind,
    RealityConsensus,
    InformationHorizon,
    ConsciousnessCascade,
    
    // Survival/Death Games
    BattleRoyale,
    HungerGames,
    SquidGame,
    RussianRoulette,
    KingOfTheHill,
    LastStand,
    
    // Trust-based Games
    PrisonersDilemma,
    TrustFall,
    LiarsDice,
    ConsciousnessPoker,
}

impl GameType {
    /// Get human-readable name for the game type
    pub fn display_name(&self) -> &'static str {
        match self {
            Self::MinorityGame => "Minority Game",
            Self::ByzantineGenerals => "Byzantine Generals",
            Self::MiniGo => "Mini Go",
            Self::MiniHoldem => "Mini Hold'em",
            Self::VoidWalker => "Void Walker",
            Self::ObserverGame => "The Observer Game",
            Self::QuantumDreamer => "Quantum Dreamer",
            Self::CollectiveMaze => "Collective Maze",
            Self::SwarmOptimization => "Swarm Optimization",
            Self::RecursiveReasoning => "Recursive Reasoning",
            Self::QuantumConsensus => "Quantum Consensus",
            Self::MirrorMind => "Mirror Mind",
            Self::RealityConsensus => "Reality Consensus",
            Self::InformationHorizon => "Information Horizon",
            Self::ConsciousnessCascade => "Consciousness Cascade",
            Self::BattleRoyale => "Battle Royale",
            Self::HungerGames => "Hunger Games",
            Self::SquidGame => "Squid Game",
            Self::RussianRoulette => "Russian Roulette",
            Self::KingOfTheHill => "King of the Hill",
            Self::LastStand => "Last Stand",
            Self::PrisonersDilemma => "Prisoner's Dilemma",
            Self::TrustFall => "Trust Fall",
            Self::LiarsDice => "Liar's Dice",
            Self::ConsciousnessPoker => "Consciousness Poker",
        }
    }
    
    /// Get category of the game
    pub fn category(&self) -> GameCategory {
        match self {
            Self::MinorityGame | Self::ByzantineGenerals | 
            Self::MiniGo | Self::MiniHoldem |
            Self::VoidWalker | Self::ObserverGame |
            Self::QuantumDreamer => GameCategory::Strategic,
            
            Self::CollectiveMaze | Self::SwarmOptimization |
            Self::RecursiveReasoning | Self::QuantumConsensus |
            Self::MirrorMind | Self::RealityConsensus |
            Self::InformationHorizon | Self::ConsciousnessCascade => GameCategory::Collective,
            
            Self::BattleRoyale | Self::HungerGames | Self::SquidGame |
            Self::RussianRoulette | Self::KingOfTheHill | Self::LastStand => GameCategory::Survival,
            
            Self::PrisonersDilemma | Self::TrustFall | Self::LiarsDice |
            Self::ConsciousnessPoker => GameCategory::Trust,
        }
    }
}

/// Game categories for organization
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GameCategory {
    Strategic,
    Collective,
    Survival,
    Trust,
}

/// Core trait that all games must implement
#[async_trait]
pub trait Game: Send + Sync {
    /// Initialize a new game with the given configuration
    async fn initialize(&mut self, config: GameConfig) -> Result<GameState>;
    
    /// Process a single round with player actions
    async fn process_round(
        &mut self, 
        state: &GameState, 
        actions: HashMap<String, PlayerAction>
    ) -> Result<RoundResult>;
    
    /// Check if the game has reached an end condition
    async fn is_game_over(&self, state: &GameState) -> bool;
    
    /// Calculate final results and analytics
    async fn calculate_final_result(&self, state: &GameState) -> GameResult;
    
    /// Get valid actions for a player in the current state
    async fn get_valid_actions(&self, _state: &GameState, _player_id: &str) -> Vec<String> {
        // Default implementation returns empty vec
        // Games should override this to provide action hints
        vec![]
    }
    
    /// Get game-specific state for visualization
    async fn get_visualization_data(&self, _state: &GameState) -> serde_json::Value {
        // Default implementation returns empty object
        // Games can override to provide custom visualization data
        serde_json::json!({})
    }
}