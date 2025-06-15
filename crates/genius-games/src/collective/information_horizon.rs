//! Information Horizon - Knowledge boundaries and emergence from partial information

use genius_core::{
    Game, GameConfig, GameState, GameType, PlayerAction, RoundResult,
    RoundOutcome, GameResult, GameAnalytics, GameEvent, EmergenceEvent, EmergenceType,
    Result, GameError,
};
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use rand::Rng;

/// Information fragment that decays/transforms when shared
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InfoFragment {
    pub id: String,
    pub content: String,
    pub fidelity: f32, // 1.0 = perfect, 0.0 = completely corrupted
    pub age: u32,
    pub shared_count: u32,
    pub category: InfoCategory,
    pub source_player: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum InfoCategory {
    Pattern,
    Number,
    Location,
    Relationship,
    Meta,
}

/// Player's information state
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InfoAgent {
    pub known_fragments: HashMap<String, InfoFragment>,
    pub bandwidth: u32, // How many fragments can share per round
    pub processing_power: f32, // Ability to reconstruct degraded info
    pub trust_network: HashMap<String, f32>, // Trust in other players
    pub reconstruction_attempts: u32,
}

/// Information sharing action
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InfoAction {
    pub action_type: InfoActionType,
    pub share_with: Option<Vec<(String, String)>>, // (player_id, fragment_id)
    pub reconstruct: Option<Vec<String>>, // fragment_ids to reconstruct
    pub combine: Option<Vec<String>>, // fragment_ids to combine
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum InfoActionType {
    Share,
    Reconstruct,
    Combine,
    Analyze,
    TrustUpdate,
}

pub struct InformationHorizonGame {
    round_number: u32,
    info_agents: HashMap<String, InfoAgent>,
    global_fragments: HashMap<String, InfoFragment>,
    hidden_pattern: String,
    decay_rate: f32,
    noise_rate: f32,
    emergence_threshold: f32,
    collective_knowledge: f32,
}

impl InformationHorizonGame {
    pub fn new() -> Self {
        Self {
            round_number: 0,
            info_agents: HashMap::new(),
            global_fragments: HashMap::new(),
            hidden_pattern: Self::generate_hidden_pattern(),
            decay_rate: 0.1,
            noise_rate: 0.05,
            emergence_threshold: 0.8,
            collective_knowledge: 0.0,
        }
    }
    
    fn generate_hidden_pattern() -> String {
        // Create a complex pattern that requires multiple fragments to understand
        "PRIME_SEQUENCE_2_3_5_7_11_13_17_19_23_29".to_string()
    }
    
    fn initialize_info_distribution(&mut self, player_ids: Vec<String>) {
        let pattern_parts = self.hidden_pattern.split('_').collect::<Vec<_>>();
        let mut fragment_id = 0;
        
        // Distribute fragments of the pattern to different players
        for (idx, player_id) in player_ids.iter().enumerate() {
            let mut agent = InfoAgent {
                known_fragments: HashMap::new(),
                bandwidth: 3,
                processing_power: 0.7,
                trust_network: HashMap::new(),
                reconstruction_attempts: 0,
            };
            
            // Initialize trust network (start with moderate trust)
            for other_id in &player_ids {
                if other_id != &player_id {
                    agent.trust_network.insert(other_id.clone(), 0.5);
                }
            }
            
            // Give each player partial information
            let start_idx = idx % pattern_parts.len();
            for i in 0..3 {
                let part_idx = (start_idx + i * 3) % pattern_parts.len();
                if part_idx < pattern_parts.len() {
                    let fragment = InfoFragment {
                        id: format!("frag_{}", fragment_id),
                        content: pattern_parts[part_idx].to_string(),
                        fidelity: 0.9, // Start with high fidelity
                        age: 0,
                        shared_count: 0,
                        category: Self::categorize_content(pattern_parts[part_idx]),
                        source_player: player_id.clone(),
                    };
                    
                    agent.known_fragments.insert(fragment.id.clone(), fragment.clone());
                    self.global_fragments.insert(fragment.id.clone(), fragment);
                    fragment_id += 1;
                }
            }
            
            self.info_agents.insert(player_id.clone(), agent);
        }
    }
    
    fn categorize_content(content: &str) -> InfoCategory {
        if content.parse::<i32>().is_ok() {
            InfoCategory::Number
        } else if content == "PRIME" || content == "SEQUENCE" {
            InfoCategory::Pattern
        } else {
            InfoCategory::Relationship
        }
    }
    
    fn process_info_sharing(
        &mut self,
        sender_id: &str,
        receiver_id: &str,
        fragment_id: &str
    ) -> Option<GameEvent> {
        let sender = self.info_agents.get(sender_id)?;
        let fragment = sender.known_fragments.get(fragment_id)?;
        
        // Create degraded copy
        let mut shared_fragment = fragment.clone();
        shared_fragment.fidelity *= 1.0 - self.decay_rate;
        shared_fragment.shared_count += 1;
        shared_fragment.age += 1;
        
        // Add noise based on trust
        let trust = sender.trust_network.get(receiver_id).unwrap_or(&0.5);
        if *trust < 0.7 {
            shared_fragment.fidelity *= 1.0 - self.noise_rate;
        }
        
        // Corrupt content if fidelity is too low
        if shared_fragment.fidelity < 0.5 {
            shared_fragment.content = Self::corrupt_content(&shared_fragment.content, shared_fragment.fidelity);
        }
        
        // Give to receiver
        if let Some(receiver) = self.info_agents.get_mut(receiver_id) {
            receiver.known_fragments.insert(shared_fragment.id.clone(), shared_fragment.clone());
        }
        
        Some(GameEvent {
            event_type: "info_shared".to_string(),
            description: format!("{} shared info with {} (fidelity: {:.2})", 
                sender_id, receiver_id, shared_fragment.fidelity),
            affected_players: vec![sender_id.to_string(), receiver_id.to_string()],
            data: serde_json::json!({
                "fragment_id": fragment_id,
                "fidelity": shared_fragment.fidelity,
                "trust_level": trust,
            }),
        })
    }
    
    fn corrupt_content(content: &str, fidelity: f32) -> String {
        let mut rng = rand::rng();
        let corruption_chance = 1.0 - fidelity;
        
        content.chars().map(|c| {
            if rng.random::<f32>() < corruption_chance {
                if c.is_numeric() {
                    // Replace with random digit
                    char::from_digit(rng.random_range(0..10), 10).unwrap()
                } else if c.is_alphabetic() {
                    // Replace with random letter
                    if c.is_uppercase() {
                        (b'A' + rng.random_range(0..26) as u8) as char
                    } else {
                        (b'a' + rng.random_range(0..26) as u8) as char
                    }
                } else {
                    c // Keep special characters
                }
            } else {
                c
            }
        }).collect()
    }
    
    fn attempt_reconstruction(
        &mut self,
        player_id: &str,
        fragment_ids: &[String]
    ) -> Option<GameEvent> {
        let agent = self.info_agents.get_mut(player_id)?;
        agent.reconstruction_attempts += 1;
        
        let mut combined_content = Vec::new();
        let mut total_fidelity = 0.0;
        
        for frag_id in fragment_ids {
            if let Some(fragment) = agent.known_fragments.get(frag_id) {
                combined_content.push(fragment.content.clone());
                total_fidelity += fragment.fidelity;
            }
        }
        
        if combined_content.is_empty() {
            return None;
        }
        
        let avg_fidelity = total_fidelity / combined_content.len() as f32;
        let reconstruction_success = avg_fidelity * agent.processing_power;
        
        if reconstruction_success > 0.7 {
            // Successful reconstruction
            let reconstructed = combined_content.join("_");
            let similarity = Self::calculate_pattern_similarity(&reconstructed, &self.hidden_pattern);
            
            Some(GameEvent {
                event_type: "reconstruction_attempt".to_string(),
                description: format!("{} reconstructed pattern with {:.0}% accuracy", 
                    player_id, similarity * 100.0),
                affected_players: vec![player_id.to_string()],
                data: serde_json::json!({
                    "accuracy": similarity,
                    "fragments_used": fragment_ids.len(),
                }),
            })
        } else {
            Some(GameEvent {
                event_type: "reconstruction_failed".to_string(),
                description: format!("{} failed to reconstruct pattern", player_id),
                affected_players: vec![player_id.to_string()],
                data: serde_json::json!({
                    "success_rate": reconstruction_success,
                }),
            })
        }
    }
    
    fn calculate_pattern_similarity(reconstructed: &str, original: &str) -> f32 {
        let r_parts: HashSet<_> = reconstructed.split('_').collect();
        let o_parts: HashSet<_> = original.split('_').collect();
        
        let intersection = r_parts.intersection(&o_parts).count();
        let union = r_parts.union(&o_parts).count();
        
        if union > 0 {
            intersection as f32 / union as f32
        } else {
            0.0
        }
    }
    
    fn calculate_collective_knowledge(&self) -> f32 {
        let mut all_known_parts = HashSet::new();
        
        for agent in self.info_agents.values() {
            for fragment in agent.known_fragments.values() {
                if fragment.fidelity > 0.5 {
                    all_known_parts.insert(fragment.content.clone());
                }
            }
        }
        
        let pattern_parts: HashSet<_> = self.hidden_pattern.split('_').collect();
        let known_correct = all_known_parts.iter()
            .filter(|part| pattern_parts.contains(part.as_str()))
            .count();
            
        known_correct as f32 / pattern_parts.len() as f32
    }
    
    fn detect_collective_emergence(&self) -> Option<EmergenceEvent> {
        let knowledge = self.calculate_collective_knowledge();
        
        if knowledge > self.emergence_threshold {
            Some(EmergenceEvent {
                round: self.round_number,
                event_type: EmergenceType::Custom("CollectiveUnderstanding".to_string()),
                description: "Collective has reconstructed the hidden pattern!".to_string(),
                emergence_score: knowledge,
                involved_players: self.info_agents.keys().cloned().collect(),
            })
        } else {
            None
        }
    }
}

impl Default for InformationHorizonGame {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl Game for InformationHorizonGame {
    async fn initialize(&mut self, config: GameConfig) -> Result<GameState> {
        let mut state = GameState::new(config.game_type.clone());
        
        // Initialize information distribution
        let player_ids: Vec<String> = config.initial_players.iter()
            .map(|p| p.id.to_string())
            .collect();
        self.initialize_info_distribution(player_ids);
        
        state.metadata.insert(
            "description".to_string(),
            serde_json::json!("Reconstruct truth from partial, decaying information"),
        );
        state.metadata.insert(
            "decay_rate".to_string(),
            serde_json::json!(self.decay_rate),
        );
        state.metadata.insert(
            "collective_knowledge".to_string(),
            serde_json::json!(0.0),
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
        
        // Age all fragments
        for fragment in self.global_fragments.values_mut() {
            fragment.age += 1;
            fragment.fidelity *= 0.98; // Natural decay
        }
        
        // Process player actions
        for (player_id, action) in &actions {
            if let Ok(info_action) = serde_json::from_value::<InfoAction>(action.data.clone()) {
                match info_action.action_type {
                    InfoActionType::Share => {
                        if let Some(shares) = info_action.share_with {
                            let mut successful_shares = 0;
                            
                            for (receiver_id, fragment_id) in shares {
                                if let Some(event) = self.process_info_sharing(
                                    player_id, 
                                    &receiver_id, 
                                    &fragment_id
                                ) {
                                    events.push(event);
                                    successful_shares += 1;
                                }
                            }
                            
                            scores_delta.insert(player_id.clone(), successful_shares * 5);
                        }
                    }
                    InfoActionType::Reconstruct => {
                        if let Some(fragment_ids) = info_action.reconstruct {
                            if let Some(event) = self.attempt_reconstruction(player_id, &fragment_ids) {
                                let is_success = event.event_type == "reconstruction_attempt";
                                events.push(event);
                                
                                if is_success {
                                    scores_delta.insert(player_id.clone(), 20);
                                } else {
                                    scores_delta.insert(player_id.clone(), -5);
                                }
                            }
                        }
                    }
                    InfoActionType::Combine => {
                        if let Some(fragment_ids) = info_action.combine {
                            if let Some(agent) = self.info_agents.get_mut(player_id) {
                                if fragment_ids.len() >= 2 {
                                    // Combine fragments into new understanding
                                    let mut combined_content = String::new();
                                    let mut total_fidelity = 0.0;
                                    
                                    for frag_id in &fragment_ids {
                                        if let Some(frag) = agent.known_fragments.get(frag_id) {
                                            combined_content.push_str(&frag.content);
                                            combined_content.push('_');
                                            total_fidelity += frag.fidelity;
                                        }
                                    }
                                    
                                    let new_fragment = InfoFragment {
                                        id: format!("combined_{}_{}", player_id, self.round_number),
                                        content: combined_content.trim_end_matches('_').to_string(),
                                        fidelity: total_fidelity / fragment_ids.len() as f32,
                                        age: 0,
                                        shared_count: 0,
                                        category: InfoCategory::Meta,
                                        source_player: player_id.clone(),
                                    };
                                    
                                    agent.known_fragments.insert(new_fragment.id.clone(), new_fragment);
                                    scores_delta.insert(player_id.clone(), 10);
                                }
                            }
                        }
                    }
                    InfoActionType::Analyze => {
                        if let Some(agent) = self.info_agents.get(player_id) {
                            let analysis_score = agent.known_fragments.values()
                                .map(|f| f.fidelity)
                                .sum::<f32>() / agent.known_fragments.len() as f32;
                                
                            scores_delta.insert(player_id.clone(), (analysis_score * 10.0) as i32);
                            
                            events.push(GameEvent {
                                event_type: "analysis".to_string(),
                                description: format!("{} analyzed their information", player_id),
                                affected_players: vec![player_id.clone()],
                                data: serde_json::json!({
                                    "fragment_count": agent.known_fragments.len(),
                                    "avg_fidelity": analysis_score,
                                }),
                            });
                        }
                    }
                    InfoActionType::TrustUpdate => {
                        // Update trust based on information quality received
                        if let Some(agent) = self.info_agents.get_mut(player_id) {
                            for (other_id, trust_delta) in agent.trust_network.clone() {
                                // Simplified trust update
                                agent.trust_network.insert(
                                    other_id,
                                    (trust_delta + 0.1).min(1.0),
                                );
                            }
                        }
                    }
                }
            }
        }
        
        // Update collective knowledge
        self.collective_knowledge = self.calculate_collective_knowledge();
        
        // Check for emergence
        let mut emergence_detected = false;
        if let Some(emergence) = self.detect_collective_emergence() {
            emergence_detected = true;
            events.push(GameEvent {
                event_type: "emergence".to_string(),
                description: emergence.description.clone(),
                affected_players: emergence.involved_players.clone(),
                data: serde_json::json!({
                    "knowledge_level": self.collective_knowledge,
                }),
            });
            
            // Bonus for all players
            for player_id in self.info_agents.keys() {
                *scores_delta.entry(player_id.clone()).or_insert(0) += 50;
            }
        }
        
        let outcome = RoundOutcome {
            winners: scores_delta.iter()
                .filter(|(_, &score)| score > 10)
                .map(|(player, _)| player.clone())
                .collect(),
            losers: scores_delta.iter()
                .filter(|(_, &score)| score < 0)
                .map(|(player, _)| player.clone())
                .collect(),
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
        state.round >= 50 || self.collective_knowledge > 0.95
    }
    
    async fn calculate_final_result(&self, state: &GameState) -> GameResult {
        // Winner has best combination of reconstruction attempts and sharing
        let mut contribution_scores = HashMap::new();
        
        for (player_id, agent) in &self.info_agents {
            let share_score = agent.known_fragments.values()
                .map(|f| f.shared_count as i32)
                .sum::<i32>() * 5;
            let reconstruction_score = agent.reconstruction_attempts as i32 * 10;
            let trust_score = agent.trust_network.values().sum::<f32>() as i32 * 3;
            
            contribution_scores.insert(
                player_id.clone(),
                share_score + reconstruction_score + trust_score,
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
                collective_coordination_score: self.collective_knowledge,
                decision_diversity_index: 0.5,
                strategic_depth: 0.7,
                emergence_frequency: 0.0,
                performance_differential: 0.0,
                custom_metrics: {
                    let mut metrics = HashMap::new();
                    metrics.insert("final_knowledge".to_string(), self.collective_knowledge);
                    metrics.insert("total_fragments".to_string(), self.global_fragments.len() as f32);
                    metrics.insert("avg_fidelity".to_string(), 
                        self.global_fragments.values().map(|f| f.fidelity).sum::<f32>() 
                        / self.global_fragments.len() as f32);
                    metrics
                },
            },
        }
    }
    
    async fn get_valid_actions(&self, _state: &GameState, _player_id: &str) -> Vec<String> {
        vec![
            "share".to_string(),
            "reconstruct".to_string(),
            "combine".to_string(),
            "analyze".to_string(),
            "update_trust".to_string(),
        ]
    }
    
    async fn get_visualization_data(&self, _state: &GameState) -> serde_json::Value {
        serde_json::json!({
            "collective_knowledge": self.collective_knowledge,
            "info_agents": self.info_agents,
            "fragment_count": self.global_fragments.len(),
            "avg_fidelity": self.global_fragments.values()
                .map(|f| f.fidelity).sum::<f32>() / self.global_fragments.len() as f32,
        })
    }
}