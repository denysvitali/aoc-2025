# Day 5: Cafeteria

## Problem Summary

The cafeteria's new inventory management system uses ingredient ID ranges to track freshness. The database contains ranges of fresh ingredient IDs (inclusive) and a list of available ingredient IDs.

- **Part 1**: Count how many available ingredient IDs fall within any fresh range.
- **Part 2**: Count the total number of unique ingredient IDs considered fresh by all ranges.

## Solution Approach

### Part 1
Parse the ranges and ingredient IDs, then check each ingredient against all ranges.

```rust
fn is_fresh(id: i64, ranges: &[RangeInclusive<i64>]) -> bool {
    ranges.iter().any(|range| range.contains(&id))
}

ingredients.iter().filter(|&&id| is_fresh(id, &ranges)).count()
```

### Part 2
Merge overlapping and adjacent ranges, then sum the lengths of the merged ranges.

```rust
fn merge_ranges(ranges: &[RangeInclusive<i64>]) -> Vec<RangeInclusive<i64>> {
    // Sort by start value
    // Merge overlapping/adjacent ranges
}

merged.iter().map(|r| r.end() - r.start() + 1).sum()
```

The key insight is that ranges can overlap (e.g., `10-14` and `12-18` merge to `10-18`), so we need to merge them before counting unique IDs.

## Example

```
3-5
10-14
16-20
12-18
```

- Part 1: Available IDs `1, 5, 8, 11, 17, 32` -> 3 fresh (5, 11, 17)
- Part 2: Merged ranges `3-5` and `10-20` -> 14 unique fresh IDs

## Answers

- Part 1: **509**
- Part 2: **336790092076620**

---

*Solution by Claude (claude-opus-4-5-20251101)*
