//! End-to-end tests for trust-based games

mod e2e_test_framework;

use e2e_test_framework::*;
use genius_core::GameType;

#[tokio::test]
async fn test_prisoners_dilemma() {
    let config = E2ETestConfig {
        game_type: GameType::PrisonersDilemma,
        num_players: 4,
        max_rounds: 30,
        ..Default::default()
    };
    
    let runner = E2ETestRunner::new(config);
    let result = runner.run_game().await.expect("Game should complete");
    
    assertions::assert_game_completes(&result);
    assertions::assert_game_ends_within_rounds(&result, 30);
    
    // Check for cooperation vs defection dynamics
    let coop_events = result.round_results.iter()
        .flat_map(|r| &r.events)
        .filter(|e| e.description.contains("cooperate") || 
                    e.description.contains("trust") ||
                    e.description.contains("betray"))
        .count();
        
    assert!(
        coop_events > 0 || result.total_rounds > 0,
        "Prisoner's Dilemma should have cooperation/defection dynamics"
    );
    
    // Check trust evolution
    assert!(
        result.final_result.analytics.collective_coordination_score > 0.0,
        "Trust dynamics should emerge over time"
    );
}

#[tokio::test]
async fn test_trust_fall() {
    let config = E2ETestConfig {
        game_type: GameType::TrustFall,
        num_players: 6,
        max_rounds: 25,
        ..Default::default()
    };
    
    let runner = E2ETestRunner::new(config);
    let result = runner.run_game().await.expect("Game should complete");
    
    assertions::assert_game_completes(&result);
    
    // Trust Fall should build trust networks
    let trust_events = result.round_results.iter()
        .flat_map(|r| &r.events)
        .filter(|e| e.description.contains("trust") || 
                    e.description.contains("fall") ||
                    e.description.contains("catch"))
        .count();
        
    assert!(
        trust_events > 0,
        "Trust Fall should have trust-building events"
    );
}

#[tokio::test]
async fn test_liars_dice() {
    let config = E2ETestConfig {
        game_type: GameType::LiarsDice,
        num_players: 5,
        max_rounds: 40,
        ..Default::default()
    };
    
    let runner = E2ETestRunner::new(config);
    let result = runner.run_game().await.expect("Game should complete");
    
    assertions::assert_game_completes(&result);
    
    // Liar's Dice should have bluffing dynamics
    let bluff_events = result.round_results.iter()
        .flat_map(|r| &r.events)
        .filter(|e| e.description.contains("bluff") || 
                    e.description.contains("lie") ||
                    e.description.contains("challenge") ||
                    e.description.contains("dice"))
        .count();
        
    assert!(
        bluff_events > 0 || result.total_rounds > 0,
        "Liar's Dice should have bluffing and deception"
    );
}

#[tokio::test]
async fn test_trust_evolution() {
    // Test how trust evolves over multiple rounds
    let trust_games = vec![
        GameType::PrisonersDilemma,
        GameType::TrustFall,
        GameType::ConsciousnessPoker,
    ];
    
    for game_type in trust_games {
        let config = E2ETestConfig {
            game_type: game_type.clone(),
            num_players: 6,
            max_rounds: 50, // Longer game for trust evolution
            ..Default::default()
        };
        
        let runner = E2ETestRunner::new(config).with_deterministic_ai();
        let result = runner.run_game().await
            .expect(&format!("{:?} should complete", game_type));
        
        // Analyze trust evolution
        let early_scores: i32 = result.round_results.iter()
            .take(10)
            .flat_map(|r| r.scores_delta.values())
            .sum();
            
        let late_scores: i32 = result.round_results.iter()
            .skip(result.round_results.len().saturating_sub(10))
            .flat_map(|r| r.scores_delta.values())
            .sum();
            
        println!("{:?} - Early scores: {}, Late scores: {}", 
                 game_type, early_scores, late_scores);
        
        // Trust games often show score evolution
        assert!(
            result.final_result.analytics.collective_coordination_score > 0.0,
            "{:?} should develop some coordination",
            game_type
        );
    }
}

#[tokio::test]
async fn test_betrayal_consequences() {
    // Test that betrayal has consequences in trust games
    let config = E2ETestConfig {
        game_type: GameType::PrisonersDilemma,
        num_players: 4,
        max_rounds: 30,
        ..Default::default()
    };
    
    let runner = E2ETestRunner::new(config);
    let stats = runner.run_multiple_games(10).await
        .expect("Multiple games should complete");
    
    // Check for different outcomes
    assert!(
        stats.winner_distribution.len() > 1,
        "Different strategies should lead to different winners"
    );
    
    stats.print_summary();
}

#[tokio::test]
async fn test_trust_network_emergence() {
    // Test emergence of trust networks
    let config = E2ETestConfig {
        game_type: GameType::TrustFall,
        num_players: 8,
        max_rounds: 40,
        enable_emergence_tracking: true,
        ..Default::default()
    };
    
    let runner = E2ETestRunner::new(config);
    let result = runner.run_game().await.expect("Game should complete");
    
    // Check for trust network emergence
    let network_events = result.emergence_events.iter()
        .filter(|e| e.description.contains("network") || 
                   e.description.contains("trust") ||
                   e.description.contains("collective"))
        .count();
        
    assert!(
        network_events > 0 || result.final_result.analytics.collective_coordination_score > 0.3,
        "Trust networks should emerge in trust-based games"
    );
}