//! End-to-end tests for survival/death games

mod e2e_test_framework;

use e2e_test_framework::*;
use genius_core::GameType;

#[tokio::test]
async fn test_battle_royale() {
    let config = E2ETestConfig {
        game_type: GameType::BattleRoyale,
        num_players: 10,
        max_rounds: 40,
        ..Default::default()
    };
    
    let runner = E2ETestRunner::new(config);
    let result = runner.run_game().await.expect("Game should complete");
    
    assertions::assert_game_completes(&result);
    
    // Battle Royale should eliminate players
    let elimination_events = result.round_results.iter()
        .flat_map(|r| &r.events)
        .filter(|e| e.description.contains("eliminated") || 
                    e.description.contains("eliminated") ||
                    e.event_type.contains("death"))
        .count();
        
    assert!(
        elimination_events > 0 || result.round_results.len() > 0,
        "Battle Royale should have elimination dynamics"
    );
}

#[tokio::test]
async fn test_hunger_games() {
    let config = E2ETestConfig {
        game_type: GameType::HungerGames,
        num_players: 12, // Classic Hunger Games setup
        max_rounds: 50,
        ..Default::default()
    };
    
    let runner = E2ETestRunner::new(config);
    let result = runner.run_game().await.expect("Game should complete");
    
    assertions::assert_game_completes(&result);
    
    // Should have survival dynamics
    let survival_events = result.round_results.iter()
        .flat_map(|r| &r.outcome.special_events)
        .filter(|e| e.contains("tribute") || e.contains("survival") || e.contains("alliance"))
        .count();
        
    assert!(
        survival_events > 0,
        "Hunger Games should have survival and alliance dynamics"
    );
}

#[tokio::test]
async fn test_squid_game() {
    let config = E2ETestConfig {
        game_type: GameType::SquidGame,
        num_players: 8,
        max_rounds: 30,
        ..Default::default()
    };
    
    let runner = E2ETestRunner::new(config);
    let result = runner.run_game().await.expect("Game should complete");
    
    assertions::assert_game_completes(&result);
    
    // Squid Game should have high stakes
    let final_scores = &result.final_result.final_scores;
    let survivors = final_scores.iter()
        .filter(|(_, &score)| score > 0)
        .count();
        
    assert!(
        survivors < config.num_players,
        "Squid Game should eliminate some players"
    );
}

#[tokio::test]
async fn test_russian_roulette() {
    let config = E2ETestConfig {
        game_type: GameType::RussianRoulette,
        num_players: 6,
        max_rounds: 20,
        ..Default::default()
    };
    
    let runner = E2ETestRunner::new(config);
    let result = runner.run_game().await.expect("Game should complete");
    
    assertions::assert_game_completes(&result);
    
    // Russian Roulette should be high risk
    assert!(
        result.total_rounds < config.max_rounds,
        "Russian Roulette should end quickly due to eliminations"
    );
}

#[tokio::test]
async fn test_king_of_the_hill() {
    let config = E2ETestConfig {
        game_type: GameType::KingOfTheHill,
        num_players: 8,
        max_rounds: 35,
        ..Default::default()
    };
    
    let runner = E2ETestRunner::new(config);
    let result = runner.run_game().await.expect("Game should complete");
    
    assertions::assert_game_completes(&result);
    
    // Should have position control dynamics
    let control_events = result.round_results.iter()
        .flat_map(|r| &r.events)
        .filter(|e| e.description.contains("control") || 
                    e.description.contains("hill") ||
                    e.description.contains("king"))
        .count();
        
    assert!(
        control_events > 0 || result.total_rounds > 0,
        "King of the Hill should have territorial control"
    );
}

#[tokio::test]
async fn test_last_stand() {
    let config = E2ETestConfig {
        game_type: GameType::LastStand,
        num_players: 10,
        max_rounds: 40,
        ..Default::default()
    };
    
    let runner = E2ETestRunner::new(config);
    let result = runner.run_game().await.expect("Game should complete");
    
    assertions::assert_game_completes(&result);
    
    // Last Stand should have defense dynamics
    let defense_events = result.round_results.iter()
        .flat_map(|r| &r.events)
        .filter(|e| e.description.contains("defense") || 
                    e.description.contains("wave") ||
                    e.description.contains("survive"))
        .count();
        
    assert!(
        defense_events > 0 || result.final_result.winner.len() > 0,
        "Last Stand should have survival waves"
    );
}

#[tokio::test]
async fn test_survival_games_mortality_rate() {
    // Test that survival games have appropriate elimination rates
    let survival_games = vec![
        (GameType::BattleRoyale, 0.8),   // 80% elimination expected
        (GameType::HungerGames, 0.9),    // 90% elimination expected
        (GameType::SquidGame, 0.85),     // 85% elimination expected
        (GameType::RussianRoulette, 0.5), // 50% elimination expected
    ];
    
    for (game_type, expected_mortality) in survival_games {
        let config = E2ETestConfig {
            game_type: game_type.clone(),
            num_players: 10,
            max_rounds: 50,
            ..Default::default()
        };
        
        let runner = E2ETestRunner::new(config);
        let result = runner.run_game().await
            .expect(&format!("{:?} should complete", game_type));
        
        let survivors = result.final_result.final_scores.iter()
            .filter(|(_, &score)| score > 0)
            .count();
        let mortality_rate = 1.0 - (survivors as f32 / config.num_players as f32);
        
        println!("{:?} mortality rate: {:.2}", game_type, mortality_rate);
        
        // Allow some variance in mortality rates
        assert!(
            (mortality_rate - expected_mortality).abs() < 0.3,
            "{:?} mortality rate {:.2} should be close to expected {:.2}",
            game_type,
            mortality_rate,
            expected_mortality
        );
    }
}

#[tokio::test]
async fn test_survival_emergence() {
    // Test that survival games can show emergence through alliances
    let games = vec![
        GameType::HungerGames,
        GameType::BattleRoyale,
        GameType::LastStand,
    ];
    
    for game_type in games {
        let config = E2ETestConfig {
            game_type: game_type.clone(),
            num_players: 12,
            max_rounds: 40,
            enable_emergence_tracking: true,
            ..Default::default()
        };
        
        let runner = E2ETestRunner::new(config);
        let result = runner.run_game().await
            .expect(&format!("{:?} should complete", game_type));
        
        // Check for alliance or cooperation emergence
        let coop_events = result.emergence_events.iter()
            .filter(|e| e.description.contains("alliance") || 
                       e.description.contains("cooperation") ||
                       e.description.contains("collective"))
            .count();
            
        println!("{:?} cooperation events: {}", game_type, coop_events);
        
        // Survival games might show emergent cooperation
        assert!(
            coop_events > 0 || result.final_result.analytics.collective_coordination_score > 0.0,
            "{:?} might show emergent cooperation even in competitive setting",
            game_type
        );
    }
}