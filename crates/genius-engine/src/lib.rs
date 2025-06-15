//! Game execution engine for the Genius platform
//! 
//! This crate provides the core game engine, analytics, and streaming capabilities.

pub mod engine;
pub mod analytics;
pub mod streaming;
pub mod scheduler;
pub mod emergence;

pub use engine::GameEngine;
pub use analytics::AnalyticsEngine;
pub use streaming::GameEventStreamer;
pub use scheduler::TurnScheduler;
pub use emergence::EmergenceDetector;