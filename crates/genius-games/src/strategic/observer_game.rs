//! The Observer Game - Quantum measurement and observation effects

use genius_core::{
    Game, GameConfig, GameState, GameType, PlayerAction, RoundResult,
    RoundOutcome, GameResult, GameAnalytics, GameEvent, EmergenceEvent, EmergenceType,
    Result, GameError,
};
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use rand::Rng;

/// Quantum state that can be in superposition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuantumState {
    pub id: String,
    pub superposition: Vec<PossibleState>,
    pub is_collapsed: bool,
    pub collapsed_state: Option<String>,
    pub entangled_with: Vec<String>,
    pub coherence: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PossibleState {
    pub state_value: String,
    pub amplitude: f32,
    pub phase: f32,
}

/// Observer with limited observation power
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Observer {
    pub observation_power: f32,
    pub observations_made: u32,
    pub collapsed_states: Vec<String>,
    pub quantum_vision: bool, // Can see superposition
    pub entanglement_mastery: f32,
}

/// Observation action
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ObserverAction {
    pub action_type: ObserverActionType,
    pub target_state: Option<String>,
    pub entangle_with: Option<Vec<String>>,
    pub measurement_type: Option<MeasurementType>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ObserverActionType {
    Observe,
    EntangleStates,
    WeakMeasurement,
    QuantumErase,
    Interfere,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MeasurementType {
    Position,
    Momentum,
    Spin,
    Energy,
}

pub struct ObserverGame {
    round_number: u32,
    quantum_states: HashMap<String, QuantumState>,
    observers: HashMap<String, Observer>,
    collapsed_count: u32,
    total_states: u32,
    decoherence_rate: f32,
    measurement_backaction: f32,
    quantum_objectives: Vec<QuantumObjective>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct QuantumObjective {
    description: String,
    target_pattern: Vec<String>,
    reward: i32,
}

impl ObserverGame {
    pub fn new() -> Self {
        Self {
            round_number: 0,
            quantum_states: HashMap::new(),
            observers: HashMap::new(),
            collapsed_count: 0,
            total_states: 20,
            decoherence_rate: 0.05,
            measurement_backaction: 0.3,
            quantum_objectives: Self::generate_objectives(),
        }
    }
    
    fn generate_objectives() -> Vec<QuantumObjective> {
        vec![
            QuantumObjective {
                description: "Create entangled pair".to_string(),
                target_pattern: vec!["entangled".to_string()],
                reward: 20,
            },
            QuantumObjective {
                description: "Maintain superposition for 5 rounds".to_string(),
                target_pattern: vec!["superposition_stable".to_string()],
                reward: 30,
            },
            QuantumObjective {
                description: "Collapse to specific pattern".to_string(),
                target_pattern: vec!["0".to_string(), "1".to_string(), "0".to_string()],
                reward: 25,
            },
        ]
    }
    
    fn initialize_quantum_field(&mut self, num_players: usize) {
        let mut rng = rand::rng();
        
        // Create quantum states in superposition
        for i in 0..self.total_states {
            let state_id = format!("q_{}", i);
            
            let superposition = vec![
                PossibleState {
                    state_value: "0".to_string(),
                    amplitude: rng.random::<f32>().sqrt(),
                    phase: rng.random::<f32>() * 2.0 * std::f32::consts::PI,
                },
                PossibleState {
                    state_value: "1".to_string(),
                    amplitude: rng.random::<f32>().sqrt(),
                    phase: rng.random::<f32>() * 2.0 * std::f32::consts::PI,
                },
            ];
            
            // Normalize amplitudes
            let total_prob: f32 = superposition.iter()
                .map(|s| s.amplitude * s.amplitude)
                .sum();
            let norm_factor = 1.0 / total_prob.sqrt();
            
            let normalized_superposition = superposition.into_iter()
                .map(|mut s| {
                    s.amplitude *= norm_factor;
                    s
                })
                .collect();
            
            self.quantum_states.insert(state_id.clone(), QuantumState {
                id: state_id,
                superposition: normalized_superposition,
                is_collapsed: false,
                collapsed_state: None,
                entangled_with: Vec::new(),
                coherence: 1.0,
            });
        }
    }
    
    fn initialize_observers(&mut self, player_ids: Vec<String>) {
        for player_id in player_ids {
            self.observers.insert(player_id, Observer {
                observation_power: 0.7,
                observations_made: 0,
                collapsed_states: Vec::new(),
                quantum_vision: false,
                entanglement_mastery: 0.3,
            });
        }
    }
    
    fn perform_observation(
        &mut self,
        observer_id: &str,
        state_id: &str,
        measurement_type: &MeasurementType
    ) -> Option<GameEvent> {
        let observer = self.observers.get_mut(observer_id)?;
        let state = self.quantum_states.get_mut(state_id)?;
        
        if state.is_collapsed {
            return Some(GameEvent {
                event_type: "already_collapsed".to_string(),
                description: format!("{} observed already collapsed state {}", observer_id, state_id),
                affected_players: vec![observer_id.to_string()],
                data: serde_json::json!({
                    "state_id": state_id,
                    "collapsed_value": state.collapsed_state,
                }),
            });
        }
        
        observer.observations_made += 1;
        
        // Observation collapses the state based on probabilities
        let mut rng = rand::rng();
        let observation_strength = observer.observation_power * (1.0 - self.measurement_backaction);
        
        if rng.random::<f32>() < observation_strength {
            // Collapse the state
            let random_val = rng.random::<f32>();
            let mut cumulative_prob = 0.0;
            
            for possible in &state.superposition {
                cumulative_prob += possible.amplitude * possible.amplitude;
                if random_val < cumulative_prob {
                    state.collapsed_state = Some(possible.state_value.clone());
                    state.is_collapsed = true;
                    self.collapsed_count += 1;
                    observer.collapsed_states.push(state_id.to_string());
                    
                    // Collapse entangled states
                    let entangled = state.entangled_with.clone();
                    for entangled_id in entangled {
                        if let Some(entangled_state) = self.quantum_states.get_mut(&entangled_id) {
                            if !entangled_state.is_collapsed {
                                // Entangled states collapse to correlated values
                                entangled_state.collapsed_state = Some(
                                    if possible.state_value == "0" { "1" } else { "0" }.to_string()
                                );
                                entangled_state.is_collapsed = true;
                                self.collapsed_count += 1;
                            }
                        }
                    }
                    
                    return Some(GameEvent {
                        event_type: "observation_collapse".to_string(),
                        description: format!("{} collapsed {} to {}", 
                            observer_id, state_id, possible.state_value),
                        affected_players: vec![observer_id.to_string()],
                        data: serde_json::json!({
                            "state_id": state_id,
                            "collapsed_to": possible.state_value,
                            "measurement_type": format!("{:?}", measurement_type),
                            "entangled_collapsed": state.entangled_with.len(),
                        }),
                    });
                }
            }
        } else {
            // Weak measurement - disturb but don't collapse
            state.coherence *= 1.0 - self.measurement_backaction;
            
            return Some(GameEvent {
                event_type: "weak_measurement".to_string(),
                description: format!("{} performed weak measurement on {}", observer_id, state_id),
                affected_players: vec![observer_id.to_string()],
                data: serde_json::json!({
                    "state_id": state_id,
                    "coherence": state.coherence,
                }),
            });
        }
        
        None
    }
    
    fn create_entanglement(
        &mut self,
        observer_id: &str,
        state_ids: Vec<String>
    ) -> Option<GameEvent> {
        if state_ids.len() < 2 {
            return None;
        }
        
        let observer = self.observers.get(observer_id)?;
        
        // Check if states can be entangled
        let mut can_entangle = true;
        for state_id in &state_ids {
            if let Some(state) = self.quantum_states.get(state_id) {
                if state.is_collapsed {
                    can_entangle = false;
                    break;
                }
            }
        }
        
        if !can_entangle {
            return None;
        }
        
        // Create entanglement with probability based on mastery
        let mut rng = rand::rng();
        if rng.random::<f32>() < observer.entanglement_mastery {
            // Entangle the states
            for i in 0..state_ids.len() {
                for j in 0..state_ids.len() {
                    if i != j {
                        if let Some(state) = self.quantum_states.get_mut(&state_ids[i]) {
                            if !state.entangled_with.contains(&state_ids[j]) {
                                state.entangled_with.push(state_ids[j].clone());
                            }
                        }
                    }
                }
            }
            
            Some(GameEvent {
                event_type: "entanglement_created".to_string(),
                description: format!("{} entangled {} states", observer_id, state_ids.len()),
                affected_players: vec![observer_id.to_string()],
                data: serde_json::json!({
                    "entangled_states": state_ids,
                }),
            })
        } else {
            Some(GameEvent {
                event_type: "entanglement_failed".to_string(),
                description: format!("{} failed to create entanglement", observer_id),
                affected_players: vec![observer_id.to_string()],
                data: serde_json::json!({}),
            })
        }
    }
    
    fn apply_decoherence(&mut self) {
        let mut rng = rand::rng();
        
        for state in self.quantum_states.values_mut() {
            if !state.is_collapsed {
                state.coherence *= 1.0 - self.decoherence_rate;
                
                // Random collapse if coherence too low
                if state.coherence < 0.3 && rng.random::<f32>() < 0.5 {
                    let random_idx = rng.random_range(0..state.superposition.len());
                    state.collapsed_state = Some(state.superposition[random_idx].state_value.clone());
                    state.is_collapsed = true;
                    self.collapsed_count += 1;
                }
            }
        }
    }
    
    fn check_quantum_objectives(&self, observer_id: &str) -> Vec<(QuantumObjective, bool)> {
        let mut completed = Vec::new();
        
        if let Some(observer) = self.observers.get(observer_id) {
            for objective in &self.quantum_objectives {
                match objective.description.as_str() {
                    "Create entangled pair" => {
                        let has_entangled = self.quantum_states.values()
                            .any(|s| s.entangled_with.len() >= 1 && 
                                    observer.collapsed_states.contains(&s.id));
                        completed.push((objective.clone(), has_entangled));
                    }
                    "Maintain superposition for 5 rounds" => {
                        let maintained = self.quantum_states.values()
                            .any(|s| !s.is_collapsed && s.coherence > 0.7);
                        completed.push((objective.clone(), maintained && self.round_number >= 5));
                    }
                    _ => {}
                }
            }
        }
        
        completed
    }
    
    fn detect_quantum_emergence(&self) -> Option<EmergenceEvent> {
        let entangled_ratio = self.quantum_states.values()
            .filter(|s| s.entangled_with.len() > 0)
            .count() as f32 / self.total_states as f32;
            
        let superposition_ratio = self.quantum_states.values()
            .filter(|s| !s.is_collapsed)
            .count() as f32 / self.total_states as f32;
            
        if entangled_ratio > 0.5 && superposition_ratio > 0.3 {
            Some(EmergenceEvent {
                round: self.round_number,
                event_type: EmergenceType::Custom("QuantumCoherence".to_string()),
                description: "Quantum field achieving macroscopic coherence!".to_string(),
                emergence_score: (entangled_ratio + superposition_ratio) / 2.0,
                involved_players: self.observers.keys().cloned().collect(),
            })
        } else {
            None
        }
    }
}

impl Default for ObserverGame {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl Game for ObserverGame {
    async fn initialize(&mut self, config: GameConfig) -> Result<GameState> {
        let mut state = GameState::new(config.game_type.clone());
        
        let player_ids: Vec<String> = config.initial_players.iter()
            .map(|p| p.id.to_string())
            .collect();
            
        self.initialize_quantum_field(player_ids.len());
        self.initialize_observers(player_ids);
        
        state.metadata.insert(
            "description".to_string(),
            serde_json::json!("Collapse quantum states through observation, but beware the consequences"),
        );
        state.metadata.insert(
            "total_states".to_string(),
            serde_json::json!(self.total_states),
        );
        state.metadata.insert(
            "decoherence_rate".to_string(),
            serde_json::json!(self.decoherence_rate),
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
        
        // Apply natural decoherence
        self.apply_decoherence();
        
        // Process observer actions
        for (player_id, action) in &actions {
            if let Ok(obs_action) = serde_json::from_value::<ObserverAction>(action.data.clone()) {
                match obs_action.action_type {
                    ObserverActionType::Observe => {
                        if let (Some(target), Some(mtype)) = (obs_action.target_state, obs_action.measurement_type) {
                            if let Some(event) = self.perform_observation(player_id, &target, &mtype) {
                                let is_collapse = event.event_type == "observation_collapse";
                                events.push(event);
                                
                                if is_collapse {
                                    scores_delta.insert(player_id.clone(), 10);
                                } else {
                                    scores_delta.insert(player_id.clone(), 2);
                                }
                            }
                        }
                    }
                    ObserverActionType::EntangleStates => {
                        if let Some(targets) = obs_action.entangle_with {
                            if let Some(event) = self.create_entanglement(player_id, targets) {
                                let is_success = event.event_type == "entanglement_created";
                                events.push(event);
                                
                                if is_success {
                                    scores_delta.insert(player_id.clone(), 15);
                                    
                                    // Increase entanglement mastery
                                    if let Some(observer) = self.observers.get_mut(player_id) {
                                        observer.entanglement_mastery = (observer.entanglement_mastery + 0.1).min(1.0);
                                    }
                                }
                            }
                        }
                    }
                    ObserverActionType::WeakMeasurement => {
                        if let Some(observer) = self.observers.get_mut(player_id) {
                            observer.quantum_vision = true;
                            scores_delta.insert(player_id.clone(), 5);
                            
                            events.push(GameEvent {
                                event_type: "quantum_vision".to_string(),
                                description: format!("{} gained quantum vision", player_id),
                                affected_players: vec![player_id.clone()],
                                data: serde_json::json!({}),
                            });
                        }
                    }
                    ObserverActionType::QuantumErase => {
                        // Attempt to uncollapse a state (very difficult)
                        if let Some(target) = obs_action.target_state {
                            if let Some(state) = self.quantum_states.get_mut(&target) {
                                if state.is_collapsed && state.coherence > 0.5 {
                                    let mut rng = rand::rng();
                                    if rng.random::<f32>() < 0.1 { // 10% chance
                                        state.is_collapsed = false;
                                        state.collapsed_state = None;
                                        self.collapsed_count -= 1;
                                        
                                        scores_delta.insert(player_id.clone(), 30);
                                        events.push(GameEvent {
                                            event_type: "quantum_erasure".to_string(),
                                            description: format!("{} erased quantum measurement!", player_id),
                                            affected_players: vec![player_id.clone()],
                                            data: serde_json::json!({
                                                "erased_state": target,
                                            }),
                                        });
                                    }
                                }
                            }
                        }
                    }
                    ObserverActionType::Interfere => {
                        // Create interference between superposed states
                        scores_delta.insert(player_id.clone(), 3);
                    }
                }
            }
        }
        
        // Check objectives
        for (player_id, observer) in &self.observers {
            let completed_objectives = self.check_quantum_objectives(player_id);
            for (objective, completed) in completed_objectives {
                if completed {
                    *scores_delta.entry(player_id.clone()).or_insert(0) += objective.reward;
                    
                    events.push(GameEvent {
                        event_type: "objective_complete".to_string(),
                        description: format!("{} completed: {}", player_id, objective.description),
                        affected_players: vec![player_id.clone()],
                        data: serde_json::json!({
                            "reward": objective.reward,
                        }),
                    });
                }
            }
        }
        
        // Check for quantum emergence
        let mut emergence_detected = false;
        if let Some(emergence) = self.detect_quantum_emergence() {
            emergence_detected = true;
            events.push(GameEvent {
                event_type: "emergence".to_string(),
                description: emergence.description.clone(),
                affected_players: emergence.involved_players.clone(),
                data: serde_json::json!({
                    "emergence_score": emergence.emergence_score,
                }),
            });
            
            // Bonus for maintaining quantum coherence
            for player_id in self.observers.keys() {
                *scores_delta.entry(player_id.clone()).or_insert(0) += 25;
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
        // Game ends when most states collapse or max rounds
        let collapse_ratio = self.collapsed_count as f32 / self.total_states as f32;
        state.round >= 50 || collapse_ratio > 0.9
    }
    
    async fn calculate_final_result(&self, state: &GameState) -> GameResult {
        // Winner is observer who best balanced observation and preservation
        let mut final_scores = HashMap::new();
        
        for (player_id, observer) in &self.observers {
            let observation_score = observer.collapsed_states.len() as i32 * 10;
            let mastery_score = (observer.entanglement_mastery * 100.0) as i32;
            let vision_score = if observer.quantum_vision { 50 } else { 0 };
            
            final_scores.insert(
                player_id.clone(),
                observation_score + mastery_score + vision_score,
            );
        }
        
        let winner = final_scores.iter()
            .max_by_key(|(_, &score)| score)
            .map(|(player, _)| player.clone())
            .unwrap_or_default();
            
        let superposition_remaining = self.quantum_states.values()
            .filter(|s| !s.is_collapsed)
            .count() as f32 / self.total_states as f32;
            
        GameResult {
            game_id: state.game_id,
            winner,
            final_scores: state.scores.clone(),
            total_rounds: state.round,
            duration_ms: 0,
            emergence_events: vec![],
            analytics: GameAnalytics {
                collective_coordination_score: superposition_remaining,
                decision_diversity_index: 0.7,
                strategic_depth: 0.8,
                emergence_frequency: 0.0,
                performance_differential: 0.0,
                custom_metrics: {
                    let mut metrics = HashMap::new();
                    metrics.insert("collapse_ratio".to_string(), 
                        self.collapsed_count as f32 / self.total_states as f32);
                    metrics.insert("entanglement_ratio".to_string(),
                        self.quantum_states.values()
                            .filter(|s| s.entangled_with.len() > 0)
                            .count() as f32 / self.total_states as f32);
                    metrics.insert("avg_coherence".to_string(),
                        self.quantum_states.values()
                            .map(|s| s.coherence)
                            .sum::<f32>() / self.total_states as f32);
                    metrics
                },
            },
        }
    }
    
    async fn get_valid_actions(&self, _state: &GameState, _player_id: &str) -> Vec<String> {
        vec![
            "observe".to_string(),
            "entangle".to_string(),
            "weak_measure".to_string(),
            "quantum_erase".to_string(),
            "interfere".to_string(),
        ]
    }
    
    async fn get_visualization_data(&self, _state: &GameState) -> serde_json::Value {
        serde_json::json!({
            "quantum_states": self.quantum_states,
            "observers": self.observers,
            "collapsed_ratio": self.collapsed_count as f32 / self.total_states as f32,
            "objectives": self.quantum_objectives,
        })
    }
}