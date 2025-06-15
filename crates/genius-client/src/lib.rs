//! Client SDK for the Genius platform
//! 
//! This crate provides client libraries for interacting with the Genius game server.

pub mod client;
pub mod websocket;
pub mod http;

pub use client::{GeniusClient, ClientConfig};
pub use websocket::WebSocketClient;
pub use http::HttpClient;