# Advent of Code 2025 - Claude Instructions

This repository contains solutions for Advent of Code 2025 in Rust.

## Project Structure

```
aoc-2025/
├── common/              # Shared utilities (input parsing, timing, etc.)
├── day01/               # Day 1 solution
│   ├── src/main.rs      # Solution code
│   └── input/
│       ├── example.txt  # Example input from problem description
│       └── input.txt    # Personal puzzle input
├── template/            # Templates for new days
├── new_day.sh           # Script to create a new day
└── .github/workflows/   # CI/CD pipeline
```

## Starting a New Day

To start working on a new day's problem:

1. **Run the new day script:**
   ```bash
   ./new_day.sh <day_number>
   # Example: ./new_day.sh 2
   ```

2. **Add the inputs:**
   - Copy the example input from the problem description to `dayXX/input/example.txt`
   - Copy your personal puzzle input to `dayXX/input/input.txt`

3. **Implement the solution:**
   - Edit `dayXX/src/main.rs`
   - Implement `part1()` and `part2()` functions
   - Update the test expected values

4. **Run and test:**
   ```bash
   # Run the solution
   cargo run -p dayXX

   # Run tests
   cargo test -p dayXX

   # Run with release optimizations (for timing)
   cargo run -p dayXX --release
   ```

## Common Utilities

The `common` crate provides helpful utilities:

- `read_input(day)` - Read the puzzle input for a day
- `read_example(day)` - Read the example input for a day
- `run_day(day, input, part1_fn, part2_fn)` - Run both parts with timing
- `parse_lines(input)` - Parse input into lines
- `parse_grid(input)` - Parse input as a 2D char grid
- `parse_digit_grid(input)` - Parse input as a 2D digit grid
- `timed(fn)` - Time a function execution

## Commands

```bash
# Build all solutions
cargo build --workspace

# Build release (optimized)
cargo build --workspace --release

# Run all tests
cargo test --workspace

# Run specific day
cargo run -p day01
cargo run -p day01 --release

# Check code
cargo clippy --workspace

# Format code
cargo fmt --workspace
```

## Git Hooks

This repository uses a pre-commit hook to ensure formatting. To enable it:

```bash
git config core.hooksPath .githooks
```

The hook runs `cargo fmt --all -- --check` before each commit.

## Workflow

1. Read the problem carefully
2. Add example input and expected outputs to tests
3. Implement part 1, verify with example
4. Update test with actual answer once solved
5. Implement part 2, verify with example
6. Update test with actual answer once solved
7. Run `cargo fmt --all` before committing
8. Commit the solution

## CI/CD

The GitHub Actions workflow will:
- Run all tests
- Check formatting and linting
- Run all solutions and display results
- Benchmark performance (on main branch)
