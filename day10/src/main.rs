use common::{read_input, run_day};

/// Parse a machine definition line into (target_lights, buttons, joltage)
/// target_lights: Vec<bool> where true = light should be ON
/// buttons: Vec<Vec<usize>> where each inner vec is the indices affected by that button
/// joltage: Vec<u64> target joltage values for each counter
fn parse_machine(line: &str) -> (Vec<bool>, Vec<Vec<usize>>, Vec<u64>) {
    // Parse indicator lights [.##.]
    let bracket_start = line.find('[').unwrap();
    let bracket_end = line.find(']').unwrap();
    let lights_str = &line[bracket_start + 1..bracket_end];
    let target_lights: Vec<bool> = lights_str.chars().map(|c| c == '#').collect();

    // Parse buttons (indices) - find all parenthesized groups
    let mut buttons = Vec::new();
    let rest = &line[bracket_end + 1..];

    // Find the curly brace to know where buttons end
    let curly_pos = rest.find('{').unwrap_or(rest.len());
    let buttons_section = &rest[..curly_pos];

    let mut i = 0;
    let chars: Vec<char> = buttons_section.chars().collect();
    while i < chars.len() {
        if chars[i] == '(' {
            // Find matching close paren
            let start = i + 1;
            while i < chars.len() && chars[i] != ')' {
                i += 1;
            }
            let content: String = chars[start..i].iter().collect();
            // Parse comma-separated indices
            let indices: Vec<usize> = content
                .split(',')
                .map(|s| s.trim().parse().unwrap())
                .collect();
            buttons.push(indices);
        }
        i += 1;
    }

    // Parse joltage requirements {3,5,4,7}
    let curly_start = line.find('{').unwrap();
    let curly_end = line.find('}').unwrap();
    let joltage_str = &line[curly_start + 1..curly_end];
    let joltage: Vec<u64> = joltage_str
        .split(',')
        .map(|s| s.trim().parse().unwrap())
        .collect();

    (target_lights, buttons, joltage)
}

/// Find minimum number of button presses to achieve target light configuration
/// This is a subset-sum problem in GF(2), looking for minimum weight solution
fn min_presses(target: &[bool], buttons: &[Vec<usize>]) -> u64 {
    let n_lights = target.len();
    let n_buttons = buttons.len();

    // Convert target to bitmask
    let mut target_mask: u64 = 0;
    for (i, &on) in target.iter().enumerate() {
        if on {
            target_mask |= 1u64 << i;
        }
    }

    // Convert each button to a bitmask
    let button_masks: Vec<u64> = buttons
        .iter()
        .map(|indices| {
            let mut mask: u64 = 0;
            for &idx in indices {
                if idx < n_lights {
                    mask |= 1u64 << idx;
                }
            }
            mask
        })
        .collect();

    // Brute force: try all 2^n_buttons combinations, find one matching target with minimum popcount
    // This works for small n_buttons (up to ~20 or so)
    if n_buttons <= 25 {
        let mut min_presses = u64::MAX;

        for combo in 0u64..(1u64 << n_buttons) {
            let mut result: u64 = 0;
            for (i, &mask) in button_masks.iter().enumerate() {
                if combo & (1u64 << i) != 0 {
                    result ^= mask;
                }
            }
            if result == target_mask {
                let presses = combo.count_ones() as u64;
                min_presses = min_presses.min(presses);
            }
        }

        if min_presses == u64::MAX {
            // No solution found - shouldn't happen for valid input
            0
        } else {
            min_presses
        }
    } else {
        // For larger inputs, use meet-in-the-middle
        meet_in_the_middle(target_mask, &button_masks)
    }
}

/// Meet-in-the-middle approach for larger button counts
fn meet_in_the_middle(target: u64, button_masks: &[u64]) -> u64 {
    use std::collections::HashMap;

    let n = button_masks.len();
    let half = n / 2;

    // First half: map XOR result -> minimum presses to achieve it
    let mut first_half: HashMap<u64, u32> = HashMap::new();
    for combo in 0u64..(1u64 << half) {
        let mut result: u64 = 0;
        for (i, &mask) in button_masks.iter().enumerate().take(half) {
            if combo & (1u64 << i) != 0 {
                result ^= mask;
            }
        }
        let presses = combo.count_ones();
        first_half
            .entry(result)
            .and_modify(|e| *e = (*e).min(presses))
            .or_insert(presses);
    }

    // Second half: for each combination, check what first-half result we need
    let mut min_presses = u64::MAX;
    let second_half_size = n - half;
    for combo in 0u64..(1u64 << second_half_size) {
        let mut result: u64 = 0;
        for i in 0..second_half_size {
            if combo & (1u64 << i) != 0 {
                result ^= button_masks[half + i];
            }
        }
        // We need first_half XOR result = target, so first_half = target XOR result
        let needed = target ^ result;
        if let Some(&first_presses) = first_half.get(&needed) {
            let total = first_presses as u64 + combo.count_ones() as u64;
            min_presses = min_presses.min(total);
        }
    }

    if min_presses == u64::MAX {
        0
    } else {
        min_presses
    }
}

