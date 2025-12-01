# Day 1: Secret Entrance

## Problem Summary

A safe dial (0-99) starts at position 50. Given a sequence of rotations (L for left/decreasing, R for right/increasing), determine the password.

- **Part 1**: Count how many times the dial points at 0 *after* a rotation completes.
- **Part 2**: Count how many times the dial points at 0 *at any point* during or after rotations (every "click" through 0 counts).

## Solution Approach

### Part 1
Straightforward simulation: apply each rotation, use modulo arithmetic to wrap around (0-99), and count when the final position is 0.

### Part 2
This required tracking the **raw cumulative position** (without modulo) to correctly count zero crossings. The key insight is that every time we cross a multiple of 100 in our raw position, we've passed through 0 on the dial.

Formula: Count multiples of 100 in the range `[low, high]` of each movement, excluding the starting position if it's already on a multiple of 100 (since we don't count "starting at 0" as a crossing).

## Where I Got Stuck

Part 2 initially gave the wrong answer (6595 instead of 6599). The bug was:

1. **Applying modulo after each step**: I was resetting position to `position.rem_euclid(100)` after each rotation, losing track of how many "laps" we'd made around the dial.

2. **Wrong interval counting**: My initial formula `(high-1)/100 - (low-1)/100` counted the wrong range - it was computing `(low, high]` instead of `[low, high]`.

3. **Not excluding start position**: When starting at 0, moving away shouldn't count as a zero crossing, but landing on 0 should.

The fix was to:
- Keep the raw cumulative position across iterations
- Use `high/100 - (low-1)/100` to count multiples in `[low, high]`
- Subtract 1 if the starting position is on a multiple of 100

## Answers

- Part 1: **1105**
- Part 2: **6599**

---

*Solution by Claude (claude-opus-4-5-20251101)*
