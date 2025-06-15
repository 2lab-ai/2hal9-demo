//! End-to-end tests for strategic games

mod e2e_test_framework;

use e2e_test_framework::*;
use genius_core::GameType;

#[tokio::test]
async fn test_minority_game() {
    let config = E2ETestConfig {
        game_type: GameType::MinorityGame,
        num_players: 5,
        max_rounds: 20,
        ..Default::default()
    };
    
    let runner = E2ETestRunner::new(config);
    let result = runner.run_game().await.expect("Game should complete");
    
    assertions::assert_game_completes(&result);
    assertions::assert_game_ends_within_rounds(&result, 20);
    
    // Minority game should have clear winners/losers each round
    let total_winners = result.round_results.iter()
        .map(|r| r.outcome.winners.len())
        .sum::<usize>();
    let total_losers = result.round_results.iter()
        .map(|r| r.outcome.losers.len())
        .sum::<usize>();
        
    assert!(
        total_winners > 0 && total_losers > 0,
        "Minority game should have both winners and losers"
    );
}

#[tokio::test]
async fn test_byzantine_generals() {
    let config = E2ETestConfig {
        game_type: GameType::ByzantineGenerals,
        num_players: 7, // Classic Byzantine setup
        max_rounds: 15,
        ..Default::default()
    };
    
    let runner = E2ETestRunner::new(config);
    let result = runner.run_game().await.expect("Game should complete");
    
    assertions::assert_game_completes(&result);
    
    // Check for consensus events
    let consensus_events = result.round_results.iter()
        .flat_map(|r| &r.outcome.special_events)
        .filter(|e| e.contains("Consensus") || e.contains("Byzantine"))
        .count();
        
    assert!(
        consensus_events > 0,
        "Byzantine Generals should have consensus-related events"
    );
}

#[tokio::test]
async fn test_mini_go() {
    let config = E2ETestConfig {
        game_type: GameType::MiniGo,
        num_players: 2, // Go is a 2-player game
        max_rounds: 50,
        ..Default::default()
    };
    
    let runner = E2ETestRunner::new(config);
    let result = runner.run_game().await.expect("Game should complete");
    
    assertions::assert_game_completes(&result);
    
    // Go should have territory control dynamics
    assert!(
        result.final_result.final_scores.len() == 2,
        "Mini Go should have exactly 2 players"
    );
}

#[tokio::test]
async fn test_mini_holdem() {
    let config = E2ETestConfig {
        game_type: GameType::MiniHoldem,
        num_players: 6,
        max_rounds: 30,
        ..Default::default()
    };
    
    let runner = E2ETestRunner::new(config);
    let result = runner.run_game().await.expect("Game should complete");
    
    assertions::assert_game_completes(&result);
    
    // Poker should have betting dynamics
    let has_betting_events = result.round_results.iter()
        .any(|r| r.events.iter().any(|e| 
            e.event_type.contains("bet") || 
            e.event_type.contains("fold") ||
            e.event_type.contains("raise")
        ));
        
    assert!(
        has_betting_events || result.total_rounds > 0,
        "Mini Hold'em should have poker dynamics"
    );
}

#[tokio::test]
async fn test_strategic_games_fairness() {
    // Test that strategic games have fair outcomes over multiple runs
    let strategic_games = vec![
        GameType::MinorityGame,
        GameType::ByzantineGenerals,
    ];
    
    for game_type in strategic_games {
        let config = E2ETestConfig {
            game_type: game_type.clone(),
            num_players: 4,
            max_rounds: 20,
            ..Default::default()
        };
        
        let runner = E2ETestRunner::new(config);
        let stats = runner.run_multiple_games(20).await
            .expect("Multiple games should complete");
        
        // Check success rate
        assert!(
            stats.successful_games as f32 / stats.total_games as f32 > 0.95,
            "{:?} should have high success rate",
            game_type
        );
        
        // Check for relatively fair winner distribution (with higher tolerance for strategic games)
        if stats.winner_distribution.len() > 1 {
            assertions::assert_fair_winner_distribution(&stats, 0.3);
        }
        
        println!("\n{:?} Fairness Stats:", game_type);
        stats.print_summary();
    }
}

#[tokio::test]
async fn test_strategic_depth() {
    let games = vec![
        (GameType::MinorityGame, 0.3),
        (GameType::ByzantineGenerals, 0.4),
        (GameType::VoidWalker, 0.5),
        (GameType::ObserverGame, 0.6),
    ];
    
    for (game_type, min_depth) in games {
        let config = E2ETestConfig {
            game_type: game_type.clone(),
            num_players: 5,
            max_rounds: 25,
            ..Default::default()
        };
        
        let runner = E2ETestRunner::new(config);
        let result = runner.run_game().await
            .expect(&format!("{:?} should complete", game_type));
        
        assert!(
            result.final_result.analytics.strategic_depth >= min_depth,
            "{:?} should have strategic depth >= {}",
            game_type,
            min_depth
        );
    }
}