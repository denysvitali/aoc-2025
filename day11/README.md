# Day 11: Reactor

## Problem Description

The factory reactor needs to communicate with a new server rack. Devices are connected in a directed graph, and data flows through paths from source to destination.

## Part 1: Count All Paths

Count all paths from device `you` to device `out` in the device graph.

**Approach**: Classic path counting in a DAG using memoized DFS. For each node, recursively count paths through all its neighbors and cache results to avoid recomputation.

## Part 2: Paths Through Required Nodes

Count all paths from device `svr` to device `out` that pass through both `dac` (digital-to-analog converter) and `fft` (fast Fourier transform) in any order.

**Approach**: Extend the memoization to include state tracking for which required nodes have been visited. The memo key becomes `(node, visited_dac, visited_fft)`, giving 4x the state space but still O(n) in the number of nodes.

### Key Insight

The graph is a DAG (directed acyclic graph), so memoization is safe - we won't visit the same node twice in a single path. The state tuple tracks which required nodes we've passed through, allowing paths that visit the required nodes in either order.

## Complexity

- **Part 1**: O(V + E) where V is nodes and E is edges
- **Part 2**: O(4 * (V + E)) due to the 4 possible states for (visited_dac, visited_fft)

## Performance

- Part 1: ~508µs
- Part 2: ~923µs

## Answers

- Part 1: **599**
- Part 2: **393474305030400**
