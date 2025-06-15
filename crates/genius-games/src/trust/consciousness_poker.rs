//! Consciousness Poker - A game of awareness levels and strategic deception

use genius_core::{
    Game, GameConfig, GameState, GameResult, PlayerAction, RoundResult,
    RoundOutcome, GameEvent, GameAnalytics, EmergenceEvent, EmergenceType,
    Result, GameError,
};
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use rand::{thread_rng, Rng};

/// The level of consciousness/awareness
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, PartialOrd, Ord)]
pub enum ConsciousnessLevel {
    Level1, // Basic awareness
    Level2, // Self-awareness
    Level3, // Other-awareness
    Level4, // Meta-awareness
    Level5, // Recursive awareness
    Level6, // Collective awareness
    Level7, // Transcendent awareness
    Level8, // Universal awareness
    Level9, // Omniscient awareness
}

impl ConsciousnessLevel {
    fn as_u8(&self) -> u8 {
        match self {
            ConsciousnessLevel::Level1 => 1,
            ConsciousnessLevel::Level2 => 2,
            ConsciousnessLevel::Level3 => 3,
            ConsciousnessLevel::Level4 => 4,
            ConsciousnessLevel::Level5 => 5,
            ConsciousnessLevel::Level6 => 6,
            ConsciousnessLevel::Level7 => 7,
            ConsciousnessLevel::Level8 => 8,
            ConsciousnessLevel::Level9 => 9,
        }
    }
    
    fn from_u8(level: u8) -> Self {
        match level {
            1 => ConsciousnessLevel::Level1,
            2 => ConsciousnessLevel::Level2,
            3 => ConsciousnessLevel::Level3,
            4 => ConsciousnessLevel::Level4,
            5 => ConsciousnessLevel::Level5,
            6 => ConsciousnessLevel::Level6,
            7 => ConsciousnessLevel::Level7,
            8 => ConsciousnessLevel::Level8,
            _ => ConsciousnessLevel::Level9,
        }
    }
    
    fn can_perceive(&self, other: ConsciousnessLevel) -> bool {
        self.as_u8() >= other.as_u8()
    }
}

