# Day 8: Playground

## Problem Summary

You find yourself in a giant underground playground where Elves are setting up Christmas decorations. They've suspended junction boxes in 3D space and need to connect them with strings of lights. The goal is to connect junction boxes efficiently, starting with the closest pairs.

- **Part 1**: Connect the 1000 closest pairs of junction boxes. Multiply together the sizes of the three largest circuits.
- **Part 2**: Continue connecting until all junction boxes form a single circuit. Return the product of the X coordinates of the last two junction boxes connected.

## Solution Approach

This is essentially Kruskal's algorithm for building a minimum spanning tree, using Union-Find (Disjoint Set Union) data structure.

### Part 1
1. Parse 3D coordinates (X,Y,Z) for each junction box
2. Generate all pairs and compute squared Euclidean distances
3. Sort pairs by distance
4. Use Union-Find to connect the first 1000 pairs
5. Find the three largest circuit sizes and multiply them

```rust
// Union-Find with path compression and union by rank
// Track circuit sizes during union operations
// Sort circuits descending and multiply top 3
```

### Part 2
1. Same setup as Part 1
2. Keep connecting pairs until only one circuit remains
3. The last connection that reduces circuit count to 1 is our answer
4. Return product of X coordinates of that pair

```rust
// Track number of circuits (starts at n)
// Each successful union decreases circuit count by 1
// When count reaches 1, return points[i].x * points[j].x
```

## Example

```
162,817,812
57,618,57
906,360,560
...
```

- **Part 1** (10 connections): Circuits of size 5, 4, 2, ... → 5 × 4 × 2 = 40
- **Part 2**: Last connection at (216,146,977) and (117,168,530) → 216 × 117 = 25272

## Key Data Structures

### Union-Find
- `parent[]`: Points to parent node (self if root)
- `rank[]`: Tree depth for union by rank optimization
- `size[]`: Circuit size for each root
- Path compression in `find()` for O(α(n)) amortized operations

## Complexity

- Time: O(n² log n) for generating and sorting all pairs
- Space: O(n²) for storing all pairs

With 1000 junction boxes, there are ~500,000 pairs to consider.

## Answers

- Part 1: **32103**
- Part 2: **8133642976**

---

*Solution by Claude (claude-opus-4-5-20251101)*
