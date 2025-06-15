//! Demo scenarios showcasing emergence patterns in consciousness games

use genius_core::{GameType, GameConfig, Player, PlayerType, PlayerAction};
use genius_games::create_game;
use genius_ai::providers::mock::MockProvider;
use std::collections::HashMap;
use tokio::time::{sleep, Duration};

/// Demonstrates cascading consciousness in the Consciousness Cascade game
pub async fn demo_consciousness_cascade() -> anyhow::Result<()> {
    println!("\n=== CONSCIOUSNESS CASCADE DEMO ===");
    println!("Watch as thoughts propagate and cascade through the network of minds...\n");
    
    let mut game = create_game(GameType::ConsciousnessCascade)?;
    
    // Create a network of 8 interconnected minds
    let players: Vec<Player> = (0..8)
        .map(|i| Player {
            id: genius_core::player::PlayerId::from_string(format!("mind_{}", i)),
            name: format!("Conscious Mind {}", i),
            player_type: PlayerType::AI {
                provider: "consciousness".to_string(),
                model: "cascade".to_string(),
            },
            metadata: serde_json::json!({
                "resonance_frequency": i as f32 * 1.5,
            }),
        })
        .collect();
    
    let config = GameConfig {
        game_type: GameType::ConsciousnessCascade,
        rounds: 20,
        time_limit_ms: 5000,
        special_rules: HashMap::new(),
        initial_players: players.clone(),
    };
    
    let mut state = game.initialize(config).await?;
    
    // Run demonstration rounds
    for round in 1..=10 {
        println!("\n--- Round {} ---", round);
        state.round = round;
        
        let mut actions = HashMap::new();
        
        // Different minds take different actions based on round
        match round {
            1..=3 => {
                // Early rounds: establish connections
                for (i, player) in players.iter().enumerate() {
                    let target = players[(i + 1) % players.len()].id.to_string();
                    actions.insert(
                        player.id.to_string(),
                        PlayerAction::new(
                            player.id.to_string(),
                            "cascade_action".to_string(),
                            serde_json::json!({
                                "action_type": "OpenChannel",
                                "target_players": [target],
                            }),
                        ),
                    );
                }
                println!("🔗 Minds establishing neural connections...");
            }
            4..=6 => {
                // Mid rounds: send thoughts
                for (i, player) in players.iter().enumerate() {
                    if i % 2 == 0 {
                        actions.insert(
                            player.id.to_string(),
                            PlayerAction::new(
                                player.id.to_string(),
                                "cascade_action".to_string(),
                                serde_json::json!({
                                    "action_type": "SendThought",
                                    "thought_content": format!("Insight #{}: Unity emerges from diversity", i),
                                }),
                            ),
                        );
                    } else {
                        actions.insert(
                            player.id.to_string(),
                            PlayerAction::new(
                                player.id.to_string(),
                                "cascade_action".to_string(),
                                serde_json::json!({
                                    "action_type": "Resonate",
                                    "frequency_adjustment": 0.1,
                                }),
                            ),
                        );
                    }
                }
                println!("💭 Thoughts flowing through the network...");
            }
            7..=10 => {
                // Late rounds: amplify and merge
                for player in &players {
                    actions.insert(
                        player.id.to_string(),
                        PlayerAction::new(
                            player.id.to_string(),
                            "cascade_action".to_string(),
                            serde_json::json!({
                                "action_type": if round % 2 == 0 { "Amplify" } else { "Merge" },
                            }),
                        ),
                    );
                }
                println!("🌊 Consciousness amplifying and merging...");
            }
            _ => {}
        }
        
        let result = game.process_round(&state, actions).await?;
        
        // Display key events
        for event in &result.events {
            match event.event_type.as_str() {
                "consciousness_cascade" => {
                    println!("⚡ CASCADE EVENT: {}", event.description);
                    println!("   Affected minds: {:?}", event.affected_players.len());
                }
                "thought_sent" => {
                    println!("💭 {}", event.description);
                }
                "emergence" => {
                    println!("🌟 EMERGENCE: {}", event.description);
                }
                _ => {}
            }
        }
        
        // Update state
        state.apply_score_deltas(&result.scores_delta);
        state.history.push(result);
        
        // Show global consciousness level
        if let Some(viz) = game.get_visualization_data(&state).await.as_object() {
            if let Some(global) = viz.get("global_consciousness").and_then(|v| v.as_f64()) {
                println!("\n📊 Global Consciousness Level: {:.2}%", global * 100.0);
            }
        }
        
        sleep(Duration::from_millis(500)).await;
    }
    
    println!("\n=== FINAL EMERGENCE ===");
    let final_result = game.calculate_final_result(&state).await;
    println!("🏆 Most Connected Mind: {}", final_result.winner);
    println!("🌐 Collective Coordination: {:.2}", 
             final_result.analytics.collective_coordination_score);
    
    Ok(())
}

