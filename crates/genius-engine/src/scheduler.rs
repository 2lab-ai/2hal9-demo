//! Turn scheduling and timeout management

use std::time::Duration;
use tokio::time::{timeout, Instant};
use genius_core::{Result, GameError};

/// Manages turn timing and scheduling
pub struct TurnScheduler {
    turn_timeout: Duration,
    grace_period: Duration,
}

impl TurnScheduler {
    pub fn new(turn_timeout: Duration) -> Self {
        Self {
            turn_timeout,
            grace_period: Duration::from_secs(5),
        }
    }
    
    /// Execute a turn with timeout
    pub async fn execute_with_timeout<F, T>(&self, player_id: String, f: F) -> Result<T>
    where
        F: std::future::Future<Output = T>,
    {
        match timeout(self.turn_timeout, f).await {
            Ok(result) => Ok(result),
            Err(_) => Err(GameError::TurnTimeout { player_id }),
        }
    }
    
    /// Check if a deadline has passed
    pub fn is_expired(&self, deadline: Instant) -> bool {
        Instant::now() > deadline
    }
}

impl Default for TurnScheduler {
    fn default() -> Self {
        Self::new(Duration::from_secs(30))
    }
}