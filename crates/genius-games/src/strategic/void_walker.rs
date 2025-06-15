//! Void Walker - Creating something from nothing through consensus

use genius_core::{
    Game, GameConfig, GameState, GameType, PlayerAction, RoundResult,
    RoundOutcome, GameResult, GameAnalytics, GameEvent, EmergenceEvent, EmergenceType,
    Result, GameError,
};
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};

/// A rule proposal for the void
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RuleProposal {
    pub id: String,
    pub proposer: String,
    pub rule_text: String,
    pub category: RuleCategory,
    pub votes: HashMap<String, bool>,
    pub round_proposed: u32,
    pub is_active: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum RuleCategory {
    Physics,
    Resource,
    Interaction,
    Winning,
    Meta,
}

/// Resources that can be created through rules
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VoidResource {
    pub name: String,
    pub amount: i32,
    pub created_by_rule: String,
}

/// Player state in the void
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VoidWalker {
    pub resources: HashMap<String, i32>,
    pub proposed_rules: Vec<String>,
    pub void_energy: f32,
    pub enlightenment_level: u32,
}

/// Action in the void
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VoidAction {
    pub action_type: VoidActionType,
    pub rule_proposal: Option<RuleProposal>,
    pub vote_on_rule: Option<(String, bool)>,
    pub resource_action: Option<ResourceAction>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum VoidActionType {
    ProposeRule,
    VoteOnRule,
    UseResource,
    Meditate,
    CreateFromVoid,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceAction {
    pub resource_name: String,
    pub action: String,
    pub amount: i32,
}

pub struct VoidWalkerGame {
    round_number: u32,
    active_rules: Vec<RuleProposal>,
    pending_rules: Vec<RuleProposal>,
    void_walkers: HashMap<String, VoidWalker>,
    void_stability: f32,
    consensus_threshold: f32,
    reality_complexity: f32,
    resource_types: HashSet<String>,
}

impl VoidWalkerGame {
    pub fn new() -> Self {
        Self {
            round_number: 0,
            active_rules: Vec::new(),
            pending_rules: Vec::new(),
            void_walkers: HashMap::new(),
            void_stability: 0.0, // Complete chaos initially
            consensus_threshold: 0.66,
            reality_complexity: 0.0,
            resource_types: HashSet::new(),
        }
    }
    
    fn initialize_void_walkers(&mut self, player_ids: Vec<String>) {
        for player_id in player_ids {
            self.void_walkers.insert(player_id, VoidWalker {
                resources: HashMap::new(),
                proposed_rules: Vec::new(),
                void_energy: 100.0,
                enlightenment_level: 0,
            });
        }
    }
    
    fn process_rule_proposal(&mut self, proposer: &str, rule: RuleProposal) -> GameEvent {
        let rule_id = format!("rule_{}", self.round_number);
        let mut new_rule = rule;
        new_rule.id = rule_id.clone();
        new_rule.proposer = proposer.to_string();
        new_rule.round_proposed = self.round_number;
        
        self.pending_rules.push(new_rule);
        
        if let Some(walker) = self.void_walkers.get_mut(proposer) {
            walker.proposed_rules.push(rule_id.clone());
            walker.void_energy -= 10.0; // Costs energy to propose
        }
        
        GameEvent {
            event_type: "rule_proposed".to_string(),
            description: format!("{} proposed: {}", proposer, rule.rule_text),
            affected_players: vec![proposer.to_string()],
            data: serde_json::json!({
                "rule_id": rule_id,
                "category": rule.category,
            }),
        }
    }
    
    fn process_rule_votes(&mut self) -> Vec<GameEvent> {
        let mut events = Vec::new();
        let total_walkers = self.void_walkers.len() as f32;
        
        let mut activated_rules = Vec::new();
        
        for rule in &mut self.pending_rules {
            let yes_votes = rule.votes.values().filter(|&&v| v).count() as f32;
            let vote_ratio = yes_votes / total_walkers;
            
            if vote_ratio >= self.consensus_threshold {
                rule.is_active = true;
                activated_rules.push(rule.clone());
                
                // Apply rule effects
                match rule.category {
                    RuleCategory::Resource => {
                        // Extract resource name from rule text
                        if let Some(resource_name) = Self::extract_resource_name(&rule.rule_text) {
                            self.resource_types.insert(resource_name.clone());
                            
                            // Give initial resources to all players
                            for walker in self.void_walkers.values_mut() {
                                walker.resources.insert(resource_name.clone(), 10);
                            }
                        }
                    }
                    RuleCategory::Physics => {
                        self.reality_complexity += 0.1;
                    }
                    RuleCategory::Interaction => {
                        self.void_stability += 0.05;
                    }
                    _ => {}
                }
                
                events.push(GameEvent {
                    event_type: "rule_activated".to_string(),
                    description: format!("Rule activated: {}", rule.rule_text),
                    affected_players: self.void_walkers.keys().cloned().collect(),
                    data: serde_json::json!({
                        "rule_id": rule.id,
                        "vote_ratio": vote_ratio,
                    }),
                });
            }
        }
        
        // Move activated rules to active list
        for rule in activated_rules {
            self.active_rules.push(rule);
        }
        
        // Clear pending rules that were voted on
        self.pending_rules.retain(|r| !r.is_active && r.round_proposed == self.round_number);
        
        events
    }
    
    fn extract_resource_name(rule_text: &str) -> Option<String> {
        // Simple extraction: look for "create <resource>"
        if rule_text.to_lowercase().contains("create") {
            let words: Vec<&str> = rule_text.split_whitespace().collect();
            if let Some(pos) = words.iter().position(|&w| w.to_lowercase() == "create") {
                if pos + 1 < words.len() {
                    return Some(words[pos + 1].to_string());
                }
            }
        }
        None
    }
    
    fn calculate_void_stability(&self) -> f32 {
        let rule_complexity = self.active_rules.len() as f32 / 10.0;
        let resource_diversity = self.resource_types.len() as f32 / 5.0;
        let walker_coherence = self.calculate_walker_coherence();
        
        (rule_complexity + resource_diversity + walker_coherence) / 3.0
    }
    
    fn calculate_walker_coherence(&self) -> f32 {
        if self.void_walkers.len() < 2 {
            return 0.0;
        }
        
        // Check how similar walkers' resources are
        let mut coherence_sum = 0.0;
        let mut comparison_count = 0;
        
        let walkers: Vec<_> = self.void_walkers.values().collect();
        
        for i in 0..walkers.len() {
            for j in i+1..walkers.len() {
                let shared_resources = walkers[i].resources.keys()
                    .filter(|k| walkers[j].resources.contains_key(*k))
                    .count();
                    
                let total_resources = walkers[i].resources.len()
                    .max(walkers[j].resources.len());
                    
                if total_resources > 0 {
                    coherence_sum += shared_resources as f32 / total_resources as f32;
                    comparison_count += 1;
                }
            }
        }
        
        if comparison_count > 0 {
            coherence_sum / comparison_count as f32
        } else {
            0.0
        }
    }
    
    fn detect_reality_emergence(&self) -> Option<EmergenceEvent> {
        let stability = self.calculate_void_stability();
        
        if stability > 0.8 && self.active_rules.len() >= 5 {
            Some(EmergenceEvent {
                round: self.round_number,
                event_type: EmergenceType::Custom("RealityEmerged".to_string()),
                description: "A coherent reality has emerged from the void!".to_string(),
                emergence_score: stability,
                involved_players: self.void_walkers.keys().cloned().collect(),
            })
        } else {
            None
        }
    }
}

impl Default for VoidWalkerGame {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl Game for VoidWalkerGame {
    async fn initialize(&mut self, config: GameConfig) -> Result<GameState> {
        let mut state = GameState::new(config.game_type.clone());
        
        // Initialize void walkers
        let player_ids: Vec<String> = config.initial_players.iter()
            .map(|p| p.id.to_string())
            .collect();
        self.initialize_void_walkers(player_ids);
        
        state.metadata.insert(
            "description".to_string(),
            serde_json::json!("Create reality from nothing through collective agreement"),
        );
        state.metadata.insert(
            "void_stability".to_string(),
            serde_json::json!(self.void_stability),
        );
        state.metadata.insert(
            "rule_count".to_string(),
            serde_json::json!(0),
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
        
        // Process void actions
        for (player_id, action) in &actions {
            if let Ok(void_action) = serde_json::from_value::<VoidAction>(action.data.clone()) {
                match void_action.action_type {
                    VoidActionType::ProposeRule => {
                        if let Some(rule) = void_action.rule_proposal {
                            let event = self.process_rule_proposal(player_id, rule);
                            events.push(event);
                            scores_delta.insert(player_id.clone(), 5);
                        }
                    }
                    VoidActionType::VoteOnRule => {
                        if let Some((rule_id, vote)) = void_action.vote_on_rule {
                            // Find the rule and record vote
                            for rule in &mut self.pending_rules {
                                if rule.id == rule_id {
                                    rule.votes.insert(player_id.clone(), vote);
                                    scores_delta.insert(player_id.clone(), 2);
                                    break;
                                }
                            }
                        }
                    }
                    VoidActionType::Meditate => {
                        if let Some(walker) = self.void_walkers.get_mut(player_id) {
                            walker.void_energy += 20.0;
                            walker.enlightenment_level += 1;
                            scores_delta.insert(player_id.clone(), 3);
                            
                            events.push(GameEvent {
                                event_type: "meditation".to_string(),
                                description: format!("{} meditated in the void", player_id),
                                affected_players: vec![player_id.clone()],
                                data: serde_json::json!({
                                    "enlightenment": walker.enlightenment_level,
                                }),
                            });
                        }
                    }
                    VoidActionType::UseResource => {
                        if let Some(resource_action) = void_action.resource_action {
                            if let Some(walker) = self.void_walkers.get_mut(player_id) {
                                if let Some(amount) = walker.resources.get_mut(&resource_action.resource_name) {
                                    *amount = (*amount as i32 + resource_action.amount).max(0) as i32;
                                    scores_delta.insert(player_id.clone(), resource_action.amount.abs());
                                }
                            }
                        }
                    }
                    VoidActionType::CreateFromVoid => {
                        if let Some(walker) = self.void_walkers.get_mut(player_id) {
                            if walker.void_energy >= 50.0 && walker.enlightenment_level >= 3 {
                                walker.void_energy -= 50.0;
                                scores_delta.insert(player_id.clone(), 20);
                                
                                events.push(GameEvent {
                                    event_type: "void_creation".to_string(),
                                    description: format!("{} created something from nothing!", player_id),
                                    affected_players: vec![player_id.clone()],
                                    data: serde_json::json!({
                                        "creation_power": walker.enlightenment_level,
                                    }),
                                });
                            }
                        }
                    }
                }
            }
        }
        
        // Process rule votes at end of round
        let mut vote_events = self.process_rule_votes();
        events.append(&mut vote_events);
        
        // Update void stability
        self.void_stability = self.calculate_void_stability();
        
        // Check for reality emergence
        let mut emergence_detected = false;
        if let Some(emergence) = self.detect_reality_emergence() {
            emergence_detected = true;
            events.push(GameEvent {
                event_type: "emergence".to_string(),
                description: emergence.description.clone(),
                affected_players: emergence.involved_players.clone(),
                data: serde_json::json!({
                    "emergence_type": "reality_emerged",
                    "stability": self.void_stability,
                }),
            });
            
            // Bonus for all players
            for player_id in self.void_walkers.keys() {
                *scores_delta.entry(player_id.clone()).or_insert(0) += 50;
            }
        }
        
        let outcome = RoundOutcome {
            winners: scores_delta.iter()
                .filter(|(_, &score)| score > 10)
                .map(|(player, _)| player.clone())
                .collect(),
            losers: vec![],
            special_events: events.iter().map(|e| e.description.clone()).collect(),
            emergence_detected,
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
        // Game ends when reality is stable or after max rounds
        state.round >= 50 || self.void_stability > 0.95
    }
    
    async fn calculate_final_result(&self, state: &GameState) -> GameResult {
        // Winner is the player who contributed most to reality creation
        let mut contribution_scores = HashMap::new();
        
        for (player_id, walker) in &self.void_walkers {
            let rule_score = walker.proposed_rules.len() as i32 * 10;
            let enlightenment_score = walker.enlightenment_level as i32 * 5;
            let resource_score: i32 = walker.resources.values().sum();
            
            contribution_scores.insert(
                player_id.clone(),
                rule_score + enlightenment_score + resource_score,
            );
        }
        
        let winner = contribution_scores.iter()
            .max_by_key(|(_, &score)| score)
            .map(|(player, _)| player.clone())
            .unwrap_or_default();
            
        GameResult {
            game_id: state.game_id,
            winner,
            final_scores: state.scores.clone(),
            total_rounds: state.round,
            duration_ms: 0,
            emergence_events: vec![],
            analytics: GameAnalytics {
                collective_coordination_score: self.void_stability,
                decision_diversity_index: self.resource_types.len() as f32 / 10.0,
                strategic_depth: self.reality_complexity,
                emergence_frequency: 0.0,
                performance_differential: 0.0,
                custom_metrics: {
                    let mut metrics = HashMap::new();
                    metrics.insert("final_stability".to_string(), self.void_stability);
                    metrics.insert("rules_created".to_string(), self.active_rules.len() as f32);
                    metrics.insert("resource_types".to_string(), self.resource_types.len() as f32);
                    metrics
                },
            },
        }
    }
    
    async fn get_valid_actions(&self, _state: &GameState, _player_id: &str) -> Vec<String> {
        vec![
            "propose_rule".to_string(),
            "vote_on_rule".to_string(),
            "use_resource".to_string(),
            "meditate".to_string(),
            "create_from_void".to_string(),
        ]
    }
    
    async fn get_visualization_data(&self, _state: &GameState) -> serde_json::Value {
        serde_json::json!({
            "void_stability": self.void_stability,
            "active_rules": self.active_rules,
            "pending_rules": self.pending_rules,
            "void_walkers": self.void_walkers,
            "reality_complexity": self.reality_complexity,
        })
    }
}