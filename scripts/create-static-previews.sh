#!/bin/bash

# Create static preview images for games until we can generate proper GIFs

SCRIPT_DIR="$(dirname "$0")"
PROJECT_ROOT="$SCRIPT_DIR/.."
DEMO_DIR="$PROJECT_ROOT/demo"

echo "📸 Creating static preview images for games..."

# Create a simple preview HTML that takes screenshots
cat > "$DEMO_DIR/capture_preview.html" << 'EOF'
<!DOCTYPE html>
<html>
<head>
    <title>Game Preview Capture</title>
    <style>
        body { margin: 0; background: #000; }
        .preview-container {
            width: 1200px;
            margin: 0 auto;
            padding: 40px;
            background: #111;
        }
        .game-frame {
            border-radius: 10px;
            overflow: hidden;
            box-shadow: 0 20px 40px rgba(0, 0, 0, 0.5);
            margin: 20px 0;
        }
        iframe {
            width: 100%;
            height: 700px;
            border: none;
            display: block;
        }
        .title {
            color: #fff;
            font-family: -apple-system, sans-serif;
            font-size: 24px;
            margin: 20px 0;
            text-align: center;
        }
    </style>
</head>
<body>
    <div class="preview-container">
        <h2 class="title">Mini Go - Premium Edition</h2>
        <div class="game-frame">
            <iframe src="mini_go_premium.html"></iframe>
        </div>
        
        <h2 class="title">Mini Hold'em - Premium Edition</h2>
        <div class="game-frame">
            <iframe src="mini_holdem_premium.html"></iframe>
        </div>
        
        <h2 class="title">Consciousness Poker</h2>
        <div class="game-frame">
            <iframe src="consciousness_poker.html"></iframe>
        </div>
    </div>
</body>
</html>
EOF

echo "✅ Preview capture page created"
echo ""
echo "📝 Instructions to create screenshots:"
echo "1. Open $DEMO_DIR/capture_preview.html in your browser"
echo "2. Take screenshots of each game"
echo "3. Save as:"
echo "   - mini_go_preview.png"
echo "   - mini_holdem_preview.png"
echo "   - consciousness_poker_preview.png"
echo ""
echo "🎮 Or play the games directly:"
echo "   - $DEMO_DIR/mini_go_premium.html"
echo "   - $DEMO_DIR/mini_holdem_premium.html"
echo "   - $DEMO_DIR/consciousness_poker.html"