/// Demonstrates reality manipulation in Quantum Dreamer
pub async fn demo_quantum_dreamer() -> anyhow::Result<()> {
    println!("\n=== QUANTUM DREAMER DEMO ===");
    println!("Enter the dreamscape where reality and imagination blur...\n");
    
    let mut game = create_game(GameType::QuantumDreamer)?;
    
    // Create dreamers with different specialties
    let dreamers = vec![
        ("Lucid Explorer", "lucidity"),
        ("Symbol Weaver", "symbols"),
        ("Reality Bender", "manipulation"),
        ("Dream Walker", "navigation"),
        ("Void Seer", "void"),
    ];
    
    let players: Vec<Player> = dreamers.iter()
        .map(|(name, specialty)| Player {
            id: genius_core::player::PlayerId::from_string(specialty.to_string()),
            name: name.to_string(),
            player_type: PlayerType::AI {
                provider: "dream".to_string(),
                model: specialty.to_string(),
            },
            metadata: serde_json::json!({
                "dream_specialty": specialty,
            }),
        })
        .collect();
    
    let config = GameConfig {
        game_type: GameType::QuantumDreamer,
        rounds: 15,
        time_limit_ms: 5000,
        special_rules: HashMap::new(),
        initial_players: players.clone(),
    };
    
    let mut state = game.initialize(config).await?;
    
    // Run dream sequence
    for round in 1..=8 {
        println!("\n--- Dream Cycle {} ---", round);
        state.round = round;
        
        let mut actions = HashMap::new();
        
        // Each dreamer takes actions based on their specialty
        for player in &players {
            let specialty = player.metadata.get("dream_specialty")
                .and_then(|v| v.as_str())
                .unwrap_or("none");
                
            let action = match (specialty, round) {
                ("lucidity", _) => {
                    serde_json::json!({
                        "action_type": "LucidControl",
                    })
                }
                ("symbols", r) if r <= 4 => {
                    let symbol = match r {
                        1 => "Door",
                        2 => "Mirror", 
                        3 => "Light",
                        _ => "Key",
                    };
                    serde_json::json!({
                        "action_type": "CreateSymbol",
                        "target_dream": format!("dream_{}_0", player.id),
                        "symbol": symbol,
                    })
                }
                ("manipulation", r) if r > 2 => {
                    serde_json::json!({
                        "action_type": "ManipulateDream",
                        "target_dream": format!("dream_{}_0", player.id),
                    })
                }
                ("navigation", r) if r % 2 == 0 => {
                    serde_json::json!({
                        "action_type": "DreamWalk",
                    })
                }
                ("void", r) if r > 5 => {
                    serde_json::json!({
                        "action_type": "CollapseDream",
                        "target_dream": format!("dream_{}_0", player.id),
                    })
                }
                _ => {
                    serde_json::json!({
                        "action_type": "ShareDream",
                        "target_dream": format!("dream_{}_0", player.id),
                        "other_dreamers": [players[0].id.to_string()],
                    })
                }
            };
            
            actions.insert(
                player.id.to_string(),
                PlayerAction::new(
                    player.id.to_string(),
                    "dream_action".to_string(),
                    action,
                ),
            );
        }
        
        println!("🌙 Dreamers manipulating the dreamscape...");
        
        let result = game.process_round(&state, actions).await?;
        
        // Display dream events
        for event in &result.events {
            match event.event_type.as_str() {
                "symbol_created" => {
                    println!("✨ {}", event.description);
                }
                "reality_bleed" => {
                    println!("🌀 REALITY BLEED: {}", event.description);
                }
                "dream_collapsed" => {
                    println!("💥 {}", event.description);
                }
                "lucid_mastery" => {
                    println!("👁️ {}", event.description);
                }
                "emergence" => {
                    println!("🎭 EMERGENCE: {}", event.description);
                }
                _ => {}
            }
        }
        
        state.apply_score_deltas(&result.scores_delta);
        state.history.push(result);
        
        // Show reality stability
        if let Some(viz) = game.get_visualization_data(&state).await.as_object() {
            if let Some(stability) = viz.get("reality_stability").and_then(|v| v.as_f64()) {
                println!("\n🌍 Reality Stability: {:.2}%", stability * 100.0);
            }
        }
        
        sleep(Duration::from_millis(500)).await;
    }
    
    println!("\n=== DREAM CONCLUSION ===");
    let final_result = game.calculate_final_result(&state).await;
    println!("🏆 Master Dreamer: {}", final_result.winner);
    
    if let Some(metrics) = final_result.analytics.custom_metrics.get("avg_lucidity") {
        println!("🧠 Average Lucidity Achieved: {:.2}%", metrics * 100.0);
    }
    
    Ok(())
}

