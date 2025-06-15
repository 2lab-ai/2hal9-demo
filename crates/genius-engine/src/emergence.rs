//! Emergence detection and analysis

use genius_core::{GameState, RoundResult, EmergenceEvent, EmergenceType};
use std::collections::HashMap;

/// Detects emergence patterns in game behavior
pub struct EmergenceDetector {
    /// History of pattern observations
    pattern_history: Vec<PatternObservation>,
    /// Thresholds for different emergence types
    thresholds: HashMap<EmergenceType, f32>,
}

#[derive(Clone, Debug)]
struct PatternObservation {
    round: u32,
    pattern_type: String,
    strength: f32,
    players_involved: Vec<String>,
}

impl EmergenceDetector {
    pub fn new() -> Self {
        let mut thresholds = HashMap::new();
        thresholds.insert(EmergenceType::CollectiveStrategy, 0.7);
        thresholds.insert(EmergenceType::SwarmIntelligence, 0.8);
        thresholds.insert(EmergenceType::NashEquilibrium, 0.85);
        thresholds.insert(EmergenceType::PhaseTransition, 0.75);
        thresholds.insert(EmergenceType::SpontaneousCoordination, 0.8);
        
        Self {
            pattern_history: Vec::new(),
            thresholds,
        }
    }
    
    /// Analyze a round for emergence events
    pub fn analyze_round(&mut self, state: &GameState, round_result: &RoundResult) -> Vec<EmergenceEvent> {
        let mut events = Vec::new();
        
        // Check for collective strategy emergence
        if let Some(event) = self.check_collective_strategy(state, round_result) {
            events.push(event);
        }
        
        // Check for spontaneous coordination
        if let Some(event) = self.check_spontaneous_coordination(state, round_result) {
            events.push(event);
        }
        
        // Check for phase transitions
        if let Some(event) = self.check_phase_transition(state, round_result) {
            events.push(event);
        }
        
        events
    }
    
    fn check_collective_strategy(&mut self, state: &GameState, round_result: &RoundResult) -> Option<EmergenceEvent> {
        // Analyze if players are converging on a collective strategy
        let action_types: HashMap<String, usize> = round_result.actions.values()
            .map(|a| a.action_type.clone())
            .fold(HashMap::new(), |mut acc, action| {
                *acc.entry(action).or_insert(0) += 1;
                acc
            });
            
        let total_actions = round_result.actions.len() as f32;
        let max_consensus = action_types.values().max().copied().unwrap_or(0) as f32;
        let consensus_ratio = max_consensus / total_actions;
        
        if consensus_ratio > self.thresholds[&EmergenceType::CollectiveStrategy] {
            return Some(EmergenceEvent {
                round: state.round,
                event_type: EmergenceType::CollectiveStrategy,
                description: format!("Players converging on collective strategy with {:.1}% consensus", consensus_ratio * 100.0),
                emergence_score: consensus_ratio,
                involved_players: round_result.actions.keys().cloned().collect(),
            });
        }
        
        None
    }
    
    fn check_spontaneous_coordination(&mut self, state: &GameState, round_result: &RoundResult) -> Option<EmergenceEvent> {
        // Check if players are coordinating without explicit communication
        // This is simplified - real implementation would be more sophisticated
        
        let coordination_score = self.calculate_coordination_score(round_result);
        
        if coordination_score > self.thresholds[&EmergenceType::SpontaneousCoordination] {
            return Some(EmergenceEvent {
                round: state.round,
                event_type: EmergenceType::SpontaneousCoordination,
                description: "Players exhibiting spontaneous coordination without explicit communication".to_string(),
                emergence_score: coordination_score,
                involved_players: round_result.actions.keys().cloned().collect(),
            });
        }
        
        None
    }
    
    fn check_phase_transition(&mut self, state: &GameState, round_result: &RoundResult) -> Option<EmergenceEvent> {
        // Detect sudden changes in collective behavior
        if self.pattern_history.len() < 5 {
            return None;
        }
        
        // Compare recent patterns to historical patterns
        let recent_avg = self.recent_pattern_strength(3);
        let historical_avg = self.historical_pattern_strength(10);
        
        let change_ratio = (recent_avg - historical_avg).abs() / historical_avg.max(0.1);
        
        if change_ratio > 0.5 {
            return Some(EmergenceEvent {
                round: state.round,
                event_type: EmergenceType::PhaseTransition,
                description: format!("System undergoing phase transition with {:.1}% change in behavior", change_ratio * 100.0),
                emergence_score: change_ratio.min(1.0),
                involved_players: round_result.actions.keys().cloned().collect(),
            });
        }
        
        None
    }
    
    fn calculate_coordination_score(&self, round_result: &RoundResult) -> f32 {
        // Simplified coordination score based on action similarity
        if round_result.actions.len() < 2 {
            return 0.0;
        }
        
        let mut similarity_sum = 0.0;
        let actions: Vec<_> = round_result.actions.values().collect();
        let mut comparison_count = 0;
        
        for i in 0..actions.len() {
            for j in i+1..actions.len() {
                if actions[i].action_type == actions[j].action_type {
                    similarity_sum += 1.0;
                }
                comparison_count += 1;
            }
        }
        
        if comparison_count > 0 {
            similarity_sum / comparison_count as f32
        } else {
            0.0
        }
    }
    
    fn recent_pattern_strength(&self, window: usize) -> f32 {
        let recent: Vec<_> = self.pattern_history.iter()
            .rev()
            .take(window)
            .collect();
            
        if recent.is_empty() {
            return 0.0;
        }
        
        recent.iter().map(|p| p.strength).sum::<f32>() / recent.len() as f32
    }
    
    fn historical_pattern_strength(&self, window: usize) -> f32 {
        let historical: Vec<_> = self.pattern_history.iter()
            .rev()
            .skip(3)
            .take(window)
            .collect();
            
        if historical.is_empty() {
            return 0.0;
        }
        
        historical.iter().map(|p| p.strength).sum::<f32>() / historical.len() as f32
    }
}

impl Default for EmergenceDetector {
    fn default() -> Self {
        Self::new()
    }
}