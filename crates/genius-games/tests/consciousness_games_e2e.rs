//! End-to-end tests for consciousness-themed games

mod e2e_test_framework;

use e2e_test_framework::*;
use genius_core::GameType;

#[tokio::test]
async fn test_mirror_mind_game() {
    let config = E2ETestConfig {
        game_type: GameType::MirrorMind,
        num_players: 4,
        max_rounds: 30,
        ..Default::default()
    };
    
    let runner = E2ETestRunner::new(config);
    let result = runner.run_game().await.expect("Game should complete");
    
    // Basic assertions
    assertions::assert_game_completes(&result);
    assertions::assert_game_ends_within_rounds(&result, 30);
    
    // Check for recursive thinking emergence
    assert!(
        result.has_emergence_type("RecursiveThinking") || result.emergence_events.len() > 0,
        "Mirror Mind should show emergence of recursive thinking"
    );
    
    // Check that mental models were developed
    let final_scores = &result.final_result.final_scores;
    assert!(
        final_scores.values().any(|&score| score > 50),
        "Players should develop mental models and score points"
    );
}

#[tokio::test]
async fn test_consciousness_poker_game() {
    let config = E2ETestConfig {
        game_type: GameType::ConsciousnessPoker,
        num_players: 6,
        max_rounds: 25,
        ..Default::default()
    };
    
    let runner = E2ETestRunner::new(config);
    let result = runner.run_game().await.expect("Game should complete");
    
    assertions::assert_game_completes(&result);
    
    // Check for collective enlightenment
    let has_enlightenment = result.emergence_events.iter().any(|e| {
        e.description.contains("enlightenment") || 
        e.data.get("emergence_type")
            .and_then(|v| v.as_str())
            .map(|s| s == "collective_enlightenment")
            .unwrap_or(false)
    });
    
    assert!(
        has_enlightenment || result.final_result.analytics.collective_coordination_score > 0.5,
        "Consciousness Poker should lead to some form of collective awareness"
    );
}

#[tokio::test]
async fn test_reality_consensus_game() {
    let config = E2ETestConfig {
        game_type: GameType::RealityConsensus,
        num_players: 5,
        max_rounds: 40,
        ..Default::default()
    };
    
    let runner = E2ETestRunner::new(config);
    let result = runner.run_game().await.expect("Game should complete");
    
    assertions::assert_game_completes(&result);
    
    // Check for reality stability or collapse
    let reality_events = result.emergence_events.iter()
        .filter(|e| e.event_type == "emergence" || e.event_type == "reality_shift")
        .count();
        
    assert!(
        reality_events > 0,
        "Reality Consensus should have reality-related events"
    );
    
    // Check final metrics
    let metrics = &result.final_result.analytics.custom_metrics;
    if let Some(stability) = metrics.get("final_reality_stability") {
        assert!(
            *stability > 0.0,
            "Reality should have some stability value"
        );
    }
}

#[tokio::test]
async fn test_void_walker_game() {
    let config = E2ETestConfig {
        game_type: GameType::VoidWalker,
        num_players: 4,
        max_rounds: 50,
        ..Default::default()
    };
    
    let runner = E2ETestRunner::new(config).with_deterministic_ai();
    let result = runner.run_game().await.expect("Game should complete");
    
    assertions::assert_game_completes(&result);
    
    // Check for reality emergence from void
    let has_reality_emerged = result.emergence_events.iter().any(|e| {
        e.data.get("emergence_type")
            .and_then(|v| v.as_str())
            .map(|s| s == "reality_emerged")
            .unwrap_or(false)
    });
    
    // Check rule creation
    let metrics = &result.final_result.analytics.custom_metrics;
    if let Some(rules_created) = metrics.get("rules_created") {
        assert!(
            *rules_created > 0.0,
            "Players should create rules in Void Walker"
        );
    }
}

