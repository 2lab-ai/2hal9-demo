#!/bin/bash

# Fix imports in all game files
find /Users/icedac/2lab.ai/2hal9-demo/crates/genius-games/src -name "*.rs" -type f | while read file; do
    # Skip lib.rs and mod.rs files
    if [[ "$file" == *"lib.rs" ]] || [[ "$file" == *"mod.rs" ]]; then
        continue
    fi
    
    # Replace old imports with correct ones
    sed -i '' 's/use super::{Game, GameConfig, GameState, GameType, Action, RoundResult, Outcome, GameResult, GameAnalytics};/use genius_core::{Game, GameConfig, GameState, GameType, PlayerAction, RoundResult, RoundOutcome, GameResult, GameAnalytics, Result, GameError};/g' "$file"
    
    # Replace Action with PlayerAction throughout the file
    sed -i '' 's/: Action/: PlayerAction/g' "$file"
    sed -i '' 's/<Action>/<PlayerAction>/g' "$file"
    sed -i '' 's/&Action/&PlayerAction/g' "$file"
    sed -i '' 's/ Action / PlayerAction /g' "$file"
    
    # Replace Outcome with RoundOutcome
    sed -i '' 's/: Outcome/: RoundOutcome/g' "$file"
    sed -i '' 's/<Outcome>/<RoundOutcome>/g' "$file"
    sed -i '' 's/&Outcome/&RoundOutcome/g' "$file"
    sed -i '' 's/ Outcome / RoundOutcome /g' "$file"
    
    echo "Fixed imports in: $file"
done

echo "Import fixes complete!"