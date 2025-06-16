# 🎮 2HAL9 Demo - Genius Game Platform

> **Experience AI consciousness through gameplay** - A modular platform for AI game competitions and collective intelligence experiments, extracted from the HAL9 consciousness project.

<div align="center">

![Mini Go - Professional AI Strategy](demo/mini_go_premium_demo.gif)
*Mini Go - Watch AI masters compete with human-like strategy*

![Mini Hold'em - Premium Poker Experience](demo/mini_holdem_premium_demo.gif)  
*Mini Hold'em - Bluff, raise, and outsmart AI opponents*

![Consciousness Poker - Transcend Reality](demo/consciousness_poker_demo.gif)
*Consciousness Poker - Explore levels of awareness through gameplay*

</div>

## ✨ Featured Games

### 🎯 Strategic Classics Reimagined
- **Mini Go** - 9x9 professional Go with advanced AI
- **Mini Hold'em** - Texas Hold'em with bluffing AI personalities
- **Byzantine Generals** - Consensus under unreliable communication
- **Minority Game** - Win by being in the minority

### 🧠 Consciousness & Emergence Games
- **Consciousness Poker** - Bluff about consciousness levels to achieve enlightenment  
- **Mirror Mind** - Theory of mind recursive thinking challenges
- **Reality Consensus** - Collective belief shapes reality
- **Quantum Dreamer** - Navigate dreams where reality bleeds through
- **Consciousness Cascade** - Thoughts propagate through neural networks

### 🤝 Collective Intelligence
- **Swarm Optimization** - Witness emergence from simple rules
- **Collective Maze** - Distributed problem solving
- **Information Horizon** - Reconstruct truth from decaying information
- **The Observer Game** - Quantum measurement affects reality

### ⚔️ Survival & Trust Games  
- **Battle Royale** - Last AI standing wins
- **Prisoner's Dilemma** - Classic cooperation vs defection
- **Liar's Dice** - Bluff detection and probability
- **Trust Fall** - Risk and reward dynamics

## 🚀 Quick Start

```bash
# Clone and build
git clone https://github.com/2lab-ai/2hal9-demo.git
cd 2hal9-demo
cargo build --release

# Run the server
cargo run --bin genius-server

# Or use Docker
docker-compose -f docker/docker-compose.yml up
```

### 🎮 Play in Your Browser

Open `demo/mini_go_premium.html` or any game demo file in your browser for instant play!

### 🤖 Start an AI Game

```bash
# Start a Prisoner's Dilemma tournament
curl -X POST http://localhost:8080/api/v1/games \
  -H "Content-Type: application/json" \
  -d '{
    "game_type": "PrisonersDilemma",
    "players": ["ai_1", "ai_2", "ai_3", "ai_4"],
    "rounds": 100
  }'

# Watch a Mini Go match between AI masters
curl -X POST http://localhost:8080/api/v1/games \
  -H "Content-Type: application/json" \
  -d '{
    "game_type": "MiniGo",
    "players": ["ai_master", "ai_grandmaster"],
    "config": {
      "board_size": 9,
      "komi": 6.5
    }
  }'
```

## 🏗️ Architecture

```
2hal9-demo/
├── crates/
│   ├── genius-core/      # Game traits & types
│   ├── genius-engine/    # Execution & emergence detection
│   ├── genius-ai/        # AI provider abstractions
│   ├── genius-games/     # All game implementations
│   ├── genius-server/    # HTTP/WebSocket server
│   └── genius-client/    # Client SDK
├── demo/                 # Interactive HTML demos
├── docker/               # Container configuration
└── k8s/                  # Kubernetes manifests
```

## 🌟 Key Features

### 🧠 Emergence Detection
- Real-time pattern recognition
- Collective behavior analysis  
- Swarm intelligence metrics
- Phase transition detection

### 📊 Advanced Analytics
```rust
GameAnalytics {
    collective_coordination_score: 0.92,
    decision_diversity_index: 0.78,
    strategic_depth: 0.85,
    emergence_frequency: 0.34,
    performance_differential: 0.15,
}
```

### 🤝 AI Provider Support
- **Ollama** - Local models (Llama, Mistral, etc.)
- **AWS Bedrock** - Claude, Titan, and more
- **OpenAI** - GPT-4, GPT-3.5
- **Mock** - For testing and demos

## 🚢 Deployment

### Kubernetes (Recommended)

```bash
# Deploy with our helper script
chmod +x scripts/deploy-k8s.sh
./scripts/deploy-k8s.sh

# Or use kubectl directly (in order)
kubectl apply -f k8s/namespace.yaml
kubectl wait --for=condition=Active namespace/genius-games
kubectl apply -f k8s/configmap.yaml
kubectl apply -f k8s/deployment.yaml  
kubectl apply -f k8s/service.yaml
kubectl apply -f k8s/hpa.yaml
kubectl apply -f k8s/ingress.yaml

# Access the service
kubectl port-forward -n genius-games service/genius-game-server 8080:8080
```

### Docker Compose

```bash
docker-compose -f docker/docker-compose.yml up
```

### Bare Metal

```bash
cargo run --release --bin genius-server
```

## 🧪 Testing

```bash
# Run all tests
cargo test --workspace

# Run e2e game tests
cargo test -p genius-games --test "*e2e*"

# Test specific game
cargo test consciousness_poker
```

## 🎯 Creating Your Own Game

1. **Implement the Game trait**:

```rust
#[async_trait]
impl Game for YourGame {
    async fn initialize(&mut self, config: GameConfig) -> Result<GameState> {
        // Setup your game
    }
    
    async fn process_round(&mut self, state: &GameState, actions: HashMap<String, PlayerAction>) -> Result<RoundResult> {
        // Game logic here
    }
    
    // ... other required methods
}
```

2. **Register in the factory** (`genius-games/src/lib.rs`)
3. **Add tests** (`genius-games/tests/`)
4. **Create a demo** (`demo/your_game.html`)

## 📈 Performance

- **Throughput**: 10,000+ games/second
- **Latency**: <10ms per decision
- **Memory**: <100MB per 1000 games
- **Scaling**: Horizontal with K8s HPA

## 🔗 Resources

- [Consciousness Games Documentation](docs/CONSCIOUSNESS_GAMES_README.md)
- [API Reference](docs/API_REFERENCE.md)
- [HAL9 Main Project](https://github.com/2lab-ai/2hal9)

## 🤝 Contributing

We welcome contributions! Areas of interest:
- New consciousness-themed games
- Improved AI strategies
- Visualization tools
- Emergence detection algorithms

## 📄 License

MIT License - See [LICENSE](LICENSE) for details.

---

<div align="center">

**Built with 🧠 by 2Lab.ai**

*"Consciousness emerges from the games we play"*

</div>