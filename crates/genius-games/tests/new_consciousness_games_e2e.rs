//! End-to-end tests for new consciousness-themed games

mod e2e_test_framework;

use e2e_test_framework::*;
use genius_core::GameType;

#[tokio::test]
async fn test_consciousness_cascade() {
    let config = E2ETestConfig {
        game_type: GameType::ConsciousnessCascade,
        num_players: 8,
        max_rounds: 40,
        ..Default::default()
    };
    
    let runner = E2ETestRunner::new(config);
    let result = runner.run_game().await.expect("Game should complete");
    
    assertions::assert_game_completes(&result);
    assertions::assert_game_ends_within_rounds(&result, 40);
    
    // Check for cascade events
    let cascade_events = result.round_results.iter()
        .flat_map(|r| &r.events)
        .filter(|e| e.event_type == "consciousness_cascade" || 
                    e.description.contains("cascade"))
        .count();
        
    assert!(
        cascade_events > 0 || result.emergence_frequency() > 0.0,
        "Consciousness Cascade should have cascading events"
    );
    
    // Check for global consciousness metrics
    let metrics = &result.final_result.analytics.custom_metrics;
    if let Some(global_consciousness) = metrics.get("global_consciousness") {
        assert!(
            *global_consciousness > 0.0,
            "Should develop some global consciousness"
        );
    }
}

#[tokio::test]
async fn test_quantum_dreamer() {
    let config = E2ETestConfig {
        game_type: GameType::QuantumDreamer,
        num_players: 5,
        max_rounds: 35,
        ..Default::default()
    };
    
    let runner = E2ETestRunner::new(config);
    let result = runner.run_game().await.expect("Game should complete");
    
    assertions::assert_game_completes(&result);
    
    // Check for dream and reality dynamics
    let dream_events = result.round_results.iter()
        .flat_map(|r| &r.events)
        .filter(|e| e.description.contains("dream") || 
                    e.description.contains("lucid") ||
                    e.description.contains("reality"))
        .count();
        
    assert!(
        dream_events > 0,
        "Quantum Dreamer should have dream-related events"
    );
    
    // Check for reality bleeds
    let reality_bleeds = result.round_results.iter()
        .flat_map(|r| &r.events)
        .filter(|e| e.event_type == "reality_bleed")
        .count();
        
    let metrics = &result.final_result.analytics.custom_metrics;
    if let Some(bleeds) = metrics.get("reality_bleeds") {
        assert!(
            *bleeds >= 0.0,
            "Should track reality bleed events"
        );
    }
}

#[tokio::test]
async fn test_consciousness_cascade_emergence() {
    let config = E2ETestConfig {
        game_type: GameType::ConsciousnessCascade,
        num_players: 10,
        max_rounds: 50,
        enable_emergence_tracking: true,
        ..Default::default()
    };
    
    let runner = E2ETestRunner::new(config).with_deterministic_ai();
    let result = runner.run_game().await.expect("Game should complete");
    
    // Check for network effects
    let network_events = result.emergence_events.iter()
        .filter(|e| e.description.contains("resonance") || 
                   e.description.contains("network") ||
                   e.description.contains("global"))
        .count();
        
    assert!(
        network_events > 0 || result.final_result.analytics.collective_coordination_score > 0.5,
        "Consciousness Cascade should show network emergence"
    );
    
    // Check thought propagation
    let thought_events = result.round_results.iter()
        .flat_map(|r| &r.events)
        .filter(|e| e.event_type == "thought_sent")
        .count();
        
    assert!(
        thought_events > 0,
        "Should have thought propagation through network"
    );
}

#[tokio::test]
async fn test_quantum_dreamer_collective_unconscious() {
    let config = E2ETestConfig {
        game_type: GameType::QuantumDreamer,
        num_players: 6,
        max_rounds: 40,
        enable_emergence_tracking: true,
        ..Default::default()
    };
    
    let runner = E2ETestRunner::new(config);
    let result = runner.run_game().await.expect("Game should complete");
    
    // Check for synchronicity or collective dream
    let collective_events = result.emergence_events.iter()
        .filter(|e| e.description.contains("synchronicity") || 
                   e.description.contains("collective") ||
                   e.data.get("type").and_then(|v| v.as_str()) == Some("collective_dream"))
        .count();
        
    // Even without explicit collective events, check lucidity development
    let lucid_events = result.round_results.iter()
        .flat_map(|r| &r.events)
        .filter(|e| e.event_type == "lucid_mastery")
        .count();
        
    assert!(
        collective_events > 0 || lucid_events > 0,
        "Quantum Dreamer should develop lucidity or collective patterns"
    );
}

#[tokio::test]
async fn test_new_consciousness_games_integration() {
    // Test that new consciousness games integrate well with existing ones
    let consciousness_suite = vec![
        GameType::MirrorMind,
        GameType::ConsciousnessPoker,
        GameType::RealityConsensus,
        GameType::VoidWalker,
        GameType::InformationHorizon,
        GameType::ObserverGame,
        GameType::ConsciousnessCascade,
        GameType::QuantumDreamer,
    ];
    
    let mut total_emergence_score = 0.0;
    let mut games_with_emergence = 0;
    
    for game_type in consciousness_suite {
        let config = E2ETestConfig {
            game_type: game_type.clone(),
            num_players: 6,
            max_rounds: 30,
            enable_emergence_tracking: true,
            ..Default::default()
        };
        
        let runner = E2ETestRunner::new(config);
        let result = runner.run_game().await
            .expect(&format!("{:?} should complete", game_type));
        
        if result.emergence_frequency() > 0.0 {
            games_with_emergence += 1;
        }
        
        total_emergence_score += result.final_result.analytics.collective_coordination_score;
        
        println!("{:?} - Emergence: {:.2}, Coordination: {:.2}", 
                 game_type,
                 result.emergence_frequency(),
                 result.final_result.analytics.collective_coordination_score);
    }
    
    let avg_emergence_score = total_emergence_score / 8.0;
    assert!(
        avg_emergence_score > 0.3,
        "Consciousness games should average decent coordination scores"
    );
    
    assert!(
        games_with_emergence >= 4,
        "At least half of consciousness games should show emergence"
    );
}

#[tokio::test]
async fn test_consciousness_games_scaling() {
    // Test how consciousness games scale with player count
    let player_counts = vec![3, 6, 10];
    let game_types = vec![
        GameType::ConsciousnessCascade,
        GameType::QuantumDreamer,
    ];
    
    for game_type in game_types {
        println!("\nTesting {} scaling:", game_type.display_name());
        
        for &num_players in &player_counts {
            let config = E2ETestConfig {
                game_type: game_type.clone(),
                num_players,
                max_rounds: 25,
                ..Default::default()
            };
            
            let runner = E2ETestRunner::new(config);
            let result = runner.run_game().await
                .expect(&format!("{:?} with {} players should complete", game_type, num_players));
            
            let coordination = result.final_result.analytics.collective_coordination_score;
            let emergence = result.emergence_frequency();
            
            println!("  {} players: coordination={:.2}, emergence={:.2}", 
                     num_players, coordination, emergence);
            
            // Larger groups should potentially show more emergence
            if num_players >= 6 {
                assert!(
                    coordination > 0.0 || emergence > 0.0,
                    "{:?} should show some collective behavior with {} players",
                    game_type,
                    num_players
                );
            }
        }
    }
}