# 2HAL9 Demo - Genius Game Platform

A modular, extensible platform for AI game competitions and collective intelligence experiments, extracted and refactored from the HAL9 consciousness project.

## 🎮 Overview

The Genius Game Platform provides a framework for running various game-theoretic experiments with AI agents. It supports:

- **17 Different Games**: From strategic games like Go and Poker to trust-based games like Prisoner's Dilemma
- **Multiple AI Providers**: Ollama (local), AWS Bedrock, OpenAI, and more
- **Collective Intelligence**: Experiments with swarm behavior and emergent strategies
- **Real-time Analytics**: Emergence detection and performance metrics
- **WebSocket Support**: Live game streaming and updates

## 🏗️ Architecture

This project follows a clean, modular architecture:

```
genius-core/      # Core traits and types
genius-engine/    # Game execution engine
genius-ai/        # AI provider abstractions
genius-games/     # Game implementations
genius-server/    # HTTP/WebSocket server
genius-client/    # Client SDK
```

## 🚀 Quick Start

### Prerequisites

- Rust 1.70+
- Docker (optional, for containerized deployment)
- Ollama (optional, for local AI models)

### Installation

```bash
# Clone the repository
git clone https://github.com/2lab-ai/2hal9-demo.git
cd 2hal9-demo

# Build the project
cargo build --release

# Run the server
cargo run --bin genius-server

# Or use Docker
docker-compose -f docker/docker-compose.yml up
```

### Running Your First Game

```bash
# Start a Prisoner's Dilemma game with AI agents
curl -X POST http://localhost:8080/api/v1/games \
  -H "Content-Type: application/json" \
  -d '{
    "game_type": "PrisonersDilemma",
    "players": ["ai_1", "ai_2"],
    "rounds": 100
  }'
```

## 🎯 Game Categories

### Strategic Games
- **Mini Go**: Simplified version of Go on a 9x9 board
- **Mini Hold'em**: Texas Hold'em poker variant
- **Byzantine Generals**: Consensus under unreliable communication
- **Minority Game**: Players win by being in the minority

### Collective Intelligence
- **Swarm Optimization**: Collective pathfinding
- **Collective Maze**: Cooperative maze solving
- **Quantum Consensus**: Superposition-based decision making
- **Recursive Reasoning**: Meta-cognitive challenges

### Survival Games
- **Battle Royale**: Last player standing wins
- **Hunger Games**: Resource management and alliances
- **Squid Game**: High-stakes elimination challenges
- **King of the Hill**: Territory control

### Trust-Based Games
- **Prisoner's Dilemma**: Classic cooperation vs defection
- **Trust Fall**: Risk and reward dynamics
- **Liar's Dice**: Bluffing and probability

## 🤖 AI Integration

### Supported Providers

```rust
// Local models with Ollama
let provider = OllamaProvider::new("llama2");

// Cloud models with AWS Bedrock
let provider = BedrockProvider::new("claude-3");

// Mock provider for testing
let provider = MockProvider::new();
```

### Collective Intelligence

Run experiments with multiple AI agents:

```rust
let collective = CollectiveIntelligence::new()
    .with_agents(16)
    .with_strategy(Strategy::EmergentConsensus);
```

## 📊 Analytics & Emergence Detection

The platform automatically tracks:
- Collective coordination scores
- Decision diversity indices
- Strategic depth metrics
- Emergence event detection
- Performance differentials

## 🔧 Development

### Adding a New Game

1. Implement the `Game` trait in `genius-games/src/your_game.rs`
2. Register it in the game factory
3. Add tests and documentation

```rust
#[async_trait]
impl Game for YourGame {
    async fn initialize(&mut self, config: GameConfig) -> Result<GameState> {
        // Your implementation
    }
    
    // ... other trait methods
}
```

### Adding a New AI Provider

1. Implement the `AIProvider` trait
2. Add to the provider factory
3. Configure in `genius.toml`

## 📚 Documentation

- [Architecture Guide](docs/architecture.md)
- [Game Development](docs/game_development.md)
- [AI Integration](docs/ai_integration.md)
- [API Reference](docs/api_reference.md)

## 🧪 Testing

```bash
# Run all tests
cargo test

# Run specific game tests
cargo test --package genius-games

# Run integration tests
cargo test --test integration_tests
```

## 🚢 Deployment

### Docker

```bash
docker build -t genius-game-server -f docker/Dockerfile .
docker run -p 8080:8080 genius-game-server
```

### Kubernetes

```bash
kubectl apply -f k8s/
```

## 📈 Performance

- **Throughput**: 10,000+ games/second
- **Latency**: <10ms per game decision
- **Scalability**: Horizontal scaling with state synchronization
- **Memory**: <100MB per 1000 active games

## 🤝 Contributing

Contributions are welcome! Please read our [Contributing Guide](CONTRIBUTING.md) for details.

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## 🔗 Related Projects

- [HAL9](https://github.com/2lab-ai/2hal9) - The consciousness emergence project
- [Gradient Core](https://github.com/2lab-ai/gradient-core) - Mathematical foundations

## 🙏 Acknowledgments

This project emerged from the HAL9 consciousness experiments, where game theory became a lens for understanding collective intelligence and emergence.