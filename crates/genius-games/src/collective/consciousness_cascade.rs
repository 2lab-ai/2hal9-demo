//! Consciousness Cascade - Consciousness flows and amplifies through the network

use genius_core::{
    Game, GameConfig, GameState, GameType, PlayerAction, RoundResult,
    RoundOutcome, GameResult, GameAnalytics, GameEvent, EmergenceEvent, EmergenceType,
    Result, GameError,
};
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};

/// A consciousness node in the network
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConsciousnessNode {
    pub player_id: String,
    pub consciousness_level: f32,
    pub connections: HashMap<String, Connection>,
    pub thoughts: Vec<Thought>,
    pub resonance_frequency: f32,
    pub cascade_potential: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Connection {
    pub strength: f32,
    pub flow_rate: f32,
    pub bidirectional: bool,
    pub resonance_match: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Thought {
    pub content: String,
    pub intensity: f32,
    pub propagation_count: u32,
    pub origin_player: String,
    pub mutations: Vec<String>,
}

/// Actions in the consciousness cascade
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CascadeAction {
    pub action_type: CascadeActionType,
    pub target_players: Option<Vec<String>>,
    pub thought_content: Option<String>,
    pub frequency_adjustment: Option<f32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CascadeActionType {
    SendThought,
    OpenChannel,
    Resonate,
    Amplify,
    Block,
    Merge,
}

pub struct ConsciousnessCascadeGame {
    round_number: u32,
    nodes: HashMap<String, ConsciousnessNode>,
    global_consciousness: f32,
    cascade_threshold: f32,
    thought_pool: Vec<Thought>,
    resonance_clusters: Vec<HashSet<String>>,
    cascade_events: Vec<CascadeEvent>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct CascadeEvent {
    round: u32,
    cascade_size: usize,
    peak_intensity: f32,
    affected_players: Vec<String>,
}

impl ConsciousnessCascadeGame {
    pub fn new() -> Self {
        Self {
            round_number: 0,
            nodes: HashMap::new(),
            global_consciousness: 0.0,
            cascade_threshold: 0.7,
            thought_pool: Vec::new(),
            resonance_clusters: Vec::new(),
            cascade_events: Vec::new(),
        }
    }
    
    fn initialize_consciousness_network(&mut self, player_ids: Vec<String>) {
        for player_id in player_ids {
            let node = ConsciousnessNode {
                player_id: player_id.clone(),
                consciousness_level: 0.5,
                connections: HashMap::new(),
                thoughts: Vec::new(),
                resonance_frequency: rand::random::<f32>() * 10.0,
                cascade_potential: 0.3,
            };
            
            self.nodes.insert(player_id, node);
        }
        
        // Initialize some random connections
        let player_list: Vec<_> = self.nodes.keys().cloned().collect();
        for i in 0..player_list.len() {
            for j in i+1..player_list.len() {
                if rand::random::<f32>() < 0.3 {
                    self.create_connection(&player_list[i], &player_list[j], 0.5);
                }
            }
        }
    }
    
    fn create_connection(&mut self, player1: &str, player2: &str, strength: f32) {
        if let (Some(node1), Some(node2)) = (self.nodes.get(player1), self.nodes.get(player2)) {
            let freq_diff = (node1.resonance_frequency - node2.resonance_frequency).abs();
            let resonance_match = 1.0 / (1.0 + freq_diff);
            
            let connection = Connection {
                strength,
                flow_rate: strength * resonance_match,
                bidirectional: true,
                resonance_match,
            };
            
            if let Some(node1) = self.nodes.get_mut(player1) {
                node1.connections.insert(player2.to_string(), connection.clone());
            }
            if let Some(node2) = self.nodes.get_mut(player2) {
                node2.connections.insert(player1.to_string(), connection);
            }
        }
    }
    
    fn propagate_thought(
        &mut self,
        origin: &str,
        thought: Thought,
        visited: &mut HashSet<String>
    ) -> Vec<GameEvent> {
        let mut events = Vec::new();
        visited.insert(origin.to_string());
        
        if let Some(node) = self.nodes.get_mut(origin) {
            // Add thought to node
            node.thoughts.push(thought.clone());
            
            // Increase consciousness from receiving thought
            node.consciousness_level = (node.consciousness_level + thought.intensity * 0.1).min(1.0);
            
            // Propagate to connected nodes
            let connections: Vec<_> = node.connections.iter()
                .filter(|(target, _)| !visited.contains(*target))
                .map(|(t, c)| (t.clone(), c.clone()))
                .collect();
                
            for (target, connection) in connections {
                if connection.flow_rate > rand::random::<f32>() {
                    // Thought mutates as it propagates
                    let mut propagated_thought = thought.clone();
                    propagated_thought.intensity *= connection.strength;
                    propagated_thought.propagation_count += 1;
                    
                    if propagated_thought.intensity > 0.1 {
                        // Add mutation based on target's frequency
                        if let Some(target_node) = self.nodes.get(&target) {
                            let mutation = format!("{}:{:.1}", target, target_node.resonance_frequency);
                            propagated_thought.mutations.push(mutation);
                        }
                        
                        // Recursive propagation
                        let mut sub_events = self.propagate_thought(&target, propagated_thought, visited);
                        events.append(&mut sub_events);
                    }
                }
            }
        }
        
        events
    }
    
    fn detect_cascade(&mut self) -> Option<Vec<GameEvent>> {
        let mut events = Vec::new();
        let mut cascade_detected = false;
        
        // Check each node for cascade potential
        for (player_id, node) in &self.nodes {
            if node.consciousness_level > self.cascade_threshold && 
               node.thoughts.len() > 3 {
                // Cascade triggered!
                cascade_detected = true;
                
                let cascade_size = node.connections.len();
                let cascade_event = CascadeEvent {
                    round: self.round_number,
                    cascade_size,
                    peak_intensity: node.consciousness_level,
                    affected_players: node.connections.keys().cloned().collect(),
                };
                
                self.cascade_events.push(cascade_event.clone());
                
                events.push(GameEvent {
                    event_type: "consciousness_cascade".to_string(),
                    description: format!("{} triggered a consciousness cascade affecting {} nodes!", 
                        player_id, cascade_size),
                    affected_players: cascade_event.affected_players.clone(),
                    data: serde_json::json!({
                        "origin": player_id,
                        "cascade_size": cascade_size,
                        "intensity": node.consciousness_level,
                    }),
                });
                
                // Boost connected nodes
                for connected_id in node.connections.keys() {
                    if let Some(connected_node) = self.nodes.get_mut(connected_id) {
                        connected_node.consciousness_level = 
                            (connected_node.consciousness_level * 1.5).min(1.0);
                        connected_node.cascade_potential += 0.1;
                    }
                }
            }
        }
        
        if cascade_detected {
            self.update_global_consciousness();
            Some(events)
        } else {
            None
        }
    }
    
    fn update_global_consciousness(&mut self) {
        let total_consciousness: f32 = self.nodes.values()
            .map(|n| n.consciousness_level)
            .sum();
            
        self.global_consciousness = total_consciousness / self.nodes.len() as f32;
    }
    
    fn find_resonance_clusters(&mut self) {
        self.resonance_clusters.clear();
        let mut visited = HashSet::new();
        
        for player_id in self.nodes.keys() {
            if !visited.contains(player_id) {
                let mut cluster = HashSet::new();
                self.explore_resonance_cluster(player_id, &mut cluster, &mut visited, 0.5);
                
                if cluster.len() > 1 {
                    self.resonance_clusters.push(cluster);
                }
            }
        }
    }
    
    fn explore_resonance_cluster(
        &self,
        player_id: &str,
        cluster: &mut HashSet<String>,
        visited: &mut HashSet<String>,
        min_resonance: f32
    ) {
        visited.insert(player_id.to_string());
        cluster.insert(player_id.to_string());
        
        if let Some(node) = self.nodes.get(player_id) {
            for (connected_id, connection) in &node.connections {
                if !visited.contains(connected_id) && connection.resonance_match >= min_resonance {
                    self.explore_resonance_cluster(connected_id, cluster, visited, min_resonance);
                }
            }
        }
    }
    
    fn detect_global_emergence(&self) -> Option<EmergenceEvent> {
        // Check for global consciousness emergence
        if self.global_consciousness > 0.8 && self.cascade_events.len() > 3 {
            let total_cascade_size: usize = self.cascade_events.iter()
                .map(|e| e.cascade_size)
                .sum();
                
            Some(EmergenceEvent {
                round: self.round_number,
                event_type: EmergenceType::Custom("GlobalConsciousness".to_string()),
                description: "Global consciousness achieved through cascading awareness!".to_string(),
                emergence_score: self.global_consciousness,
                involved_players: self.nodes.keys().cloned().collect(),
            })
        } else if self.resonance_clusters.iter().any(|c| c.len() > self.nodes.len() / 2) {
            Some(EmergenceEvent {
                round: self.round_number,
                event_type: EmergenceType::Custom("ResonanceUnity".to_string()),
                description: "Majority of nodes achieved resonance unity!".to_string(),
                emergence_score: 0.9,
                involved_players: self.nodes.keys().cloned().collect(),
            })
        } else {
            None
        }
    }
}

impl Default for ConsciousnessCascadeGame {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl Game for ConsciousnessCascadeGame {
    async fn initialize(&mut self, config: GameConfig) -> Result<GameState> {
        let mut state = GameState::new(config.game_type.clone());
        
        let player_ids: Vec<String> = config.initial_players.iter()
            .map(|p| p.id.to_string())
            .collect();
        self.initialize_consciousness_network(player_ids);
        
        state.metadata.insert(
            "description".to_string(),
            serde_json::json!("Consciousness flows and cascades through the network of minds"),
        );
        state.metadata.insert(
            "global_consciousness".to_string(),
            serde_json::json!(0.0),
        );
        state.metadata.insert(
            "cascade_threshold".to_string(),
            serde_json::json!(self.cascade_threshold),
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
        
        // Process player actions
        for (player_id, action) in &actions {
            if let Ok(cascade_action) = serde_json::from_value::<CascadeAction>(action.data.clone()) {
                match cascade_action.action_type {
                    CascadeActionType::SendThought => {
                        if let Some(content) = cascade_action.thought_content {
                            let thought = Thought {
                                content,
                                intensity: 0.7,
                                propagation_count: 0,
                                origin_player: player_id.clone(),
                                mutations: Vec::new(),
                            };
                            
                            let mut visited = HashSet::new();
                            let mut prop_events = self.propagate_thought(player_id, thought, &mut visited);
                            events.append(&mut prop_events);
                            
                            scores_delta.insert(player_id.clone(), visited.len() as i32 * 5);
                            
                            events.push(GameEvent {
                                event_type: "thought_sent".to_string(),
                                description: format!("{} sent a thought through the network", player_id),
                                affected_players: visited.into_iter().collect(),
                                data: serde_json::json!({
                                    "origin": player_id,
                                    "content": content,
                                }),
                            });
                        }
                    }
                    CascadeActionType::OpenChannel => {
                        if let Some(targets) = cascade_action.target_players {
                            for target in targets {
                                self.create_connection(player_id, &target, 0.7);
                                scores_delta.insert(player_id.clone(), 3);
                            }
                        }
                    }
                    CascadeActionType::Resonate => {
                        if let Some(freq_adj) = cascade_action.frequency_adjustment {
                            if let Some(node) = self.nodes.get_mut(player_id) {
                                node.resonance_frequency += freq_adj;
                                node.resonance_frequency = node.resonance_frequency.clamp(0.0, 10.0);
                                
                                // Update resonance matches
                                for (_, connection) in node.connections.iter_mut() {
                                    // Recalculate resonance (simplified)
                                    connection.resonance_match *= 1.1;
                                    connection.resonance_match = connection.resonance_match.min(1.0);
                                }
                                
                                scores_delta.insert(player_id.clone(), 5);
                            }
                        }
                    }
                    CascadeActionType::Amplify => {
                        if let Some(node) = self.nodes.get_mut(player_id) {
                            // Amplify all current thoughts
                            for thought in &mut node.thoughts {
                                thought.intensity *= 1.5;
                                thought.intensity = thought.intensity.min(1.0);
                            }
                            node.cascade_potential += 0.1;
                            scores_delta.insert(player_id.clone(), 8);
                        }
                    }
                    CascadeActionType::Block => {
                        // Temporarily reduce incoming connections
                        if let Some(node) = self.nodes.get_mut(player_id) {
                            for connection in node.connections.values_mut() {
                                connection.flow_rate *= 0.5;
                            }
                            scores_delta.insert(player_id.clone(), -2);
                        }
                    }
                    CascadeActionType::Merge => {
                        // Merge consciousness with connected nodes
                        if let Some(node) = self.nodes.get(player_id) {
                            let avg_consciousness = node.connections.keys()
                                .filter_map(|id| self.nodes.get(id))
                                .map(|n| n.consciousness_level)
                                .sum::<f32>() / node.connections.len().max(1) as f32;
                                
                            if let Some(node) = self.nodes.get_mut(player_id) {
                                node.consciousness_level = 
                                    (node.consciousness_level + avg_consciousness) / 2.0;
                                scores_delta.insert(player_id.clone(), 10);
                            }
                        }
                    }
                }
            }
        }
        
        // Detect cascades
        if let Some(mut cascade_events) = self.detect_cascade() {
            events.append(&mut cascade_events);
            
            // Bonus points for cascade participants
            for event in &cascade_events {
                if let Some(players) = event.data.get("affected_players").and_then(|v| v.as_array()) {
                    for player in players {
                        if let Some(player_id) = player.as_str() {
                            *scores_delta.entry(player_id.to_string()).or_insert(0) += 20;
                        }
                    }
                }
            }
        }
        
        // Find resonance clusters
        self.find_resonance_clusters();
        
        // Award points for being in large clusters
        for cluster in &self.resonance_clusters {
            if cluster.len() >= 3 {
                for player_id in cluster {
                    *scores_delta.entry(player_id.clone()).or_insert(0) += cluster.len() as i32;
                }
            }
        }
        
        // Update global consciousness
        self.update_global_consciousness();
        
        // Check for global emergence
        let mut emergence_detected = false;
        if let Some(emergence) = self.detect_global_emergence() {
            emergence_detected = true;
            events.push(GameEvent {
                event_type: "emergence".to_string(),
                description: emergence.description.clone(),
                affected_players: emergence.involved_players.clone(),
                data: serde_json::json!({
                    "emergence_type": "global_consciousness",
                    "score": self.global_consciousness,
                }),
            });
            
            // Massive bonus for achieving global consciousness
            for player_id in self.nodes.keys() {
                *scores_delta.entry(player_id.clone()).or_insert(0) += 100;
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
        state.round >= 50 || self.global_consciousness > 0.95
    }
    
    async fn calculate_final_result(&self, state: &GameState) -> GameResult {
        let winner = state.scores.iter()
            .max_by_key(|(_, &score)| score)
            .map(|(player, _)| player.clone())
            .unwrap_or_default();
            
        let total_thoughts = self.nodes.values()
            .map(|n| n.thoughts.len())
            .sum::<usize>();
            
        let avg_connections = self.nodes.values()
            .map(|n| n.connections.len() as f32)
            .sum::<f32>() / self.nodes.len() as f32;
            
        GameResult {
            game_id: state.game_id,
            winner,
            final_scores: state.scores.clone(),
            total_rounds: state.round,
            duration_ms: 0,
            emergence_events: vec![],
            analytics: GameAnalytics {
                collective_coordination_score: self.global_consciousness,
                decision_diversity_index: 0.6,
                strategic_depth: avg_connections / 10.0,
                emergence_frequency: self.cascade_events.len() as f32 / state.round as f32,
                performance_differential: 0.0,
                custom_metrics: {
                    let mut metrics = HashMap::new();
                    metrics.insert("global_consciousness".to_string(), self.global_consciousness);
                    metrics.insert("total_cascades".to_string(), self.cascade_events.len() as f32);
                    metrics.insert("total_thoughts".to_string(), total_thoughts as f32);
                    metrics.insert("avg_connections".to_string(), avg_connections);
                    metrics.insert("cluster_count".to_string(), self.resonance_clusters.len() as f32);
                    metrics
                },
            },
        }
    }
    
    async fn get_valid_actions(&self, _state: &GameState, _player_id: &str) -> Vec<String> {
        vec![
            "send_thought".to_string(),
            "open_channel".to_string(),
            "resonate".to_string(),
            "amplify".to_string(),
            "block".to_string(),
            "merge".to_string(),
        ]
    }
    
    async fn get_visualization_data(&self, _state: &GameState) -> serde_json::Value {
        serde_json::json!({
            "nodes": self.nodes,
            "global_consciousness": self.global_consciousness,
            "cascade_events": self.cascade_events,
            "resonance_clusters": self.resonance_clusters,
            "thought_count": self.thought_pool.len(),
        })
    }
}