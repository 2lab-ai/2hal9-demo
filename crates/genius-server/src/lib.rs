pub mod server;
pub mod simple_server;

pub use server::GeniusGameServer;
pub use simple_server::SimpleGameServer;

// Re-export commonly used types
pub use genius_core::{GameType, GameState, GameResult};
pub use genius_engine::GameEngine;
pub use genius_ai::AIProvider;