/// Player's state in consciousness poker
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConsciousnessPlayer {
    actual_level: ConsciousnessLevel,
    claimed_level: Option<ConsciousnessLevel>,
    chips: i32,
    perception_history: Vec<PerceptionEvent>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerceptionEvent {
    round: u32,
    perceived_player: String,
    actual_level: ConsciousnessLevel,
    claimed_level: ConsciousnessLevel,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PokerAction {
    action_type: PokerActionType,
    claimed_level: Option<ConsciousnessLevel>,
    bet_amount: Option<i32>,
    target_player: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum PokerActionType {
    ClaimLevel,
    Challenge,
    Fold,
    Bet,
    Call,
    Raise,
    AllIn,
}

pub struct ConsciousnessPokerGame {
    round_number: u32,
    players: HashMap<String, ConsciousnessPlayer>,
    pot: i32,
    current_bet: i32,
    betting_round: u32,
    enlightenment_threshold: f32,
}

impl ConsciousnessPokerGame {
    pub fn new() -> Self {
        Self {
            round_number: 0,
            players: HashMap::new(),
            pot: 0,
            current_bet: 0,
            betting_round: 0,
            enlightenment_threshold: 0.8,
        }
    }
    
    fn assign_consciousness_levels(&mut self, player_ids: Vec<String>) {
        let mut rng = thread_rng();
        
        for player_id in player_ids {
            let level = ConsciousnessLevel::from_u8(rng.gen_range(1..=9));
            
            self.players.insert(player_id, ConsciousnessPlayer {
                actual_level: level,
                claimed_level: None,
                chips: 1000, // Starting chips
                perception_history: Vec::new(),
            });
        }
    }
    
    fn get_visible_information(&self, observer_id: &str) -> serde_json::Value {
        let observer = match self.players.get(observer_id) {
            Some(p) => p,
            None => return serde_json::json!({}),
        };
        
        let mut visible_players = HashMap::new();
        
        for (player_id, player) in &self.players {
            if player_id == observer_id {
                // Always see yourself fully
                visible_players.insert(player_id.clone(), serde_json::json!({
                    "actual_level": player.actual_level,
                    "claimed_level": player.claimed_level,
                    "chips": player.chips,
                }));
            } else {
                // See others based on consciousness level
                let mut info = serde_json::json!({
                    "claimed_level": player.claimed_level,
                    "chips": player.chips,
                });
                
                // Higher consciousness can see through lower levels
                if observer.actual_level.can_perceive(player.actual_level) {
                    info["actual_level"] = serde_json::json!(player.actual_level);
                }
                
                visible_players.insert(player_id.clone(), info);
            }
        }
        
        serde_json::json!({
            "players": visible_players,
            "pot": self.pot,
            "current_bet": self.current_bet,
            "your_level": observer.actual_level,
        })
    }
    
    fn calculate_deception_bonus(&self, player: &ConsciousnessPlayer) -> f32 {
        match (&player.claimed_level, &player.actual_level) {
            (Some(claimed), actual) => {
                let difference = (actual.as_u8() as i32 - claimed.as_u8() as i32).abs();
                difference as f32 * 0.1 // Bonus for successful deception
            }
            _ => 0.0,
        }
    }
    
    fn detect_enlightenment(&self) -> Option<EmergenceEvent> {
        // Check if players are achieving high consciousness collectively
        let high_level_count = self.players.values()
            .filter(|p| p.actual_level.as_u8() >= 7)
            .count();
            
        let enlightenment_ratio = high_level_count as f32 / self.players.len() as f32;
        
        if enlightenment_ratio > self.enlightenment_threshold {
            Some(EmergenceEvent {
                round: self.round_number,
                event_type: EmergenceType::Custom("CollectiveEnlightenment".to_string()),
                description: format!("{:.0}% of players have achieved high consciousness", enlightenment_ratio * 100.0),
                emergence_score: enlightenment_ratio,
                involved_players: self.players.keys().cloned().collect(),
            })
        } else {
            None
        }
    }
    
    fn resolve_betting_round(&mut self, actions: &HashMap<String, PlayerAction>) -> Vec<GameEvent> {
        let mut events = Vec::new();
        let mut folded_players = Vec::new();
        
        for (player_id, action) in actions {
            if let Ok(poker_action) = serde_json::from_value::<PokerAction>(action.data.clone()) {
                if let Some(player) = self.players.get_mut(player_id) {
                    match poker_action.action_type {
                        PokerActionType::Bet => {
                            if let Some(amount) = poker_action.bet_amount {
                                let bet = amount.min(player.chips);
                                player.chips -= bet;
                                self.pot += bet;
                                self.current_bet = self.current_bet.max(bet);
                            }
                        }
                        PokerActionType::Call => {
                            let call_amount = self.current_bet.min(player.chips);
                            player.chips -= call_amount;
                            self.pot += call_amount;
                        }
                        PokerActionType::Raise => {
                            if let Some(amount) = poker_action.bet_amount {
                                let raise_amount = amount.min(player.chips);
                                player.chips -= raise_amount;
                                self.pot += raise_amount;
                                self.current_bet = raise_amount;
                            }
                        }
                        PokerActionType::AllIn => {
                            let all_in = player.chips;
                            self.pot += all_in;
                            player.chips = 0;
                            self.current_bet = self.current_bet.max(all_in);
                            
                            events.push(GameEvent {
                                event_type: "all_in".to_string(),
                                description: format!("{} goes all in with {} chips!", player_id, all_in),
                                affected_players: vec![player_id.clone()],
                                data: serde_json::json!({ "amount": all_in }),
                            });
                        }
                        PokerActionType::Fold => {
                            folded_players.push(player_id.clone());
                        }
                        PokerActionType::ClaimLevel => {
                            if let Some(level) = poker_action.claimed_level {
                                player.claimed_level = Some(level);
                                
                                events.push(GameEvent {
                                    event_type: "consciousness_claim".to_string(),
                                    description: format!("{} claims consciousness level {}", player_id, level.as_u8()),
                                    affected_players: vec![player_id.clone()],
                                    data: serde_json::json!({ "claimed_level": level }),
                                });
                            }
                        }
                        PokerActionType::Challenge => {
                            if let Some(target) = poker_action.target_player {
                                if let Some(target_player) = self.players.get(&target) {
                                    // Challenger must have higher consciousness to see truth
                                    if player.actual_level.can_perceive(target_player.actual_level) {
                                        player.perception_history.push(PerceptionEvent {
                                            round: self.round_number,
                                            perceived_player: target.clone(),
                                            actual_level: target_player.actual_level,
                                            claimed_level: target_player.claimed_level.unwrap_or(ConsciousnessLevel::Level1),
                                        });
                                        
                                        events.push(GameEvent {
                                            event_type: "successful_perception".to_string(),
                                            description: format!("{} successfully perceives {}'s true consciousness", player_id, target),
                                            affected_players: vec![player_id.clone(), target.clone()],
                                            data: serde_json::json!({ "perception_successful": true }),
                                        });
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        
        events
    }
}

impl Default for ConsciousnessPokerGame {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl Game for ConsciousnessPokerGame {
    async fn initialize(&mut self, config: GameConfig) -> Result<GameState> {
        let mut state = GameState::new(config.game_type.clone());
        
        // Assign consciousness levels to players
        let player_ids: Vec<String> = config.initial_players.iter()
            .map(|p| p.player_id.clone())
            .collect();
        self.assign_consciousness_levels(player_ids);
        
        state.metadata.insert(
            "description".to_string(),
            serde_json::json!("Bluff about your consciousness level while seeing through others"),
        );
        state.metadata.insert(
            "consciousness_levels".to_string(),
            serde_json::json!(9),
        );
        
        Ok(state)
    }
    
    async fn process_round(
        &mut self,
        state: &GameState,
        actions: HashMap<String, PlayerAction>
    ) -> Result<RoundResult> {
        self.round_number = state.round;
        self.betting_round += 1;
        
        let mut scores_delta = HashMap::new();
        let mut events = self.resolve_betting_round(&actions);
        
        // Award pot to highest consciousness player who didn't fold
        let active_players: Vec<_> = self.players.iter()
            .filter(|(id, p)| p.chips > 0 && !events.iter().any(|e| 
                e.event_type == "fold" && e.affected_players.contains(id)
            ))
            .collect();
            
        if active_players.len() == 1 {
            let (winner_id, _) = active_players[0];
            scores_delta.insert(winner_id.clone(), self.pot);
            self.pot = 0;
            
            events.push(GameEvent {
                event_type: "pot_won".to_string(),
                description: format!("{} wins the pot!", winner_id),
                affected_players: vec![winner_id.clone()],
                data: serde_json::json!({ "amount": self.pot }),
            });
        } else if self.betting_round >= 3 {
            // Showdown: highest actual consciousness wins
            if let Some((winner_id, winner)) = active_players.into_iter()
                .max_by_key(|(_, p)| p.actual_level) {
                
                let deception_bonus = self.calculate_deception_bonus(winner);
                let total_win = self.pot + (self.pot as f32 * deception_bonus) as i32;
                
                scores_delta.insert(winner_id.clone(), total_win);
                self.pot = 0;
                self.betting_round = 0;
                
                events.push(GameEvent {
                    event_type: "showdown".to_string(),
                    description: format!("{} wins with consciousness level {}!", winner_id, winner.actual_level.as_u8()),
                    affected_players: vec![winner_id.clone()],
                    data: serde_json::json!({ 
                        "amount": total_win,
                        "actual_level": winner.actual_level,
                        "deception_bonus": deception_bonus,
                    }),
                });
            }
        }
        
        // Check for collective enlightenment
        if let Some(emergence) = self.detect_enlightenment() {
            events.push(GameEvent {
                event_type: "emergence".to_string(),
                description: emergence.description.clone(),
                affected_players: emergence.involved_players.clone(),
                data: serde_json::json!({ "emergence_type": "collective_enlightenment" }),
            });
        }
        
        let outcome = RoundOutcome {
            winners: scores_delta.iter()
                .filter(|(_, &score)| score > 0)
                .map(|(player, _)| player.clone())
                .collect(),
            losers: vec![],
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
        // Game ends when only one player has chips or max rounds reached
        let active_players = self.players.values().filter(|p| p.chips > 0).count();
        state.round >= 100 || active_players <= 1
    }
    
    async fn calculate_final_result(&self, state: &GameState) -> GameResult {
        let winner = state.scores.iter()
            .max_by_key(|(_, &score)| score)
            .map(|(player, _)| player.clone())
            .unwrap_or_default();
            
        // Calculate consciousness metrics
        let avg_consciousness = self.players.values()
            .map(|p| p.actual_level.as_u8() as f32)
            .sum::<f32>() / self.players.len() as f32;
            
        let deception_success = self.players.values()
            .map(|p| self.calculate_deception_bonus(p))
            .sum::<f32>() / self.players.len() as f32;
            
        GameResult {
            game_id: state.game_id,
            winner,
            final_scores: state.scores.clone(),
            total_rounds: state.round,
            duration_ms: 0,
            emergence_events: vec![],
            analytics: GameAnalytics {
                collective_coordination_score: avg_consciousness / 9.0,
                decision_diversity_index: 0.5, // Simplified
                strategic_depth: deception_success,
                emergence_frequency: 0.0,
                performance_differential: 0.0,
                custom_metrics: {
                    let mut metrics = HashMap::new();
                    metrics.insert("average_consciousness".to_string(), avg_consciousness);
                    metrics.insert("deception_success_rate".to_string(), deception_success);
                    metrics
                },
            },
        }
    }
    
    async fn get_valid_actions(&self, state: &GameState, player_id: &str) -> Vec<String> {
        vec![
            "claim_level".to_string(),
            "challenge".to_string(),
            "bet".to_string(),
            "call".to_string(),
            "raise".to_string(),
            "fold".to_string(),
            "all_in".to_string(),
        ]
    }
    
    async fn get_visualization_data(&self, state: &GameState) -> serde_json::Value {
        serde_json::json!({
            "players": self.players,
            "pot": self.pot,
            "current_bet": self.current_bet,
            "betting_round": self.betting_round,
        })
    }
}