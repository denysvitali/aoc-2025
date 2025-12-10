use common::{read_input, run_day};

/// Parse a machine definition line into (target_lights, buttons)
/// target_lights: Vec<bool> where true = light should be ON
/// buttons: Vec<Vec<usize>> where each inner vec is the indices toggled by that button
fn parse_machine(line: &str) -> (Vec<bool>, Vec<Vec<usize>>) {
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

    (target_lights, buttons)
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
        for i in 0..half {
            if combo & (1u64 << i) != 0 {
                result ^= button_masks[i];
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
        let (target, buttons) = parse_machine(line);
        total += min_presses(&target, &buttons);
    }
    total as i64
}

fn part2(input: &str) -> i64 {
    // TODO: Implement part 2
    let _lines: Vec<&str> = input.lines().collect();
    0
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
        let (target, buttons) = parse_machine(line);
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
    }

    #[test]
    fn test_machine1() {
        let line = "[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}";
        let (target, buttons) = parse_machine(line);
        assert_eq!(min_presses(&target, &buttons), 2);
    }

    #[test]
    fn test_machine2() {
        let line = "[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}";
        let (target, buttons) = parse_machine(line);
        assert_eq!(min_presses(&target, &buttons), 3);
    }

    #[test]
    fn test_machine3() {
        let line = "[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}";
        let (target, buttons) = parse_machine(line);
        assert_eq!(min_presses(&target, &buttons), 2);
    }

    #[test]
    fn test_part1_example() {
        let input = read_example(10);
        assert_eq!(part1(&input), 7);
    }

    #[test]
    fn test_part2_example() {
        let input = read_example(10);
        assert_eq!(part2(&input), 0); // TODO: Update expected value
    }

    #[test]
    fn test_part1() {
        let input = read_input(10);
        assert_eq!(part1(&input), 0); // TODO: Update expected value after solving
    }

    #[test]
    fn test_part2() {
        let input = read_input(10);
        assert_eq!(part2(&input), 0); // TODO: Update expected value after solving
    }
}
