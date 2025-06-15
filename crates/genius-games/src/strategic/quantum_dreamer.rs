//! Quantum Dreamer - Dream states collapse and merge across parallel realities

use genius_core::{
    Game, GameConfig, GameState, GameType, PlayerAction, RoundResult,
    RoundOutcome, GameResult, GameAnalytics, GameEvent, EmergenceEvent, EmergenceType,
    Result, GameError,
};
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use rand::Rng;

/// A dream state that exists in quantum superposition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DreamState {
    pub id: String,
    pub reality_level: f32, // 0.0 = pure dream, 1.0 = concrete reality
    pub symbols: Vec<DreamSymbol>,
    pub coherence: f32,
    pub lucidity: f32,
    pub parent_dreamer: String,
    pub shared_dreamers: HashSet<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DreamSymbol {
    pub symbol_type: SymbolType,
    pub meaning: String,
    pub power: f32,
    pub stability: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum SymbolType {
    Door,      // Transitions between states
    Mirror,    // Self-reflection
    Shadow,    // Hidden aspects
    Light,     // Clarity/awareness
    Void,      // Unknown potential
    Key,       // Solutions
    Maze,      // Confusion
    Bridge,    // Connection
}

/// Dreamer's state of consciousness
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Dreamer {
    pub lucidity_level: f32,
    pub dream_power: f32,
    pub reality_anchor: f32,
    pub active_dreams: Vec<String>,
    pub dream_fragments: HashMap<String, f32>, // dream_id -> ownership
    pub symbol_mastery: HashMap<SymbolType, f32>,
}

/// Actions within dreams
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DreamAction {
    pub action_type: DreamActionType,
    pub target_dream: Option<String>,
    pub symbol: Option<SymbolType>,
    pub other_dreamers: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DreamActionType {
    EnterDream,
    CreateSymbol,
    ManipulateDream,
    ShareDream,
    CollapseDream,
    LucidControl,
    DreamWalk,
}

pub struct QuantumDreamerGame {
    round_number: u32,
    dreamers: HashMap<String, Dreamer>,
    dream_states: HashMap<String, DreamState>,
    reality_bleeds: Vec<RealityBleed>,
    collective_unconscious: CollectiveUnconscious,
    lucidity_threshold: f32,
    reality_stability: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct RealityBleed {
    source_dream: String,
    affected_dreams: Vec<String>,
    bleed_strength: f32,
    symbol_leakage: Vec<SymbolType>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct CollectiveUnconscious {
    shared_symbols: HashMap<SymbolType, f32>,
    archetypal_dreams: Vec<String>,
    synchronicity_level: f32,
}

impl QuantumDreamerGame {
    pub fn new() -> Self {
        Self {
            round_number: 0,
            dreamers: HashMap::new(),
            dream_states: HashMap::new(),
            reality_bleeds: Vec::new(),
            collective_unconscious: CollectiveUnconscious {
                shared_symbols: HashMap::new(),
                archetypal_dreams: Vec::new(),
                synchronicity_level: 0.0,
            },
            lucidity_threshold: 0.7,
            reality_stability: 1.0,
        }
    }
    
    fn initialize_dreamscape(&mut self, player_ids: Vec<String>) {
        // Initialize dreamers
        for player_id in &player_ids {
            let mut symbol_mastery = HashMap::new();
            for symbol in &[SymbolType::Door, SymbolType::Mirror, SymbolType::Light] {
                symbol_mastery.insert(symbol.clone(), rand::random::<f32>() * 0.5);
            }
            
            self.dreamers.insert(player_id.clone(), Dreamer {
                lucidity_level: 0.3,
                dream_power: 0.5,
                reality_anchor: 0.8,
                active_dreams: Vec::new(),
                dream_fragments: HashMap::new(),
                symbol_mastery,
            });
        }
        
        // Create initial personal dreams
        for (idx, player_id) in player_ids.iter().enumerate() {
            let dream_id = format!("dream_{}_{}", player_id, idx);
            
            let initial_symbols = vec![
                DreamSymbol {
                    symbol_type: SymbolType::Door,
                    meaning: "Gateway to possibilities".to_string(),
                    power: 0.5,
                    stability: 0.7,
                },
                DreamSymbol {
                    symbol_type: SymbolType::Mirror,
                    meaning: "Self-reflection".to_string(),
                    power: 0.4,
                    stability: 0.8,
                },
            ];
            
            let dream = DreamState {
                id: dream_id.clone(),
                reality_level: 0.2,
                symbols: initial_symbols,
                coherence: 0.6,
                lucidity: 0.3,
                parent_dreamer: player_id.clone(),
                shared_dreamers: HashSet::new(),
            };
            
            self.dream_states.insert(dream_id.clone(), dream);
            
            if let Some(dreamer) = self.dreamers.get_mut(player_id) {
                dreamer.active_dreams.push(dream_id.clone());
                dreamer.dream_fragments.insert(dream_id, 1.0);
            }
        }
    }
    
    fn enter_dream(&mut self, dreamer_id: &str, dream_id: &str) -> Option<GameEvent> {
        let dreamer = self.dreamers.get_mut(dreamer_id)?;
        let dream = self.dream_states.get_mut(dream_id)?;
        
        // Check if dreamer can enter (lucidity check)
        if dreamer.lucidity_level < 0.3 && dream.parent_dreamer != dreamer_id {
            return Some(GameEvent {
                event_type: "dream_blocked".to_string(),
                description: format!("{} lacks lucidity to enter foreign dream", dreamer_id),
                affected_players: vec![dreamer_id.to_string()],
                data: serde_json::json!({
                    "required_lucidity": 0.3,
                    "current_lucidity": dreamer.lucidity_level,
                }),
            });
        }
        
        // Enter the dream
        dream.shared_dreamers.insert(dreamer_id.to_string());
        dreamer.active_dreams.push(dream_id.to_string());
        
        // Gain partial ownership
        dreamer.dream_fragments.insert(dream_id.to_string(), 0.2);
        
        // Dream affects dreamer's consciousness
        dreamer.lucidity_level = (dreamer.lucidity_level + dream.lucidity * 0.1).min(1.0);
        
        Some(GameEvent {
            event_type: "dream_entered".to_string(),
            description: format!("{} entered the dream '{}'", dreamer_id, dream_id),
            affected_players: vec![dreamer_id.to_string(), dream.parent_dreamer.clone()],
            data: serde_json::json!({
                "dream_id": dream_id,
                "new_lucidity": dreamer.lucidity_level,
            }),
        })
    }
    
    fn create_symbol(
        &mut self,
        dreamer_id: &str,
        dream_id: &str,
        symbol_type: SymbolType
    ) -> Option<GameEvent> {
        let dreamer = self.dreamers.get(dreamer_id)?;
        let dream = self.dream_states.get_mut(dream_id)?;
        
        // Check if dreamer has enough power
        let mastery = dreamer.symbol_mastery.get(&symbol_type).unwrap_or(&0.0);
        if dreamer.dream_power < 0.3 || *mastery < 0.2 {
            return None;
        }
        
        // Create symbol with meaning based on context
        let meaning = match symbol_type {
            SymbolType::Door => "Portal to new dream state",
            SymbolType::Mirror => "Reflection of inner truth",
            SymbolType::Shadow => "Hidden aspect revealed",
            SymbolType::Light => "Illumination of consciousness",
            SymbolType::Void => "Pure potential",
            SymbolType::Key => "Solution to dream puzzle",
            SymbolType::Maze => "Complexity and confusion",
            SymbolType::Bridge => "Connection between dreams",
        };
        
        let symbol = DreamSymbol {
            symbol_type: symbol_type.clone(),
            meaning: meaning.to_string(),
            power: dreamer.dream_power * mastery,
            stability: dream.coherence,
        };
        
        dream.symbols.push(symbol.clone());
        
        // Update collective unconscious
        *self.collective_unconscious.shared_symbols
            .entry(symbol_type.clone())
            .or_insert(0.0) += 0.1;
            
        Some(GameEvent {
            event_type: "symbol_created".to_string(),
            description: format!("{} manifested {} in dream", dreamer_id, meaning),
            affected_players: dream.shared_dreamers.iter().cloned().collect(),
            data: serde_json::json!({
                "symbol": symbol_type,
                "power": symbol.power,
                "dream_id": dream_id,
            }),
        })
    }
    
    fn manipulate_dream(&mut self, dreamer_id: &str, dream_id: &str) -> Option<GameEvent> {
        let dreamer = self.dreamers.get_mut(dreamer_id)?;
        let dream = self.dream_states.get_mut(dream_id)?;
        
        // Calculate manipulation power
        let ownership = dreamer.dream_fragments.get(dream_id).unwrap_or(&0.0);
        let power = dreamer.dream_power * ownership * dreamer.lucidity_level;
        
        if power < 0.1 {
            return None;
        }
        
        // Manipulate dream properties
        dream.reality_level = (dream.reality_level + power * 0.2).clamp(0.0, 1.0);
        dream.coherence = (dream.coherence + power * 0.1).clamp(0.0, 1.0);
        
        // Increase dreamer's mastery
        dreamer.lucidity_level = (dreamer.lucidity_level + 0.05).min(1.0);
        
        // Chance of reality bleed
        if dream.reality_level > 0.7 && rand::random::<f32>() < 0.3 {
            let bleed = RealityBleed {
                source_dream: dream_id.to_string(),
                affected_dreams: self.find_nearby_dreams(dream_id),
                bleed_strength: dream.reality_level - 0.7,
                symbol_leakage: dream.symbols.iter()
                    .map(|s| s.symbol_type.clone())
                    .take(2)
                    .collect(),
            };
            
            self.reality_bleeds.push(bleed.clone());
            
            return Some(GameEvent {
                event_type: "reality_bleed".to_string(),
                description: format!("Dream '{}' bleeding into reality!", dream_id),
                affected_players: dream.shared_dreamers.iter().cloned().collect(),
                data: serde_json::json!({
                    "dream_id": dream_id,
                    "reality_level": dream.reality_level,
                    "affected_dreams": bleed.affected_dreams,
                }),
            });
        }
        
        Some(GameEvent {
            event_type: "dream_manipulated".to_string(),
            description: format!("{} shaped the dream fabric", dreamer_id),
            affected_players: vec![dreamer_id.to_string()],
            data: serde_json::json!({
                "dream_id": dream_id,
                "new_reality": dream.reality_level,
                "new_coherence": dream.coherence,
            }),
        })
    }
    
    fn find_nearby_dreams(&self, dream_id: &str) -> Vec<String> {
        // Find dreams that share dreamers
        let dream = match self.dream_states.get(dream_id) {
            Some(d) => d,
            None => return vec![],
        };
        
        let mut nearby = Vec::new();
        for (other_id, other_dream) in &self.dream_states {
            if other_id != dream_id {
                let shared: HashSet<_> = dream.shared_dreamers
                    .intersection(&other_dream.shared_dreamers)
                    .collect();
                if !shared.is_empty() {
                    nearby.push(other_id.clone());
                }
            }
        }
        
        nearby
    }
    
    fn collapse_dream(&mut self, dream_id: &str) -> Option<Vec<GameEvent>> {
        let dream = self.dream_states.get(dream_id)?;
        
        if dream.reality_level < 0.8 {
            return None;
        }
        
        let mut events = Vec::new();
        
        // Dream collapses into reality
        let affected_dreamers: Vec<_> = dream.shared_dreamers.iter().cloned().collect();
        
        for dreamer_id in &affected_dreamers {
            if let Some(dreamer) = self.dreamers.get_mut(dreamer_id) {
                // Reality anchor is affected
                dreamer.reality_anchor *= 0.8;
                
                // But gain power from collapsed dream
                dreamer.dream_power += dream.reality_level * 0.2;
                
                // Learn from dream symbols
                for symbol in &dream.symbols {
                    *dreamer.symbol_mastery
                        .entry(symbol.symbol_type.clone())
                        .or_insert(0.0) += symbol.power * 0.1;
                }
            }
        }
        
        events.push(GameEvent {
            event_type: "dream_collapsed".to_string(),
            description: format!("Dream '{}' collapsed into reality!", dream_id),
            affected_players: affected_dreamers,
            data: serde_json::json!({
                "dream_id": dream_id,
                "final_reality": dream.reality_level,
                "symbols_absorbed": dream.symbols.len(),
            }),
        });
        
        // Remove the dream
        self.dream_states.remove(dream_id);
        
        // Update reality stability
        self.reality_stability *= 0.95;
        
        Some(events)
    }
    
    fn detect_synchronicity(&self) -> Option<EmergenceEvent> {
        // Check for meaningful coincidences across dreams
        let mut symbol_frequency = HashMap::new();
        
        for dream in self.dream_states.values() {
            for symbol in &dream.symbols {
                *symbol_frequency.entry(&symbol.symbol_type).or_insert(0) += 1;
            }
        }
        
        let max_frequency = symbol_frequency.values().max().copied().unwrap_or(0);
        
        if max_frequency >= self.dreamers.len() {
            Some(EmergenceEvent {
                round: self.round_number,
                event_type: EmergenceType::Custom("Synchronicity".to_string()),
                description: "Meaningful patterns emerging across all dreams!".to_string(),
                emergence_score: self.collective_unconscious.synchronicity_level,
                involved_players: self.dreamers.keys().cloned().collect(),
            })
        } else {
            None
        }
    }
    
    fn detect_collective_dream(&self) -> Option<EmergenceEvent> {
        // Check if all dreamers share a dream
        for dream in self.dream_states.values() {
            if dream.shared_dreamers.len() >= self.dreamers.len() {
                return Some(EmergenceEvent {
                    round: self.round_number,
                    event_type: EmergenceType::Custom("CollectiveDream".to_string()),
                    description: "All dreamers united in a single dream reality!".to_string(),
                    emergence_score: dream.coherence,
                    involved_players: self.dreamers.keys().cloned().collect(),
                });
            }
        }
        
        None
    }
}

impl Default for QuantumDreamerGame {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl Game for QuantumDreamerGame {
    async fn initialize(&mut self, config: GameConfig) -> Result<GameState> {
        let mut state = GameState::new(config.game_type.clone());
        
        let player_ids: Vec<String> = config.initial_players.iter()
            .map(|p| p.id.to_string())
            .collect();
        self.initialize_dreamscape(player_ids);
        
        state.metadata.insert(
            "description".to_string(),
            serde_json::json!("Navigate dream states where reality and imagination blur"),
        );
        state.metadata.insert(
            "reality_stability".to_string(),
            serde_json::json!(self.reality_stability),
        );
        state.metadata.insert(
            "dream_count".to_string(),
            serde_json::json!(self.dream_states.len()),
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
        
        // Update collective synchronicity
        self.collective_unconscious.synchronicity_level = 
            self.collective_unconscious.shared_symbols.values().sum::<f32>() 
            / self.collective_unconscious.shared_symbols.len().max(1) as f32;
        
        // Process dream actions
        for (player_id, action) in &actions {
            if let Ok(dream_action) = serde_json::from_value::<DreamAction>(action.data.clone()) {
                match dream_action.action_type {
                    DreamActionType::EnterDream => {
                        if let Some(target) = dream_action.target_dream {
                            if let Some(event) = self.enter_dream(player_id, &target) {
                                let success = event.event_type != "dream_blocked";
                                events.push(event);
                                
                                if success {
                                    scores_delta.insert(player_id.clone(), 5);
                                }
                            }
                        }
                    }
                    DreamActionType::CreateSymbol => {
                        if let (Some(dream_id), Some(symbol)) = 
                            (dream_action.target_dream, dream_action.symbol) {
                            if let Some(event) = self.create_symbol(player_id, &dream_id, symbol) {
                                events.push(event);
                                scores_delta.insert(player_id.clone(), 8);
                            }
                        }
                    }
                    DreamActionType::ManipulateDream => {
                        if let Some(dream_id) = dream_action.target_dream {
                            if let Some(event) = self.manipulate_dream(player_id, &dream_id) {
                                let is_bleed = event.event_type == "reality_bleed";
                                events.push(event);
                                
                                if is_bleed {
                                    scores_delta.insert(player_id.clone(), 20);
                                } else {
                                    scores_delta.insert(player_id.clone(), 10);
                                }
                            }
                        }
                    }
                    DreamActionType::ShareDream => {
                        if let (Some(dream_id), Some(others)) = 
                            (dream_action.target_dream, dream_action.other_dreamers) {
                            if let Some(dream) = self.dream_states.get_mut(&dream_id) {
                                for other_id in others {
                                    dream.shared_dreamers.insert(other_id);
                                }
                                scores_delta.insert(player_id.clone(), 5);
                                
                                events.push(GameEvent {
                                    event_type: "dream_shared".to_string(),
                                    description: format!("{} opened their dream to others", player_id),
                                    affected_players: dream.shared_dreamers.iter().cloned().collect(),
                                    data: serde_json::json!({
                                        "dream_id": dream_id,
                                        "dreamer_count": dream.shared_dreamers.len(),
                                    }),
                                });
                            }
                        }
                    }
                    DreamActionType::CollapseDream => {
                        if let Some(dream_id) = dream_action.target_dream {
                            if let Some(mut collapse_events) = self.collapse_dream(&dream_id) {
                                events.append(&mut collapse_events);
                                scores_delta.insert(player_id.clone(), 30);
                            }
                        }
                    }
                    DreamActionType::LucidControl => {
                        if let Some(dreamer) = self.dreamers.get_mut(player_id) {
                            dreamer.lucidity_level = (dreamer.lucidity_level + 0.2).min(1.0);
                            scores_delta.insert(player_id.clone(), 5);
                            
                            if dreamer.lucidity_level > self.lucidity_threshold {
                                events.push(GameEvent {
                                    event_type: "lucid_mastery".to_string(),
                                    description: format!("{} achieved lucid mastery!", player_id),
                                    affected_players: vec![player_id.clone()],
                                    data: serde_json::json!({
                                        "lucidity": dreamer.lucidity_level,
                                    }),
                                });
                                scores_delta.insert(player_id.clone(), 15);
                            }
                        }
                    }
                    DreamActionType::DreamWalk => {
                        // Walk between multiple dreams
                        if let Some(dreamer) = self.dreamers.get(player_id) {
                            if dreamer.lucidity_level > 0.6 {
                                let walk_count = dreamer.active_dreams.len();
                                scores_delta.insert(player_id.clone(), walk_count as i32 * 3);
                                
                                events.push(GameEvent {
                                    event_type: "dream_walk".to_string(),
                                    description: format!("{} walks between {} dreams", 
                                        player_id, walk_count),
                                    affected_players: vec![player_id.clone()],
                                    data: serde_json::json!({
                                        "dreams_visited": walk_count,
                                    }),
                                });
                            }
                        }
                    }
                }
            }
        }
        
        // Process reality bleeds
        for bleed in &self.reality_bleeds {
            for affected_id in &bleed.affected_dreams {
                if let Some(affected_dream) = self.dream_states.get_mut(affected_id) {
                    affected_dream.reality_level += bleed.bleed_strength * 0.1;
                    
                    // Symbols leak between dreams
                    for symbol_type in &bleed.symbol_leakage {
                        if !affected_dream.symbols.iter().any(|s| &s.symbol_type == symbol_type) {
                            affected_dream.symbols.push(DreamSymbol {
                                symbol_type: symbol_type.clone(),
                                meaning: "Leaked from another dream".to_string(),
                                power: bleed.bleed_strength,
                                stability: 0.5,
                            });
                        }
                    }
                }
            }
        }
        
        // Check for emergence
        let mut emergence_detected = false;
        
        if let Some(sync_event) = self.detect_synchronicity() {
            emergence_detected = true;
            events.push(GameEvent {
                event_type: "emergence".to_string(),
                description: sync_event.description.clone(),
                affected_players: sync_event.involved_players.clone(),
                data: serde_json::json!({
                    "type": "synchronicity",
                    "level": self.collective_unconscious.synchronicity_level,
                }),
            });
            
            // Bonus for synchronicity
            for player_id in self.dreamers.keys() {
                *scores_delta.entry(player_id.clone()).or_insert(0) += 25;
            }
        }
        
        if let Some(collective_event) = self.detect_collective_dream() {
            emergence_detected = true;
            events.push(GameEvent {
                event_type: "emergence".to_string(),
                description: collective_event.description.clone(),
                affected_players: collective_event.involved_players.clone(),
                data: serde_json::json!({
                    "type": "collective_dream",
                }),
            });
            
            // Major bonus for collective dreaming
            for player_id in self.dreamers.keys() {
                *scores_delta.entry(player_id.clone()).or_insert(0) += 50;
            }
        }
        
        let outcome = RoundOutcome {
            winners: scores_delta.iter()
                .filter(|(_, &score)| score > 15)
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
        state.round >= 50 || 
        self.reality_stability < 0.2 || 
        self.collective_unconscious.synchronicity_level > 0.95
    }
    
    async fn calculate_final_result(&self, state: &GameState) -> GameResult {
        let winner = state.scores.iter()
            .max_by_key(|(_, &score)| score)
            .map(|(player, _)| player.clone())
            .unwrap_or_default();
            
        let avg_lucidity = self.dreamers.values()
            .map(|d| d.lucidity_level)
            .sum::<f32>() / self.dreamers.len() as f32;
            
        let total_symbols = self.dream_states.values()
            .map(|d| d.symbols.len())
            .sum::<usize>();
            
        GameResult {
            game_id: state.game_id,
            winner,
            final_scores: state.scores.clone(),
            total_rounds: state.round,
            duration_ms: 0,
            emergence_events: vec![],
            analytics: GameAnalytics {
                collective_coordination_score: self.collective_unconscious.synchronicity_level,
                decision_diversity_index: 0.7,
                strategic_depth: avg_lucidity,
                emergence_frequency: self.reality_bleeds.len() as f32 / state.round as f32,
                performance_differential: 0.0,
                custom_metrics: {
                    let mut metrics = HashMap::new();
                    metrics.insert("avg_lucidity".to_string(), avg_lucidity);
                    metrics.insert("reality_stability".to_string(), self.reality_stability);
                    metrics.insert("total_dreams".to_string(), self.dream_states.len() as f32);
                    metrics.insert("total_symbols".to_string(), total_symbols as f32);
                    metrics.insert("reality_bleeds".to_string(), self.reality_bleeds.len() as f32);
                    metrics
                },
            },
        }
    }
    
    async fn get_valid_actions(&self, _state: &GameState, _player_id: &str) -> Vec<String> {
        vec![
            "enter_dream".to_string(),
            "create_symbol".to_string(),
            "manipulate_dream".to_string(),
            "share_dream".to_string(),
            "collapse_dream".to_string(),
            "lucid_control".to_string(),
            "dream_walk".to_string(),
        ]
    }
    
    async fn get_visualization_data(&self, _state: &GameState) -> serde_json::Value {
        serde_json::json!({
            "dreamers": self.dreamers,
            "dream_states": self.dream_states,
            "reality_bleeds": self.reality_bleeds,
            "collective_unconscious": self.collective_unconscious,
            "reality_stability": self.reality_stability,
        })
    }
}