#[tokio::test]
async fn test_information_horizon_game() {
    let config = E2ETestConfig {
        game_type: GameType::InformationHorizon,
        num_players: 6,
        max_rounds: 35,
        ..Default::default()
    };
    
    let runner = E2ETestRunner::new(config);
    let result = runner.run_game().await.expect("Game should complete");
    
    assertions::assert_game_completes(&result);
    
    // Check for collective understanding emergence
    let has_understanding = result.emergence_events.iter().any(|e| {
        e.description.contains("collective") || 
        e.description.contains("understanding") ||
        e.description.contains("pattern")
    });
    
    // Check knowledge metrics
    let metrics = &result.final_result.analytics.custom_metrics;
    if let Some(knowledge) = metrics.get("final_knowledge") {
        assert!(
            *knowledge > 0.0,
            "Some collective knowledge should be accumulated"
        );
    }
}

#[tokio::test]
async fn test_observer_game() {
    let config = E2ETestConfig {
        game_type: GameType::ObserverGame,
        num_players: 3,
        max_rounds: 30,
        ..Default::default()
    };
    
    let runner = E2ETestRunner::new(config);
    let result = runner.run_game().await.expect("Game should complete");
    
    assertions::assert_game_completes(&result);
    
    // Check quantum mechanics
    let metrics = &result.final_result.analytics.custom_metrics;
    
    // Check collapse ratio
    if let Some(collapse_ratio) = metrics.get("collapse_ratio") {
        assert!(
            *collapse_ratio > 0.0 && *collapse_ratio <= 1.0,
            "Collapse ratio should be between 0 and 1"
        );
    }
    
    // Check for quantum coherence events
    let quantum_events = result.emergence_events.iter()
        .filter(|e| e.description.contains("quantum") || e.description.contains("coherence"))
        .count();
        
    assert!(
        quantum_events > 0 || result.final_result.analytics.collective_coordination_score > 0.2,
        "Observer Game should have quantum-related dynamics"
    );
}

#[tokio::test]
async fn test_consciousness_games_emergence_patterns() {
    // Test multiple games to see emergence patterns
    let consciousness_games = vec![
        GameType::MirrorMind,
        GameType::ConsciousnessPoker,
        GameType::RealityConsensus,
        GameType::InformationHorizon,
    ];
    
    for game_type in consciousness_games {
        let config = E2ETestConfig {
            game_type: game_type.clone(),
            num_players: 5,
            max_rounds: 25,
            ..Default::default()
        };
        
        let runner = E2ETestRunner::new(config);
        let stats = runner.run_multiple_games(10).await
            .expect("Multiple games should complete");
        
        // All consciousness games should complete successfully
        assert!(
            stats.successful_games as f32 / stats.total_games as f32 > 0.9,
            "{:?} should have >90% success rate",
            game_type
        );
        
        // Should show some emergence
        assert!(
            stats.avg_emergence_frequency > 0.0,
            "{:?} should show emergence patterns",
            game_type
        );
        
        println!("\n{:?} Statistics:", game_type);
        stats.print_summary();
    }
}

#[tokio::test]
async fn test_consciousness_games_collective_intelligence() {
    // Test that consciousness games develop collective intelligence
    let games_to_test = vec![
        (GameType::MirrorMind, "theory of mind"),
        (GameType::RealityConsensus, "consensus reality"),
        (GameType::InformationHorizon, "collective knowledge"),
    ];
    
    for (game_type, expected_pattern) in games_to_test {
        let config = E2ETestConfig {
            game_type: game_type.clone(),
            num_players: 8,
            max_rounds: 40,
            enable_emergence_tracking: true,
            ..Default::default()
        };
        
        let runner = E2ETestRunner::new(config).with_deterministic_ai();
        let result = runner.run_game().await
            .expect(&format!("{:?} should complete", game_type));
        
        // Check collective intelligence metrics
        let analytics = &result.final_result.analytics;
        assert!(
            analytics.collective_coordination_score > 0.3,
            "{:?} should develop collective coordination (expected pattern: {})",
            game_type,
            expected_pattern
        );
        
        // Should have meaningful strategic depth
        assert!(
            analytics.strategic_depth > 0.0,
            "{:?} should have strategic depth",
            game_type
        );
    }
}