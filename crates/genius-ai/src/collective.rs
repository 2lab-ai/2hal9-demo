//! Collective intelligence and swarm behavior

use genius_core::{PlayerAction, GameState, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Strategy for collective decision making
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CollectiveStrategy {
    Majority,
    Consensus,
    WeightedVote,
    RandomSample,
}

/// Result of collective decision making
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CollectiveDecision {
    pub action: PlayerAction,
    pub votes: HashMap<String, PlayerAction>,
    pub consensus_score: f32,
    pub dissent_rate: f32,
}

/// Manager for collective intelligence
pub struct CollectiveIntelligence {
    strategy: CollectiveStrategy,
    agents: Vec<String>,
}

impl CollectiveIntelligence {
    pub fn new(strategy: CollectiveStrategy) -> Self {
        Self {
            strategy,
            agents: Vec::new(),
        }
    }
    
    pub fn add_agent(&mut self, agent_id: String) {
        self.agents.push(agent_id);
    }
    
    pub async fn make_collective_decision(
        &self,
        _game_state: &GameState,
        individual_decisions: HashMap<String, PlayerAction>,
    ) -> Result<CollectiveDecision> {
        match self.strategy {
            CollectiveStrategy::Majority => self.majority_vote(individual_decisions),
            CollectiveStrategy::Consensus => self.consensus_decision(individual_decisions),
            CollectiveStrategy::WeightedVote => self.weighted_vote(individual_decisions),
            CollectiveStrategy::RandomSample => self.random_sample(individual_decisions),
        }
    }
    
    fn majority_vote(&self, decisions: HashMap<String, PlayerAction>) -> Result<CollectiveDecision> {
        // Count votes for each action type
        let mut vote_counts: HashMap<String, i32> = HashMap::new();
        
        for action in decisions.values() {
            *vote_counts.entry(action.action_type.clone()).or_insert(0) += 1;
        }
        
        // Find the action with most votes
        let (winning_action_type, _) = vote_counts.iter()
            .max_by_key(|(_, &count)| count)
            .ok_or_else(|| genius_core::GameError::InvalidAction { reason: "No votes".to_string() })?;
        
        // Get the first action of the winning type
        let winning_action = decisions.values()
            .find(|a| &a.action_type == winning_action_type)
            .ok_or_else(|| genius_core::GameError::InvalidAction { reason: "No matching action".to_string() })?
            .clone();
        
        let total_agents = decisions.len() as f32;
        let consensus_score = vote_counts[winning_action_type] as f32 / total_agents;
        let dissent_rate = 1.0 - consensus_score;
        
        Ok(CollectiveDecision {
            action: winning_action,
            votes: decisions,
            consensus_score,
            dissent_rate,
        })
    }
    
    fn consensus_decision(&self, decisions: HashMap<String, PlayerAction>) -> Result<CollectiveDecision> {
        // For now, same as majority vote
        // TODO: Implement true consensus mechanism
        self.majority_vote(decisions)
    }
    
    fn weighted_vote(&self, decisions: HashMap<String, PlayerAction>) -> Result<CollectiveDecision> {
        // For now, same as majority vote
        // TODO: Implement confidence-weighted voting
        self.majority_vote(decisions)
    }
    
    fn random_sample(&self, decisions: HashMap<String, PlayerAction>) -> Result<CollectiveDecision> {
        // Pick a random decision
        let action = decisions.values()
            .next()
            .ok_or_else(|| genius_core::GameError::InvalidAction { reason: "No decisions".to_string() })?
            .clone();
            
        Ok(CollectiveDecision {
            action,
            votes: decisions,
            consensus_score: 0.0,
            dissent_rate: 1.0,
        })
    }
}