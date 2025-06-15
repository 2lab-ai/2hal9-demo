#!/bin/bash

# Fix any remaining Action -> PlayerAction references
find /Users/icedac/2lab.ai/2hal9-demo/crates/genius-games/src -name "*.rs" -type f | while read file; do
    # Replace standalone Action references
    sed -i '' 's/\bAction\b/PlayerAction/g' "$file"
    sed -i '' 's/\bOutcome\b/RoundOutcome/g' "$file"
done

echo "Fixed remaining imports!"