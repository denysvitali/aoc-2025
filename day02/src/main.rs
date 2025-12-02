use common::{read_input, run_day};

/// Check if a number is "invalid" for part 1 - repeated exactly twice.
/// E.g., 11 (1 repeated), 6464 (64 repeated), 123123 (123 repeated)
fn is_invalid_id_v1(n: u64) -> bool {
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

/// Check if a number is "invalid" for part 2 - repeated at least twice.
/// E.g., 111 (1 repeated 3x), 1212 (12 repeated 2x), 123123123 (123 repeated 3x)
fn is_invalid_id_v2(n: u64) -> bool {
    let s = n.to_string();
    let len = s.len();

    // Try all possible pattern lengths from 1 to len/2
    for pattern_len in 1..=len / 2 {
        // Pattern must divide evenly into the total length
        if !len.is_multiple_of(pattern_len) {
            continue;
        }

        let pattern = &s[..pattern_len];
        let repetitions = len / pattern_len;

        // Need at least 2 repetitions
        if repetitions < 2 {
            continue;
        }

        // Check if the entire string is this pattern repeated
        let mut matches = true;
        for i in 1..repetitions {
            let start = i * pattern_len;
            let end = start + pattern_len;
            if &s[start..end] != pattern {
                matches = false;
                break;
            }
        }

        if matches {
            return true;
        }
    }

    false
}

/// Parse ranges from input
fn parse_ranges(input: &str) -> Vec<(u64, u64)> {
    let mut ranges = Vec::new();
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
        ranges.push((start, end));
    }

    ranges
}

/// Find all invalid IDs using the given validation function
fn find_invalid_ids<F>(input: &str, is_invalid: F) -> Vec<u64>
where
    F: Fn(u64) -> bool,
{
    let mut invalid_ids = Vec::new();

    for (start, end) in parse_ranges(input) {
        for n in start..=end {
            if is_invalid(n) {
                invalid_ids.push(n);
            }
        }
    }

    invalid_ids
}

fn part1(input: &str) -> i64 {
    let invalid_ids = find_invalid_ids(input, is_invalid_id_v1);
    invalid_ids.iter().sum::<u64>() as i64
}

fn part2(input: &str) -> i64 {
    let invalid_ids = find_invalid_ids(input, is_invalid_id_v2);
    invalid_ids.iter().sum::<u64>() as i64
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
        assert_eq!(part2(&input), 4174379265);
    }

    #[test]
    fn test_part1() {
        let input = read_input(2);
        assert_eq!(part1(&input), 29818212493);
    }

    #[test]
    fn test_part2() {
        let input = read_input(2);
        assert_eq!(part2(&input), 37432260594);
    }
}
