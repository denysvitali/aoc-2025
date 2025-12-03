# Day 3: Lobby

## Problem Summary

The elevators are offline and the escalator needs emergency battery power. Batteries are arranged in banks (one per input line), each labeled with a joltage rating (1-9). You need to select exactly N batteries from each bank to form the largest possible number.

- **Part 1**: Select exactly **2** batteries to form the maximum 2-digit joltage.
- **Part 2**: Select exactly **12** batteries to form the maximum 12-digit joltage.

The selected batteries must maintain their relative order (no rearranging).

## Solution Approach

### Part 1
Brute force all pairs (i, j) where i < j:
```rust
for i in 0..digits.len() {
    for j in (i + 1)..digits.len() {
        let joltage = digits[i] * 10 + digits[j];
        max_joltage = max_joltage.max(joltage);
    }
}
```

### Part 2
Greedy algorithm - at each position, pick the largest digit possible while ensuring enough digits remain:

```rust
for i in 0..12 {
    // Can pick from [start, n - remaining_needed]
    let end = n - (12 - i);
    // Find max digit in range [start, end]
    // Update start to position after the picked digit
}
```

The key insight is that for position i:
- We've already picked i digits
- We need 12 - i more (including this one)
- So we can only search up to index `n - (12 - i)` to ensure enough remain

## Example

For `818181911112111` in part 2:
- Need 12 digits from 15 available
- Position 0: search [0, 3], find 8 at index 0
- Position 1: search [1, 4], find 8 at index 2
- Position 2: search [3, 5], find 8 at index 4
- Continue until we get `888911112111`

## Answers

- Part 1: **17405**
- Part 2: **171990312704598**

---

*Solution by Claude (claude-opus-4-5-20251101)*
