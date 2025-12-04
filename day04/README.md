# Day 4: Printing Department

## Problem Summary

The printing department has paper rolls (`@`) arranged on a grid. Forklifts can only access a roll if it has fewer than 4 adjacent rolls in the 8 surrounding positions (including diagonals).

- **Part 1**: Count how many rolls are initially accessible by forklifts.
- **Part 2**: Repeatedly remove all accessible rolls until no more can be removed. Count the total rolls removed.

## Solution Approach

### Part 1
For each roll, count adjacent rolls in all 8 directions. If count < 4, it's accessible.

```rust
for dr in -1..=1 {
    for dc in -1..=1 {
        if dr == 0 && dc == 0 { continue; }
        // Check if neighbor is '@' and count
    }
}
if adjacent < 4 { count += 1; }
```

### Part 2
Simulate the removal process iteratively:

1. Find all currently accessible rolls (< 4 adjacent)
2. Remove them all simultaneously
3. Repeat until no rolls are accessible
4. Sum total removed across all iterations

```rust
loop {
    let to_remove: Vec<_> = /* find all accessible rolls */;
    if to_remove.is_empty() { break; }
    for (r, c) in to_remove {
        grid[r][c] = '.';
    }
    total += to_remove.len();
}
```

The key insight is that removing rolls in each round can expose new rolls that become accessible in the next round, creating a cascading effect.

## Example

Initial grid with 13 accessible rolls (marked `x`):
```
..xx.xx@x.
x@@.@.@.@@
@@@@@.x.@@
@.@@@@..@.
x@.@@@@.@x
.@@@@@@@.@
.@.@.@.@@@
x.@@@.@@@@
.@@@@@@@@.
x.x.@@@.x.
```

After 9 removal rounds, 43 total rolls are removed.

## Answers

- Part 1: **1409**
- Part 2: **8366**

---

*Solution by Claude (claude-opus-4-5-20251101)*
