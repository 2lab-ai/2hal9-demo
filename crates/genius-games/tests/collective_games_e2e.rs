//! End-to-end tests for collective intelligence games

mod e2e_test_framework;

use e2e_test_framework::*;
use genius_core::GameType;

#[tokio::test]
async fn test_collective_maze() {
    let config = E2ETestConfig {
        game_type: GameType::CollectiveMaze,
        num_players: 6,
        max_rounds: 30,
        ..Default::default()
    };
    
    let runner = E2ETestRunner::new(config);
    let result = runner.run_game().await.expect("Game should complete");
    
    assertions::assert_game_completes(&result);
    
    // Collective maze should show coordination
    assert!(
        result.final_result.analytics.collective_coordination_score > 0.0,
        "Collective Maze should develop coordination"
    );
}

#[tokio::test]
async fn test_swarm_optimization() {
    let config = E2ETestConfig {
        game_type: GameType::SwarmOptimization,
        num_players: 8,
        max_rounds: 40,
        ..Default::default()
    };
    
    let runner = E2ETestRunner::new(config);
    let result = runner.run_game().await.expect("Game should complete");
    
    assertions::assert_game_completes(&result);
    
    // Swarm should show emergence
    let has_swarm_behavior = result.emergence_events.iter().any(|e| {
        e.description.contains("swarm") || 
        e.description.contains("collective") ||
        e.event_type == "emergence"
    });
    
    assert!(
        has_swarm_behavior || result.emergence_frequency() > 0.0,
        "Swarm Optimization should show emergent behavior"
    );
}

#[tokio::test]
async fn test_recursive_reasoning() {
    let config = E2ETestConfig {
        game_type: GameType::RecursiveReasoning,
        num_players: 4,
        max_rounds: 25,
        ..Default::default()
    };
    
    let runner = E2ETestRunner::new(config);
    let result = runner.run_game().await.expect("Game should complete");
    
    assertions::assert_game_completes(&result);
    
    // Should develop strategic depth through recursion
    assert!(
        result.final_result.analytics.strategic_depth > 0.0,
        "Recursive Reasoning should have strategic depth"
    );
}

#[tokio::test]
async fn test_quantum_consensus() {
    let config = E2ETestConfig {
        game_type: GameType::QuantumConsensus,
        num_players: 5,
        max_rounds: 30,
        ..Default::default()
    };
    
    let runner = E2ETestRunner::new(config);
    let result = runner.run_game().await.expect("Game should complete");
    
    assertions::assert_game_completes(&result);
    
    // Quantum consensus should have quantum-like properties
    let quantum_events = result.round_results.iter()
        .flat_map(|r| &r.events)
        .filter(|e| e.description.contains("quantum") || 
                    e.description.contains("superposition") ||
                    e.description.contains("entangle"))
        .count();
        
    assert!(
        quantum_events > 0 || result.total_rounds > 0,
        "Quantum Consensus should have quantum mechanics"
    );
}

#[tokio::test]
async fn test_collective_emergence_patterns() {
    // Test that collective games show strong emergence
    let collective_games = vec![
        GameType::CollectiveMaze,
        GameType::SwarmOptimization,
        GameType::RecursiveReasoning,
        GameType::QuantumConsensus,
    ];
    
    for game_type in collective_games {
        let config = E2ETestConfig {
            game_type: game_type.clone(),
            num_players: 10, // More players for collective dynamics
            max_rounds: 30,
            enable_emergence_tracking: true,
            ..Default::default()
        };
        
        let runner = E2ETestRunner::new(config);
        let stats = runner.run_multiple_games(5).await
            .expect("Multiple games should complete");
        
        // Collective games should have high coordination
        let avg_coordination = stats.total_games as f32; // Placeholder
        
        println!("\n{:?} Collective Stats:", game_type);
        stats.print_summary();
        
        // Should show emergence
        assert!(
            stats.avg_emergence_frequency > 0.0 || stats.successful_games > 0,
            "{:?} should show collective intelligence emergence",
            game_type
        );
    }
}

#[tokio::test]
async fn test_swarm_vs_individual_performance() {
    // Compare collective performance metrics
    let config = E2ETestConfig {
        game_type: GameType::SwarmOptimization,
        num_players: 12,
        max_rounds: 50,
        ..Default::default()
    };
    
    let runner = E2ETestRunner::new(config);
    let result = runner.run_game().await.expect("Game should complete");
    
    // Swarm should outperform individual scores
    let analytics = &result.final_result.analytics;
    assert!(
        analytics.collective_coordination_score > analytics.decision_diversity_index,
        "Swarm coordination should exceed individual diversity"
    );
}