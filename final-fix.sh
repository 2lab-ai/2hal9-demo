#!/bin/bash

# Final comprehensive fix for all imports and types
find /Users/icedac/2lab.ai/2hal9-demo/crates/genius-games/src -name "*.rs" -type f | while read file; do
    # Skip lib.rs and mod.rs files
    if [[ "$file" == *"lib.rs" ]] || [[ "$file" == *"mod.rs" ]]; then
        continue
    fi
    
    # Fix Action type in function parameters
    sed -i '' 's/HashMap<String, Action>/HashMap<String, PlayerAction>/g' "$file"
    
    # Fix Outcome type 
    sed -i '' 's/Outcome {/RoundOutcome {/g' "$file"
    sed -i '' 's/: Outcome/: RoundOutcome/g' "$file"
    
    # Fix standalone Action references that weren't caught
    sed -i '' 's/&Action)/&PlayerAction)/g' "$file"
    sed -i '' 's/(Action)/(PlayerAction)/g' "$file"
    sed -i '' 's/ Action,/ PlayerAction,/g' "$file"
    sed -i '' 's/<Action,/<PlayerAction,/g' "$file"
    
    # Fix anyhow::Result to genius_core::Result
    sed -i '' 's/anyhow::Result<GameState>/Result<GameState>/g' "$file"
    sed -i '' 's/anyhow::Result<RoundResult>/Result<RoundResult>/g' "$file"
    
    # Add GameEvent import if missing
    if ! grep -q "GameEvent" "$file" && grep -q "RoundResult" "$file"; then
        sed -i '' 's/use genius_core::{Game, GameConfig, GameState, GameType, PlayerAction, RoundResult, RoundOutcome, GameResult, GameAnalytics,/use genius_core::{Game, GameConfig, GameState, GameType, PlayerAction, RoundResult, RoundOutcome, GameResult, GameAnalytics, GameEvent,/g' "$file"
    fi
    
    echo "Applied final fixes to: $file"
done

echo "Final fixes complete!"