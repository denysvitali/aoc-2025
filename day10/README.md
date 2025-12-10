# Day 10: Factory Machine Buttons

## Problem Description

Configure factory machines by pressing buttons to achieve target states. Each machine has indicator lights and joltage counters.

## Part 1: Toggle Lights

Each button toggles specific indicator lights (XOR operation). Find the minimum number of button presses to achieve a target light configuration.

**Approach**: This is a subset-sum problem in GF(2). Each button is represented as a bitmask, and we need to find the minimum-weight combination that XORs to the target.

- For small button counts (<=25): Brute force all 2^n combinations
- For larger counts: Meet-in-the-middle approach, splitting buttons in half and using a hashmap

## Part 2: Increment Counters

Each button increments specific joltage counters by 1. Find the minimum total button presses to reach exact target joltage values.

**Approach**: This is an Integer Linear Programming (ILP) problem: minimize sum(x) subject to Ax = b, x >= 0.

### Key Insight

The naive A* search over the state space fails because counter values can reach ~300, creating an enormous state space.

### Solution: Gaussian Elimination + Constrained Search

1. **Build the system**: Matrix A where A[i][j] = 1 if button j affects counter i
2. **Gaussian elimination**: Reduce to row echelon form, identifying pivot columns
3. **Identify free variables**: Columns not used as pivots
4. **Search**: Enumerate non-negative integer assignments to free variables, computing pivot variable values from the reduced system
5. **Pruning**: Skip branches where the current sum already exceeds the best found solution

This reduces complexity from exploring an exponential state space to searching over a much smaller space of free variable combinations.

## Complexity

- **Part 1**: O(2^n) or O(2^(n/2)) with meet-in-the-middle
- **Part 2**: O(k^f) where k is the max target value and f is the number of free variables (typically small after Gaussian elimination), with aggressive pruning

## Performance

- Part 1: ~9ms
- Part 2: ~777ms (release mode)

## Answers

- Part 1: **449**
- Part 2: **17848**
