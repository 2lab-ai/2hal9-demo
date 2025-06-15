//! Reality Consensus - A game where collective belief shapes reality

use genius_core::{
    Game, GameConfig, GameState, GameResult, PlayerAction, RoundResult,
    RoundOutcome, GameEvent, GameAnalytics, EmergenceEvent, EmergenceType,
    Result, GameError,
};
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// The state of a reality fragment
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum RealityState {
    Stable,
    Unstable,
    Glitched,
    Collapsing,
    Transcendent,
}

/// A fragment of reality that can be voted on
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RealityFragment {
    id: String,
    description: String,
    current_state: RealityState,
    belief_votes: HashMap<String, bool>,
    stability: f32,
    glitch_probability: f32,
}

/// Player's belief action
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BeliefAction {
    fragment_id: String,
    believes: bool,
    conviction_strength: f32, // 0.0 to 1.0
    proposed_reality: Option<String>, // New reality to propose
}

/// A glitch in reality caused by dissent
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RealityGlitch {
    fragment_id: String,
    severity: f32,
    affected_players: Vec<String>,
    consequence: String,
}

pub struct RealityConsensusGame {
    round_number: u32,
    reality_fragments: HashMap<String, RealityFragment>,
    glitches: Vec<RealityGlitch>,
    consensus_threshold: f32,
    reality_stability: f32,
    entropy_rate: f32,
}

impl RealityConsensusGame {
    pub fn new() -> Self {
        Self {
            round_number: 0,
            reality_fragments: HashMap::new(),
            glitches: Vec::new(),
            consensus_threshold: 0.66, // 2/3 majority needed
            reality_stability: 1.0,
            entropy_rate: 0.05,
        }
    }
    
    fn initialize_reality_fragments(&mut self) {
        let fragments = vec![
            ("gravity", "Gravity pulls objects downward"),
            ("time", "Time flows forward linearly"),
            ("causality", "Causes precede effects"),
            ("identity", "Each player is a separate entity"),
            ("logic", "Contradictions cannot exist"),
            ("perception", "What we see is real"),
            ("memory", "The past is fixed"),
            ("physics", "Energy is conserved"),
        ];
        
        for (id, description) in fragments {
            self.reality_fragments.insert(id.to_string(), RealityFragment {
                id: id.to_string(),
                description: description.to_string(),
                current_state: RealityState::Stable,
                belief_votes: HashMap::new(),
                stability: 1.0,
                glitch_probability: 0.0,
            });
        }
    }
    
    fn calculate_consensus(&self, fragment: &RealityFragment) -> f32 {
        if fragment.belief_votes.is_empty() {
            return 0.0;
        }
        
        let believers = fragment.belief_votes.values().filter(|&&v| v).count() as f32;
        let total = fragment.belief_votes.len() as f32;
        believers / total
    }
    
    fn update_reality_state(&mut self, fragment_id: &str, consensus: f32) -> Vec<GameEvent> {
        let mut events = Vec::new();
        
        if let Some(fragment) = self.reality_fragments.get_mut(fragment_id) {
            let old_state = fragment.current_state.clone();
            
            // Update stability based on consensus
            fragment.stability = consensus;
            fragment.glitch_probability = 1.0 - consensus;
            
            // Determine new state
            fragment.current_state = match consensus {
                c if c >= 0.9 => RealityState::Transcendent,
                c if c >= self.consensus_threshold => RealityState::Stable,
                c if c >= 0.5 => RealityState::Unstable,
                c if c >= 0.25 => RealityState::Glitched,
                _ => RealityState::Collapsing,
            };
            
            if old_state != fragment.current_state {
                events.push(GameEvent {
                    event_type: "reality_shift".to_string(),
                    description: format!("Reality fragment '{}' shifted from {:?} to {:?}", 
                        fragment_id, old_state, fragment.current_state),
                    affected_players: fragment.belief_votes.keys().cloned().collect(),
                    data: serde_json::json!({
                        "fragment": fragment_id,
                        "old_state": old_state,
                        "new_state": fragment.current_state,
                        "consensus": consensus,
                    }),
                });
            }
            
            // Generate glitches for unstable realities
            if fragment.glitch_probability > 0.5 {
                let dissenters: Vec<_> = fragment.belief_votes.iter()
                    .filter(|(_, &believes)| !believes)
                    .map(|(player, _)| player.clone())
                    .collect();
                    
                if !dissenters.is_empty() {
                    let glitch = RealityGlitch {
                        fragment_id: fragment_id.to_string(),
                        severity: fragment.glitch_probability,
                        affected_players: dissenters.clone(),
                        consequence: self.generate_glitch_consequence(fragment_id, fragment.glitch_probability),
                    };
                    
                    events.push(GameEvent {
                        event_type: "reality_glitch".to_string(),
                        description: format!("Reality glitch in '{}': {}", fragment_id, glitch.consequence),
                        affected_players: glitch.affected_players.clone(),
                        data: serde_json::json!({
                            "fragment": fragment_id,
                            "severity": glitch.severity,
                            "consequence": glitch.consequence,
                        }),
                    });
                    
                    self.glitches.push(glitch);
                }
            }
        }
        
        events
    }
    
