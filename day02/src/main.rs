use common::{read_input, run_day};

/// Check if a number is "invalid" - meaning it's a repeated sequence of digits.
/// E.g., 11 (5 repeated), 6464 (64 repeated), 123123 (123 repeated)
fn is_invalid_id(n: u64) -> bool {
    let s = n.to_string();
    let len = s.len();

    // Must have even length to be a repeated sequence
    if !len.is_multiple_of(2) {
        return false;
    }

    let half = len / 2;
    let (first_half, second_half) = s.split_at(half);

    first_half == second_half
}

/// Parse input and find all invalid IDs in the ranges
fn find_invalid_ids(input: &str) -> Vec<u64> {
    let mut invalid_ids = Vec::new();

    // Parse ranges from comma-separated input (may be on multiple lines)
    let cleaned = input.replace('\n', "");
    for range_str in cleaned.trim().split(',') {
        let range_str = range_str.trim();
        if range_str.is_empty() {
            continue;
        }

        let parts: Vec<&str> = range_str.split('-').collect();
        if parts.len() != 2 {
            continue;
        }

        let start: u64 = parts[0].parse().unwrap();
        let end: u64 = parts[1].parse().unwrap();

        // Check each number in the range
        for n in start..=end {
            if is_invalid_id(n) {
                invalid_ids.push(n);
            }
        }
    }

    invalid_ids
}

fn part1(input: &str) -> i64 {
    let invalid_ids = find_invalid_ids(input);
    invalid_ids.iter().sum::<u64>() as i64
}

fn part2(input: &str) -> i64 {
    // TODO: Implement part 2
    let _lines: Vec<&str> = input.lines().collect();
    0
}

fn main() {
    let input = read_input(2);
    let result = run_day(2, &input, part1, part2);
    result.print(2);
}

#[cfg(test)]
mod tests {
    use super::*;
    use common::read_example;

    #[test]
    fn test_part1_example() {
        let input = read_example(2);
        assert_eq!(part1(&input), 1227775554); // Sum of all invalid IDs in example
    }

    #[test]
    fn test_part2_example() {
        let input = read_example(2);
        assert_eq!(part2(&input), 0); // TODO: Update expected value
    }

    #[test]
    fn test_part1() {
        let input = read_input(2);
        assert_eq!(part1(&input), 29818212493);
    }

    #[test]
    fn test_part2() {
        let input = read_input(2);
        assert_eq!(part2(&input), 0); // TODO: Update expected value after solving
    }
}
