# Day 7: Laboratories

## Problem Summary

You're stuck in a teleporter room with a broken quantum tachyon manifold. A tachyon beam enters from position `S` and travels downward. When it encounters a splitter (`^`), the beam stops and creates two new beams: one going left and one going right.

- **Part 1**: Count how many times the beam is split as it travels through the manifold. When multiple beams land on the same position, they merge into one.
- **Part 2**: Apply the many-worlds interpretation - each split creates two separate timelines. Count the total number of timelines after all particles complete their journeys.

## Solution Approach

### Part 1
Track beam positions using a `HashSet` (beams at the same position merge). Process each row, and when a beam hits a splitter, count it as a split and add left/right beams.

```rust
// For each row, check each beam position
// If beam hits '^', increment splits and spawn left/right beams
// Beams at same position automatically merge via HashSet
```

### Part 2
Track timeline counts using a `HashMap<position, count>`. When timelines at a position hit a splitter, each timeline splits into two (doubling the count for that path).

```rust
// HashMap<column, timeline_count>
// When a splitter is hit, each timeline becomes 2
// Sum all timeline counts at the end
```

## Example

```
.......S.......
.......^.......
......^.^......
.....^.^.^.....
....^.^...^....
...^.^...^.^...
..^...^.....^..
.^.^.^.^.^...^.
```

- **Part 1**: 21 splits (counting each unique beam hitting a splitter)
- **Part 2**: 40 timelines (each split doubles the timeline count for that path)

## Challenges and Confusing Parts

1. **Human error with input**: Initially got the wrong answer because the puzzle input was incomplete - it was missing rows at the end. After pulling the corrected input, the solution worked correctly.

2. **Beam merging vs timeline counting**: The key insight is that Part 1 uses `HashSet` (beams merge when they reach the same position), while Part 2 uses `HashMap` with counts (timelines are independent even if particles end up at the same position).

3. **Ragged lines**: The input has lines of different lengths. Beams that go beyond a row's length simply exit the manifold (they don't cause errors, just stop being tracked).

4. **Off-by-one considerations**: Processing starts from row 1 (after S), and we need to handle both left edge (col > 0 check) and right edge (beams can go beyond grid width and exit).

## Answers

- Part 1: **1535**
- Part 2: **4404709551015**

---

*Solution by Claude (claude-opus-4-5-20251101)*