    fn generate_glitch_consequence(&self, fragment_id: &str, severity: f32) -> String {
        match fragment_id {
            "gravity" => format!("Players experience {:.0}% gravity reversal", severity * 100.0),
            "time" => format!("Time loops back {:.0} rounds", (severity * 3.0).ceil()),
            "causality" => "Effects now precede causes for affected players".to_string(),
            "identity" => "Player identities begin to merge and blur".to_string(),
            "logic" => "Contradictions become temporarily possible".to_string(),
            "perception" => format!("Reality appears {:.0}% different to each player", severity * 100.0),
            "memory" => "Past events become mutable and uncertain".to_string(),
            "physics" => format!("Conservation laws violated by {:.0}%", severity * 100.0),
            _ => "Unknown reality distortion occurs".to_string(),
        }
    }
    
    fn calculate_global_stability(&self) -> f32 {
        if self.reality_fragments.is_empty() {
            return 1.0;
        }
        
        let total_stability: f32 = self.reality_fragments.values()
            .map(|f| f.stability)
            .sum();
            
        total_stability / self.reality_fragments.len() as f32
    }
    
    fn detect_reality_collapse(&self) -> Option<EmergenceEvent> {
        let collapse_count = self.reality_fragments.values()
            .filter(|f| matches!(f.current_state, RealityState::Collapsing))
            .count();
            
        let collapse_ratio = collapse_count as f32 / self.reality_fragments.len() as f32;
        
        if collapse_ratio > 0.5 {
            Some(EmergenceEvent {
                round: self.round_number,
                event_type: EmergenceType::Custom("RealityCollapse".to_string()),
                description: format!("Reality collapsing! {:.0}% of fragments unstable", collapse_ratio * 100.0),
                emergence_score: collapse_ratio,
                involved_players: self.reality_fragments.values()
                    .flat_map(|f| f.belief_votes.keys().cloned())
                    .collect(),
            })
        } else {
            None
        }
    }
    
    fn detect_transcendent_consensus(&self) -> Option<EmergenceEvent> {
        let transcendent_count = self.reality_fragments.values()
            .filter(|f| matches!(f.current_state, RealityState::Transcendent))
            .count();
            
        let transcendent_ratio = transcendent_count as f32 / self.reality_fragments.len() as f32;
        
        if transcendent_ratio > 0.75 {
            Some(EmergenceEvent {
                round: self.round_number,
                event_type: EmergenceType::Custom("TranscendentReality".to_string()),
                description: "Players achieving transcendent consensus on reality!".to_string(),
                emergence_score: transcendent_ratio,
                involved_players: self.reality_fragments.values()
                    .flat_map(|f| f.belief_votes.keys().cloned())
                    .collect(),
            })
        } else {
            None
        }
    }
}

