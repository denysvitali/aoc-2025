#!/bin/bash

# Script to create a new day's solution from template

set -e

if [ -z "$1" ]; then
    echo "Usage: ./new_day.sh <day_number>"
    echo "Example: ./new_day.sh 2"
    exit 1
fi

DAY_NUM=$1
DAY_PADDED=$(printf "%02d" $DAY_NUM)
DAY_DIR="day${DAY_PADDED}"

if [ -d "$DAY_DIR" ]; then
    echo "Error: $DAY_DIR already exists!"
    exit 1
fi

echo "Creating $DAY_DIR..."

# Create directory structure
mkdir -p "$DAY_DIR/src"
mkdir -p "$DAY_DIR/input"

# Create Cargo.toml from template
sed "s/{{DAY}}/${DAY_PADDED}/g" template/Cargo.toml.template > "$DAY_DIR/Cargo.toml"

# Create main.rs from template
sed "s/{{DAY_NUM}}/${DAY_NUM}/g" template/main.rs.template > "$DAY_DIR/src/main.rs"

# Create empty input files
touch "$DAY_DIR/input/example.txt"
touch "$DAY_DIR/input/input.txt"

# Add to workspace Cargo.toml
if ! grep -q "\"$DAY_DIR\"" Cargo.toml; then
    # Add the new day to the members list
    sed -i "s/\]$/    \"$DAY_DIR\",\n]/" Cargo.toml
fi

echo "Created $DAY_DIR successfully!"
echo ""
echo "Next steps:"
echo "  1. Add the example input to $DAY_DIR/input/example.txt"
echo "  2. Add your puzzle input to $DAY_DIR/input/input.txt"
echo "  3. Implement the solution in $DAY_DIR/src/main.rs"
echo "  4. Run with: cargo run -p $DAY_DIR"
echo "  5. Test with: cargo test -p $DAY_DIR"