/// Demonstrates collective reality shaping
pub async fn demo_reality_consensus() -> anyhow::Result<()> {
    println!("\n=== REALITY CONSENSUS DEMO ===");
    println!("Watch as collective belief shapes and reshapes reality...\n");
    
    let mut game = create_game(GameType::RealityConsensus)?;
    
    let players: Vec<Player> = (0..6)
        .map(|i| Player {
            id: genius_core::player::PlayerId::from_string(format!("believer_{}", i)),
            name: format!("Reality Shaper {}", i),
            player_type: PlayerType::AI {
                provider: "consensus".to_string(),
                model: "belief".to_string(),
            },
            metadata: serde_json::json!({}),
        })
        .collect();
    
    let config = GameConfig {
        game_type: GameType::RealityConsensus,
        rounds: 12,
        time_limit_ms: 5000,
        special_rules: HashMap::new(),
        initial_players: players.clone(),
    };
    
    let mut state = game.initialize(config).await?;
    
    // Reality fragments to vote on
    let reality_fragments = vec![
        ("gravity", vec![true, true, false, true, true, false]),
        ("time", vec![true, false, true, true, false, true]),
        ("causality", vec![false, true, true, false, true, true]),
    ];
    
    for round in 1..=6 {
        println!("\n--- Reality Consensus Round {} ---", round);
        state.round = round;
        
        let mut actions = HashMap::new();
        
        // Each player votes on reality fragments
        for (i, player) in players.iter().enumerate() {
            let fragment_idx = (round - 1) % reality_fragments.len();
            let (fragment_id, votes) = &reality_fragments[fragment_idx];
            let believes = votes[i];
            
            actions.insert(
                player.id.to_string(),
                PlayerAction::new(
                    player.id.to_string(),
                    "belief_action".to_string(),
                    serde_json::json!({
                        "action_type": "Believe",
                        "fragment_id": fragment_id,
                        "believes": believes,
                        "conviction_strength": if believes { 0.9 } else { 0.3 },
                    }),
                ),
            );
        }
        
        let result = game.process_round(&state, actions).await?;
        
        // Display consensus events
        for event in &result.events {
            match event.event_type.as_str() {
                "reality_shift" => {
                    println!("🔄 {}", event.description);
                }
                "reality_glitch" => {
                    println!("⚠️ GLITCH: {}", event.description);
                }
                "emergence" => {
                    println!("🌟 {}", event.description);
                }
                _ => {}
            }
        }
        
        state.apply_score_deltas(&result.scores_delta);
        state.history.push(result);
        
        sleep(Duration::from_millis(300)).await;
    }
    
    println!("\n=== CONSENSUS ACHIEVED ===");
    let final_result = game.calculate_final_result(&state).await;
    println!("🌍 Final Reality Stability: {:.2}", 
             final_result.analytics.collective_coordination_score);
    
    Ok(())
}

/// Main demo runner
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    println!("🧠 GENIUS GAMES: CONSCIOUSNESS EMERGENCE SHOWCASE 🧠");
    println!("===================================================");
    
    // Run each demo
    demo_consciousness_cascade().await?;
    
    println!("\n\nPress Enter to continue to Quantum Dreamer demo...");
    let mut input = String::new();
    std::io::stdin().read_line(&mut input)?;
    
    demo_quantum_dreamer().await?;
    
    println!("\n\nPress Enter to continue to Reality Consensus demo...");
    input.clear();
    std::io::stdin().read_line(&mut input)?;
    
    demo_reality_consensus().await?;
    
    println!("\n\n🎭 EMERGENCE SHOWCASE COMPLETE 🎭");
    println!("These demonstrations show how collective intelligence");
    println!("and consciousness can emerge from simple game rules.");
    
    Ok(())
}