impl Default for RealityConsensusGame {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl Game for RealityConsensusGame {
    async fn initialize(&mut self, config: GameConfig) -> Result<GameState> {
        let mut state = GameState::new(config.game_type.clone());
        
        // Initialize reality fragments
        self.initialize_reality_fragments();
        
        state.metadata.insert(
            "description".to_string(),
            serde_json::json!("Shape reality through collective belief - but beware the glitches of dissent"),
        );
        state.metadata.insert(
            "consensus_threshold".to_string(),
            serde_json::json!(self.consensus_threshold),
        );
        state.metadata.insert(
            "reality_fragments".to_string(),
            serde_json::json!(self.reality_fragments.keys().collect::<Vec<_>>()),
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
        
        // Apply entropy - reality naturally decays without maintenance
        self.reality_stability *= (1.0 - self.entropy_rate);
        
        // Clear previous round's votes
        for fragment in self.reality_fragments.values_mut() {
            fragment.belief_votes.clear();
        }
        
        // Process player beliefs
        for (player_id, action) in &actions {
            if let Ok(belief) = serde_json::from_value::<BeliefAction>(action.data.clone()) {
                // Record belief vote
                if let Some(fragment) = self.reality_fragments.get_mut(&belief.fragment_id) {
                    fragment.belief_votes.insert(player_id.clone(), belief.believes);
                    
                    // Stronger conviction = more influence
                    if belief.conviction_strength > 0.8 {
                        scores_delta.insert(player_id.clone(), 10);
                    }
                }
                
                // Handle new reality proposals
                if let Some(new_reality) = belief.proposed_reality {
                    let proposal_id = format!("custom_{}", self.round_number);
                    self.reality_fragments.insert(proposal_id.clone(), RealityFragment {
                        id: proposal_id.clone(),
                        description: new_reality.clone(),
                        current_state: RealityState::Unstable,
                        belief_votes: HashMap::new(),
                        stability: 0.5,
                        glitch_probability: 0.5,
                    });
                    
                    events.push(GameEvent {
                        event_type: "reality_proposal".to_string(),
                        description: format!("{} proposes new reality: {}", player_id, new_reality),
                        affected_players: vec![player_id.clone()],
                        data: serde_json::json!({
                            "proposer": player_id,
                            "fragment_id": proposal_id,
                            "description": new_reality,
                        }),
                    });
                }
            }
        }
        
        // Update reality states based on consensus
        let fragment_ids: Vec<_> = self.reality_fragments.keys().cloned().collect();
        for fragment_id in fragment_ids {
            let consensus = self.reality_fragments.get(&fragment_id)
                .map(|f| self.calculate_consensus(f))
                .unwrap_or(0.0);
                
            let mut fragment_events = self.update_reality_state(&fragment_id, consensus);
            events.append(&mut fragment_events);
        }
        
        // Award points based on consensus participation
        for (fragment_id, fragment) in &self.reality_fragments {
            let consensus = self.calculate_consensus(fragment);
            
            if consensus >= self.consensus_threshold {
                // Reward consensus builders
                for (player_id, &believes) in &fragment.belief_votes {
                    if believes {
                        *scores_delta.entry(player_id.clone()).or_insert(0) += 5;
                    }
                }
            } else if consensus < 0.5 {
                // Penalize reality breakers (unless that's the goal)
                for (player_id, &believes) in &fragment.belief_votes {
                    if !believes {
                        *scores_delta.entry(player_id.clone()).or_insert(0) -= 2;
                    }
                }
            }
        }
        
        // Check for emergence events
        if let Some(collapse) = self.detect_reality_collapse() {
            events.push(GameEvent {
                event_type: "emergence".to_string(),
                description: collapse.description.clone(),
                affected_players: collapse.involved_players.clone(),
                data: serde_json::json!({ "type": "reality_collapse" }),
            });
        }
        
        if let Some(transcendent) = self.detect_transcendent_consensus() {
            events.push(GameEvent {
                event_type: "emergence".to_string(),
                description: transcendent.description.clone(),
                affected_players: transcendent.involved_players.clone(),
                data: serde_json::json!({ "type": "transcendent_reality" }),
            });
            
            // Bonus points for achieving transcendence
            for player_id in &transcendent.involved_players {
                *scores_delta.entry(player_id.clone()).or_insert(0) += 50;
            }
        }
        
        // Update global stability
        self.reality_stability = self.calculate_global_stability();
        
        let outcome = RoundOutcome {
            winners: scores_delta.iter()
                .filter(|(_, &score)| score > 0)
                .map(|(player, _)| player.clone())
                .collect(),
            losers: scores_delta.iter()
                .filter(|(_, &score)| score < 0)
                .map(|(player, _)| player.clone())
                .collect(),
            special_events: events.iter().map(|e| e.description.clone()).collect(),
            emergence_detected: events.iter().any(|e| e.event_type == "emergence"),
        };
        
        Ok(RoundResult {
            round: state.round,
            actions,
            outcome,
            scores_delta,
            events,
            timestamp: chrono::Utc::now(),
        })
    }
    
    async fn is_game_over(&self, state: &GameState) -> bool {
        // Game ends if reality completely collapses or transcends
        let collapse_ratio = self.reality_fragments.values()
            .filter(|f| matches!(f.current_state, RealityState::Collapsing))
            .count() as f32 / self.reality_fragments.len() as f32;
            
        let transcendent_ratio = self.reality_fragments.values()
            .filter(|f| matches!(f.current_state, RealityState::Transcendent))
            .count() as f32 / self.reality_fragments.len() as f32;
            
        state.round >= 100 || collapse_ratio > 0.9 || transcendent_ratio > 0.9 || self.reality_stability < 0.1
    }
    
    async fn calculate_final_result(&self, state: &GameState) -> GameResult {
        let winner = state.scores.iter()
            .max_by_key(|(_, &score)| score)
            .map(|(player, _)| player.clone())
            .unwrap_or_default();
            
        let final_stability = self.calculate_global_stability();
        let glitch_rate = self.glitches.len() as f32 / state.round as f32;
        
        GameResult {
            game_id: state.game_id,
            winner,
            final_scores: state.scores.clone(),
            total_rounds: state.round,
            duration_ms: 0,
            emergence_events: vec![],
            analytics: GameAnalytics {
                collective_coordination_score: final_stability,
                decision_diversity_index: 1.0 - final_stability,
                strategic_depth: 0.5,
                emergence_frequency: glitch_rate,
                performance_differential: 0.0,
                custom_metrics: {
                    let mut metrics = HashMap::new();
                    metrics.insert("final_reality_stability".to_string(), final_stability);
                    metrics.insert("total_glitches".to_string(), self.glitches.len() as f32);
                    metrics.insert("reality_fragments".to_string(), self.reality_fragments.len() as f32);
                    metrics
                },
            },
        }
    }
    
    async fn get_valid_actions(&self, _state: &GameState, _player_id: &str) -> Vec<String> {
        vec![
            "believe".to_string(),
            "disbelieve".to_string(),
            "propose_reality".to_string(),
            "strengthen_belief".to_string(),
            "question_reality".to_string(),
        ]
    }
    
    async fn get_visualization_data(&self, _state: &GameState) -> serde_json::Value {
        serde_json::json!({
            "reality_fragments": self.reality_fragments,
            "glitches": self.glitches,
            "global_stability": self.reality_stability,
            "consensus_threshold": self.consensus_threshold,
        })
    }
}