use axum::{
    extract::{Path, State},
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use genius_core::{GameConfig, GameType, GameState, PlayerAction};
use genius_engine::GameEngine;
use genius_games::create_game;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, sync::Arc};
use tokio::sync::RwLock;
use tower_http::cors::CorsLayer;
use tower_http::services::{ServeDir, ServeFile};
use uuid::Uuid;

pub struct SimpleGameServer {
    games: Arc<RwLock<HashMap<Uuid, GameState>>>,
    engine: Arc<GameEngine>,
}

impl SimpleGameServer {
    pub fn new() -> Self {
        Self {
            games: Arc::new(RwLock::new(HashMap::new())),
            engine: Arc::new(GameEngine::new()),
        }
    }

    pub async fn run(self, addr: &str) -> anyhow::Result<()> {
        // Serve static files
        let static_dir = ServeDir::new("crates/genius-server/static")
            .not_found_service(ServeFile::new("crates/genius-server/static/index.html"));
        
        // Serve demo files
        let demo_dir = ServeDir::new("demo");
        
        let app = Router::new()
            // API routes
            .route("/api/v1/games", post(create_game_handler))
            .route("/api/v1/games/:id", get(get_game_handler))
            .route("/api/v1/games/:id/actions", post(submit_action_handler))
            .route("/api/v1/stats", get(get_stats_handler))
            // Static files
            .nest_service("/demo", demo_dir)
            .fallback_service(static_dir)
            .layer(CorsLayer::permissive())
            .with_state(Arc::new(self));
        
        let listener = tokio::net::TcpListener::bind(addr).await?;
        tracing::info!("🚀 Simple Game Server listening on http://{}", addr);
        tracing::info!("📊 Dashboard available at http://{}/", addr);
        tracing::info!("🎮 Demos available at http://{}/demo/", addr);
        
        axum::serve(listener, app).await?;
        Ok(())
    }
}

#[derive(Deserialize)]
struct CreateGameRequest {
    game_type: String,
    #[serde(default)]
    players: Vec<String>,
    #[serde(default)]
    config: Option<serde_json::Value>,
}

#[derive(Serialize)]
struct CreateGameResponse {
    game_id: Uuid,
    status: String,
}

async fn create_game_handler(
    State(server): State<Arc<SimpleGameServer>>,
    Json(req): Json<CreateGameRequest>,
) -> impl IntoResponse {
    let game_id = Uuid::new_v4();
    
    // Parse game type
    let game_type = match req.game_type.as_str() {
        "MiniGo" => GameType::MiniGo,
        "MiniHoldem" => GameType::MiniHoldem,
        "PrisonersDilemma" => GameType::PrisonersDilemma,
        "ConsciousnessPoker" => GameType::ConsciousnessPoker,
        _ => GameType::MinorityGame, // Default
    };
    
    // Create game config
    let config = GameConfig {
        game_type,
        rounds: 100,
        time_limit_ms: 5000,
        special_rules: HashMap::new(),
        initial_players: req.players.into_iter()
            .map(|name| genius_core::Player {
                id: genius_core::player::PlayerId::from_string(name.clone()),
                name,
                player_type: genius_core::player::PlayerType::Human,
                metadata: serde_json::Value::Null,
            })
            .collect(),
    };
    
    // Create game instance
    match create_game(game_type) {
        Ok(mut game) => {
            match game.initialize(config).await {
                Ok(state) => {
                    server.games.write().await.insert(game_id, state);
                    Json(CreateGameResponse {
                        game_id,
                        status: "created".to_string(),
                    })
                }
                Err(e) => {
                    tracing::error!("Failed to initialize game: {}", e);
                    Json(CreateGameResponse {
                        game_id,
                        status: format!("error: {}", e),
                    })
                }
            }
        }
        Err(e) => {
            tracing::error!("Failed to create game: {}", e);
            Json(CreateGameResponse {
                game_id,
                status: format!("error: {}", e),
            })
        }
    }
}

async fn get_game_handler(
    State(server): State<Arc<SimpleGameServer>>,
    Path(game_id): Path<Uuid>,
) -> impl IntoResponse {
    let games = server.games.read().await;
    match games.get(&game_id) {
        Some(state) => Json(serde_json::json!({
            "game_id": game_id,
            "state": state,
        })),
        None => Json(serde_json::json!({
            "error": "Game not found",
        })),
    }
}

#[derive(Deserialize)]
struct SubmitActionRequest {
    player_id: String,
    action: serde_json::Value,
}

async fn submit_action_handler(
    State(server): State<Arc<SimpleGameServer>>,
    Path(game_id): Path<Uuid>,
    Json(req): Json<SubmitActionRequest>,
) -> impl IntoResponse {
    Json(serde_json::json!({
        "status": "action_received",
        "game_id": game_id,
        "player_id": req.player_id,
    }))
}

async fn get_stats_handler() -> impl IntoResponse {
    Json(serde_json::json!({
        "server_status": "online",
        "active_games": 0,
        "total_players": 0,
        "games_played": 0,
        "version": "0.1.0",
    }))
}