#!/bin/bash

# Fix all imports comprehensively
find "$(dirname "$0")/../crates/genius-games/src" -name "*.rs" -type f | while read file; do
    # Skip lib.rs and mod.rs files
    if [[ "$file" == *"lib.rs" ]] || [[ "$file" == *"mod.rs" ]]; then
        continue
    fi
    
    # Check if file uses "use super::*;"
    if grep -q "use super::\*;" "$file"; then
        # Replace with proper imports
        sed -i '' 's/use super::\*;/use genius_core::{Game, GameConfig, GameState, GameType, PlayerAction, RoundResult, RoundOutcome, GameResult, GameAnalytics, GameEvent, EmergenceEvent, EmergenceType, Result, GameError};\nuse serde::{Serialize, Deserialize};/g' "$file"
        echo "Fixed super::* imports in: $file"
    fi
    
    # Add serde imports if missing
    if ! grep -q "use serde::" "$file"; then
        # Add after the first use statement
        sed -i '' '0,/^use /{s/^use /use serde::{Serialize, Deserialize};\nuse /}' "$file"
        echo "Added serde imports to: $file"
    fi
done

echo "Import fixes complete!"