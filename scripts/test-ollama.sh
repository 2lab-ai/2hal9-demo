#!/bin/bash

# Test Ollama integration with the Genius Game Platform

echo "🦙 Testing Ollama Integration..."

# Check if Ollama is installed
if ! command -v ollama &> /dev/null; then
    echo "❌ Ollama is not installed!"
    echo ""
    echo "📥 To install Ollama:"
    echo "   macOS: brew install ollama"
    echo "   Linux: curl -fsSL https://ollama.ai/install.sh | sh"
    echo ""
    exit 1
fi

# Check if Ollama is running
if ! curl -s http://localhost:11434/api/version > /dev/null 2>&1; then
    echo "⚠️  Ollama is not running. Starting Ollama..."
    ollama serve &
    OLLAMA_PID=$!
    sleep 3
fi

echo "✅ Ollama is running"

# List available models
echo ""
echo "📋 Available models:"
ollama list

# Check if we have any models
if [ $(ollama list | wc -l) -eq 1 ]; then
    echo ""
    echo "⚠️  No models found. Pulling llama2..."
    ollama pull llama2
fi

# Test API endpoint
echo ""
echo "🧪 Testing Ollama API..."
curl -X POST http://localhost:11434/api/generate \
  -H "Content-Type: application/json" \
  -d '{
    "model": "llama2",
    "prompt": "You are playing a game of Prisoners Dilemma. Your opponent cooperated last round. What do you do? Answer with just one word: cooperate or defect",
    "stream": false,
    "options": {
      "temperature": 0.7,
      "max_tokens": 10
    }
  }' | jq -r '.response'

echo ""
echo "✅ Ollama API test complete"

# Create test Rust file
echo ""
echo "📝 Creating Ollama test program..."

cat > test_ollama_integration.rs << 'EOF'
use genius_ai::providers::ollama::OllamaProvider;
use genius_ai::AIProvider;
use genius_core::{GameContext, PlayerAction};
use serde_json::json;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    println!("🦙 Testing Ollama Provider...");
    
    // Create Ollama provider
    let provider = OllamaProvider::new("http://localhost:11434", "llama2");
    
    // Create game context
    let context = GameContext {
        game_type: "PrisonersDilemma".to_string(),
        round: 5,
        history: vec![
            json!({"action": "cooperate", "opponent": "defect"}),
            json!({"action": "defect", "opponent": "cooperate"}),
        ],
        game_state: json!({
            "my_score": 10,
            "opponent_score": 12
        }),
        valid_actions: vec!["cooperate".to_string(), "defect".to_string()],
        time_remaining_ms: 5000,
    };
    
    // Get AI decision
    println!("🤔 Asking AI for decision...");
    match provider.get_action(&context).await {
        Ok(action) => {
            println!("✅ AI Decision: {}", action.action_type);
            if let Some(reasoning) = action.reasoning {
                println!("💭 Reasoning: {}", reasoning);
            }
        }
        Err(e) => {
            println!("❌ Error: {}", e);
        }
    }
    
    Ok(())
}
EOF

echo "✅ Test program created"
echo ""
echo "📦 To run the test:"
echo "   1. cd to 2hal9-demo directory"
echo "   2. cargo run --bin test_ollama"
echo ""
echo "🎮 To use Ollama in games:"
echo '   Create AI player with provider: "ollama"'
echo '   Model options: llama2, mistral, codellama, etc.'

# Cleanup
if [ ! -z "$OLLAMA_PID" ]; then
    echo ""
    echo "🛑 Stopping Ollama server..."
    kill $OLLAMA_PID
fi