fn part1(input: &str) -> i64 {
    let mut total = 0u64;
    for line in input.lines() {
        if line.trim().is_empty() {
            continue;
        }
        let (target, buttons, _) = parse_machine(line);
        total += min_presses(&target, &buttons);
    }
    total as i64
}

/// Find minimum button presses to reach target joltage values
/// Each button increments certain counters by 1
/// This is an Integer Linear Programming problem: minimize sum(x_i) subject to A*x = b, x >= 0
fn min_presses_joltage(target: &[u64], buttons: &[Vec<usize>]) -> u64 {
    let n_counters = target.len();
    let n_buttons = buttons.len();

    if n_buttons == 0 {
        return if target.iter().all(|&t| t == 0) {
            0
        } else {
            u64::MAX
        };
    }

    // Build matrix A where A[i][j] = 1 if button j affects counter i
    let mut matrix: Vec<Vec<i64>> = vec![vec![0; n_buttons]; n_counters];
    for (j, button) in buttons.iter().enumerate() {
        for &counter_idx in button {
            if counter_idx < n_counters {
                matrix[counter_idx][j] = 1;
            }
        }
    }

    let target_i64: Vec<i64> = target.iter().map(|&t| t as i64).collect();

    // Use Gaussian elimination to solve the system, then search for minimum sum solution
    solve_min_sum_ilp(&matrix, &target_i64, n_buttons, n_counters)
}

/// Solve Ax = b for non-negative integers x, minimizing sum(x)
/// Uses Gaussian elimination to find particular solution + null space, then searches
fn solve_min_sum_ilp(
    matrix: &[Vec<i64>],
    target: &[i64],
    n_buttons: usize,
    n_counters: usize,
) -> u64 {
    // Augmented matrix [A | b]
    let mut aug: Vec<Vec<i64>> = matrix
        .iter()
        .zip(target)
        .map(|(row, &t)| {
            let mut r = row.clone();
            r.push(t);
            r
        })
        .collect();

    let n_cols = n_buttons;

    // Gaussian elimination with partial pivoting (over rationals, but we track denominators)
    // Actually, let's use integer elimination with LCM to avoid fractions

    // Track which columns are pivot columns
    let mut pivot_cols: Vec<usize> = Vec::new();
    let mut pivot_row = 0;

    for col in 0..n_cols {
        if pivot_row >= n_counters {
            break;
        }

        // Find pivot
        let pivot_idx = (pivot_row..n_counters).find(|&row| aug[row][col] != 0);

        let Some(p_idx) = pivot_idx else {
            continue;
        };

        // Swap rows
        aug.swap(pivot_row, p_idx);
        pivot_cols.push(col);

        // Eliminate other rows
        let pivot_val = aug[pivot_row][col];
        for row in 0..n_counters {
            if row != pivot_row && aug[row][col] != 0 {
                let factor = aug[row][col];
                for c in 0..=n_cols {
                    aug[row][c] = aug[row][c] * pivot_val - factor * aug[pivot_row][c];
                }
            }
        }

        pivot_row += 1;
    }

    // Check for inconsistent system
    for aug_row in aug.iter().take(n_counters).skip(pivot_row) {
        if aug_row[n_cols] != 0 {
            return u64::MAX; // No solution
        }
    }

    // Free variables are those not in pivot_cols
    let free_vars: Vec<usize> = (0..n_buttons).filter(|c| !pivot_cols.contains(c)).collect();

    let n_free = free_vars.len();

    // For each assignment of free variables, compute pivot variables
    // Then check if all are non-negative integers

    // The pivot variables are determined by:
    // x_pivot[i] = (aug[i][n_cols] - sum(aug[i][free_j] * x_free[j])) / aug[i][pivot_cols[i]]

    // We need to search over non-negative integer values for free variables
    // The range is bounded by: x_free[j] <= max_target / contribution

    let max_target = target.iter().copied().max().unwrap_or(0) as u64;

    // For small number of free variables, enumerate
    // For larger, use iterative deepening on sum

    if n_free == 0 {
        // Unique solution - check if it's valid
        let mut solution = vec![0i64; n_buttons];
        for (i, &pc) in pivot_cols.iter().enumerate() {
            let pivot_val = aug[i][pc];
            if pivot_val == 0 {
                return u64::MAX;
            }
            if aug[i][n_cols] % pivot_val != 0 {
                return u64::MAX; // Non-integer solution
            }
            solution[pc] = aug[i][n_cols] / pivot_val;
            if solution[pc] < 0 {
                return u64::MAX; // Negative solution
            }
        }
        return solution.iter().map(|&x| x as u64).sum();
    }

    // Search for minimum sum solution
    // Use iterative deepening on total sum
    let mut best_sum = u64::MAX;

    // Estimate upper bound on free variables
    let free_upper_bounds: Vec<i64> = free_vars.iter().map(|_| (max_target + 1) as i64).collect();

    // Generate combinations with increasing sum
    let ctx = SearchContext {
        aug: &aug,
        pivot_cols: &pivot_cols,
        free_vars: &free_vars,
        upper_bounds: &free_upper_bounds,
        n_buttons,
    };
    search_free_vars(&ctx, 0, &mut vec![0i64; n_free], 0, &mut best_sum);

    best_sum
}

