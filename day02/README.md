# Day 2: Gift Shop

## Problem Summary

A young Elf added invalid product IDs to the gift shop database. Invalid IDs are numbers made of a repeating digit sequence.

- **Part 1**: Find IDs that are a sequence repeated exactly twice (e.g., `11`, `6464`, `123123`).
- **Part 2**: Find IDs that are a sequence repeated at least twice (e.g., `111`, `1212`, `123123123`).

Given ranges of IDs, sum all the invalid ones.

## Solution Approach

### Part 1
For a number to be a repeated-twice pattern, it must have even length and split into two identical halves:
```rust
let (first_half, second_half) = s.split_at(len / 2);
first_half == second_half
```

### Part 2
Try all possible pattern lengths from 1 to `len/2`. For each length that evenly divides the total length, check if the entire string is that pattern repeated:
```rust
for pattern_len in 1..=len / 2 {
    if !len.is_multiple_of(pattern_len) { continue; }
    let pattern = &s[..pattern_len];
    // Check if all subsequent chunks match the pattern
}
```

## Performance Notes

The solution iterates through every number in every range, which works but takes ~400-500ms per part. Potential optimizations:
- Generate repeated patterns directly instead of checking every number
- For part 1, only check even-length numbers
- Skip ranges that can't contain any repeated patterns

However, for AoC purposes, sub-second performance is acceptable.

## Answers

- Part 1: **29818212493**
- Part 2: **37432260594**

---

*Solution by Claude (claude-opus-4-5-20251101)*
