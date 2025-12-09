# Day 9: Movie Theater

## Problem Summary

The movie theater has a tile floor with red tiles at specific coordinates. The Elves want to find the largest rectangle that uses red tiles as opposite corners.

- **Part 1**: Find the largest rectangle using any two red tiles as opposite corners.
- **Part 2**: Red tiles are connected by green tiles forming a loop. Find the largest rectangle using red corners where all tiles inside are red or green.

## Solution Approach

### Part 1
Simply iterate through all pairs of red tiles and compute the rectangle area for each pair. The area includes all tiles from corner to corner (inclusive).

```rust
// Area = (|x2 - x1| + 1) * (|y2 - y1| + 1)
// Return maximum over all pairs
```

### Part 2
The red tiles form vertices of a rectilinear polygon connected by green lines. The interior is also green. We need to find the largest rectangle that:
1. Has red tiles at opposite corners
2. Is entirely contained within the polygon (boundary + interior)

Algorithm:
1. Build line segments connecting consecutive red tiles
2. For each pair of red tiles, check if their rectangle fits inside the polygon:
   - All 4 corners must be inside or on the boundary (ray casting)
   - No polygon edge can cut through the rectangle's interior

```rust
// Ray casting: count vertical segment crossings to determine inside/outside
// Check no segment intersects rectangle interior
```

## Example

```
7,1
11,1
11,7
9,7
9,5
2,5
2,3
7,3
```

Visual representation with red tiles (#) and green tiles (X):
```
..............
.......#XXX#..
.......XXXXX..
..#XXXX#XXXX..
..XXXXXXXXXX..
..#XXXXXX#XX..
.........XXX..
.........#X#..
..............
```

- **Part 1**: Largest rectangle = 50 (between 2,5 and 11,1)
- **Part 2**: Largest valid rectangle = 24 (between 9,5 and 2,3)

## Key Insight

For Part 2, we don't need to materialize the entire grid (which would be ~100k × 100k). Instead, we use computational geometry:
- Point-in-polygon test via ray casting
- Rectangle containment = corners inside + no edge intersections

## Complexity

- Time: O(n² × m) where n = number of red tiles, m = number of segments
- Space: O(n) for storing tiles and segments

## Answers

- Part 1: **4744899849**
- Part 2: **1540192500**

---

*Solution by Claude (claude-opus-4-5-20251101)*
