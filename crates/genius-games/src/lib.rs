//! Game implementations for the Genius platform
//! 
//! This crate contains all game implementations organized by category.

pub mod strategic;
pub mod collective;
pub mod survival;
pub mod trust;

use genius_core::{Game, GameConfig, GameType, GameError};
use std::sync::Arc;

/// Factory function to create game instances
pub fn create_game(game_type: GameType) -> Result<Box<dyn Game>, GameError> {
    match game_type {
        // Strategic Games
        GameType::MinorityGame => Ok(Box::new(strategic::minority_game::MinorityGame::new())),
        GameType::ByzantineGenerals => Ok(Box::new(strategic::byzantine_generals::ByzantineGeneralsGame::new())),
        GameType::MiniGo => Ok(Box::new(strategic::mini_go::MiniGoGame::new())),
        GameType::MiniHoldem => Ok(Box::new(strategic::mini_holdem::MiniHoldemGame::new())),
        GameType::VoidWalker => Ok(Box::new(strategic::void_walker::VoidWalkerGame::new())),
        GameType::ObserverGame => Ok(Box::new(strategic::observer_game::ObserverGame::new())),
        GameType::QuantumDreamer => Ok(Box::new(strategic::quantum_dreamer::QuantumDreamerGame::new())),
        
        // Collective Intelligence Games
        GameType::CollectiveMaze => Ok(Box::new(collective::collective_maze::CollectiveMazeGame::new())),
        GameType::SwarmOptimization => Ok(Box::new(collective::swarm_optimization::SwarmOptimizationGame::new())),
        GameType::RecursiveReasoning => Ok(Box::new(collective::recursive_reasoning::RecursiveReasoningGame::new())),
        GameType::QuantumConsensus => Ok(Box::new(collective::quantum_consensus::QuantumConsensusGame::new())),
        GameType::MirrorMind => Ok(Box::new(collective::mirror_mind::MirrorMindGame::new())),
        GameType::RealityConsensus => Ok(Box::new(collective::reality_consensus::RealityConsensusGame::new())),
        GameType::InformationHorizon => Ok(Box::new(collective::information_horizon::InformationHorizonGame::new())),
        GameType::ConsciousnessCascade => Ok(Box::new(collective::consciousness_cascade::ConsciousnessCascadeGame::new())),
        
        // Survival Games
        GameType::BattleRoyale => Ok(Box::new(survival::battle_royale::BattleRoyaleGame::new())),
        GameType::HungerGames => Ok(Box::new(survival::hunger_games::HungerGamesGame::new())),
        GameType::SquidGame => Ok(Box::new(survival::squid_game::SquidGame::new())),
        GameType::RussianRoulette => Ok(Box::new(survival::russian_roulette::RussianRoulette::new())),
        GameType::KingOfTheHill => Ok(Box::new(survival::king_of_the_hill::KingOfTheHill::new())),
        GameType::LastStand => Ok(Box::new(survival::last_stand::LastStand::new())),
        
        // Trust Games
        GameType::PrisonersDilemma => Ok(Box::new(trust::prisoners_dilemma::PrisonersDilemmaGame::new())),
        GameType::TrustFall => Ok(Box::new(trust::trust_fall::TrustFall::new())),
        GameType::LiarsDice => Ok(Box::new(trust::liars_dice::LiarsDiceGame::new())),
        GameType::ConsciousnessPoker => Ok(Box::new(trust::consciousness_poker::ConsciousnessPokerGame::new())),
    }
}

/// Registry of all available games
pub struct GameRegistry {
    games: Vec<GameType>,
}

impl GameRegistry {
    pub fn new() -> Self {
        Self {
            games: vec![
                // Strategic
                GameType::MinorityGame,
                GameType::ByzantineGenerals,
                GameType::MiniGo,
                GameType::MiniHoldem,
                GameType::VoidWalker,
                GameType::ObserverGame,
                GameType::QuantumDreamer,
                // Collective
                GameType::CollectiveMaze,
                GameType::SwarmOptimization,
                GameType::RecursiveReasoning,
                GameType::QuantumConsensus,
                GameType::MirrorMind,
                GameType::RealityConsensus,
                GameType::InformationHorizon,
                GameType::ConsciousnessCascade,
                // Survival
                GameType::BattleRoyale,
                GameType::HungerGames,
                GameType::SquidGame,
                GameType::RussianRoulette,
                GameType::KingOfTheHill,
                GameType::LastStand,
                // Trust
                GameType::PrisonersDilemma,
                GameType::TrustFall,
                GameType::LiarsDice,
                GameType::ConsciousnessPoker,
            ],
        }
    }
    
    pub fn all_games(&self) -> &[GameType] {
        &self.games
    }
    
    pub fn games_by_category(&self, category: genius_core::GameCategory) -> Vec<GameType> {
        self.games.iter()
            .filter(|game| game.category() == category)
            .cloned()
            .collect()
    }
}

impl Default for GameRegistry {
    fn default() -> Self {
        Self::new()
    }
}