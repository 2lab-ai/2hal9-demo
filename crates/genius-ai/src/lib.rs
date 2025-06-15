//! AI provider abstractions and implementations
//! 
//! This crate provides interfaces for various AI backends and collective intelligence.

pub mod provider;
pub mod collective;
pub mod sota;

// Re-export provider implementations
pub mod providers {
    pub mod ollama;
    pub mod bedrock;
    pub mod mock;
    
    pub use ollama::OllamaProvider;
    pub use bedrock::BedrockProvider;
    pub use mock::MockProvider;
}

pub use provider::{AIProvider, AIDecision};
pub use collective::{CollectiveIntelligence, CollectiveStrategy};
pub use sota::{SOTAManager, ReasoningChain};