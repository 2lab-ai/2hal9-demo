#!/bin/bash

# Fix any remaining Action -> PlayerAction references
find "$(dirname "$0")/../crates/genius-games/src" -name "*.rs" -type f | while read file; do
    # Replace standalone Action references
    sed -i '' 's/\bAction\b/PlayerAction/g' "$file"
    sed -i '' 's/\bOutcome\b/RoundOutcome/g' "$file"
done

echo "Fixed remaining imports!"