struct SearchContext<'a> {
    aug: &'a [Vec<i64>],
    pivot_cols: &'a [usize],
    free_vars: &'a [usize],
    upper_bounds: &'a [i64],
    n_buttons: usize,
}

fn search_free_vars(
    ctx: &SearchContext,
    idx: usize,
    free_vals: &mut Vec<i64>,
    current_free_sum: u64,
    best_sum: &mut u64,
) {
    if current_free_sum >= *best_sum {
        return; // Prune - already worse than best
    }

    if idx == ctx.free_vars.len() {
        // Evaluate this assignment
        let mut solution = vec![0i64; ctx.n_buttons];

        // Set free variables
        for (i, &fv) in ctx.free_vars.iter().enumerate() {
            solution[fv] = free_vals[i];
        }

        // Compute pivot variables
        for (i, &pc) in ctx.pivot_cols.iter().enumerate() {
            let pivot_val = ctx.aug[i][pc];
            if pivot_val == 0 {
                return;
            }

            let mut rhs = ctx.aug[i][ctx.n_buttons]; // The target column
            for (j, &fv) in ctx.free_vars.iter().enumerate() {
                rhs -= ctx.aug[i][fv] * free_vals[j];
            }

            if rhs % pivot_val != 0 {
                return; // Non-integer
            }
            solution[pc] = rhs / pivot_val;
            if solution[pc] < 0 {
                return; // Negative
            }
        }

        let total: u64 = solution.iter().map(|&x| x as u64).sum();
        if total < *best_sum {
            *best_sum = total;
        }
        return;
    }

    // Try values for free_vars[idx]
    for val in 0..=ctx.upper_bounds[idx] {
        free_vals[idx] = val;
        let new_sum = current_free_sum + val as u64;
        if new_sum >= *best_sum {
            break; // Further values will only be worse
        }
        search_free_vars(ctx, idx + 1, free_vals, new_sum, best_sum);
    }
}

fn part2(input: &str) -> i64 {
    let mut total = 0u64;
    for line in input.lines() {
        if line.trim().is_empty() {
            continue;
        }
        let (_, buttons, joltage) = parse_machine(line);
        let presses = min_presses_joltage(&joltage, &buttons);
        if presses == u64::MAX {
            panic!("No solution found for line: {}", line);
        }
        total += presses;
    }
    total as i64
}

fn main() {
    let input = read_input(10);
    let result = run_day(10, &input, part1, part2);
    result.print(10);
}

#[cfg(test)]
mod tests {
    use super::*;
    use common::read_example;

    #[test]
    fn test_parse_machine() {
        let line = "[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}";
        let (target, buttons, joltage) = parse_machine(line);
        assert_eq!(target, vec![false, true, true, false]);
        assert_eq!(
            buttons,
            vec![
                vec![3],
                vec![1, 3],
                vec![2],
                vec![2, 3],
                vec![0, 2],
                vec![0, 1]
            ]
        );
        assert_eq!(joltage, vec![3, 5, 4, 7]);
    }

    #[test]
    fn test_machine1() {
        let line = "[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}";
        let (target, buttons, _) = parse_machine(line);
        assert_eq!(min_presses(&target, &buttons), 2);
    }

    #[test]
    fn test_machine2() {
        let line = "[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}";
        let (target, buttons, _) = parse_machine(line);
        assert_eq!(min_presses(&target, &buttons), 3);
    }

    #[test]
    fn test_machine3() {
        let line = "[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}";
        let (target, buttons, _) = parse_machine(line);
        assert_eq!(min_presses(&target, &buttons), 2);
    }

    #[test]
    fn test_joltage1() {
        let line = "[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}";
        let (_, buttons, joltage) = parse_machine(line);
        assert_eq!(min_presses_joltage(&joltage, &buttons), 10);
    }

    #[test]
    fn test_joltage2() {
        let line = "[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}";
        let (_, buttons, joltage) = parse_machine(line);
        assert_eq!(min_presses_joltage(&joltage, &buttons), 12);
    }

    #[test]
    fn test_joltage3() {
        let line = "[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}";
        let (_, buttons, joltage) = parse_machine(line);
        assert_eq!(min_presses_joltage(&joltage, &buttons), 11);
    }

    #[test]
    fn test_part1_example() {
        let input = read_example(10);
        assert_eq!(part1(&input), 7);
    }

    #[test]
    fn test_part2_example() {
        let input = read_example(10);
        assert_eq!(part2(&input), 33);
    }

    #[test]
    fn test_part1() {
        let input = read_input(10);
        assert_eq!(part1(&input), 449);
    }

    #[test]
    fn test_part2() {
        let input = read_input(10);
        assert_eq!(part2(&input), 17848);
    }
}
