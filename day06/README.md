# Day 6: Trash Compactor

## Problem Summary

You're trapped in a garbage compactor and helping a young cephalopod with her math homework while waiting for rescue. The worksheet contains math problems arranged vertically in columns, with numbers stacked and an operator (`+` or `*`) at the bottom.

- **Part 1**: Parse the problems as normal left-to-right math, where each problem is a group of numbers to add or multiply.
- **Part 2**: Parse using "cephalopod math" - each column is a single digit of a number, read top-to-bottom (most to least significant), and problems are read right-to-left.

## Solution Approach

### Part 1
Parse the grid column by column, identifying problem boundaries (columns of all spaces). For each problem, extract the numbers row by row and apply the operator.

```rust
// Find problem boundaries by scanning for all-space columns
// Extract numbers from each row within a problem
// Apply the operator (+/*) from the bottom row
```

### Part 2
The key insight is the completely different reading order:
1. **Columns are digits**: Each column represents one digit of a number
2. **Top-to-bottom**: Most significant digit at top, least significant at bottom
3. **Right-to-left**: Read problems starting from the rightmost column

```rust
// Scan from right to left
// For each column within a problem, build a number from digits (top to bottom)
// Each column becomes one operand
```

## Example

```
123 328  51 64
 45 64  387 23
  6 98  215 314
*   +   *   +
```

**Part 1** (normal reading):
- `123 * 45 * 6 = 33210`
- `328 + 64 + 98 = 490`
- `51 * 387 * 215 = 4243455`
- `64 + 23 + 314 = 401`
- Grand total: **4277556**

**Part 2** (cephalopod reading, right-to-left columns):
- Rightmost: column `4`, `3`, `4` -> `4 + 431 + 623 = 1058`
- Second: columns form `175 * 581 * 32 = 3253600`
- Third: `8 + 248 + 369 = 625`
- Leftmost: `356 * 24 * 1 = 8544`
- Grand total: **3263827**

## Challenges and Confusing Parts

1. **Ragged lines**: Lines may have different lengths, requiring padding to align columns properly.

2. **Part 2 paradigm shift**: The completely different reading order in Part 2 was initially confusing. Understanding that each column becomes a multi-digit number (not each row) was the key insight.

3. **Column vs Row confusion**: In Part 1, numbers span across columns (horizontal). In Part 2, numbers are built from a single column (vertical digits). This reversal required rewriting the parsing logic entirely.

4. **Right-to-left iteration**: Iterating columns from right to left with proper boundary detection required careful index handling with signed integers to avoid underflow.

## Answers

- Part 1: **8108520669952**
- Part 2: **11708563470209**

---

*Solution by Claude (claude-opus-4-5-20251101)*
