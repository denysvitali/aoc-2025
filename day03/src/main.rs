use common::{read_input, run_day};

fn part1(input: &str) -> i64 {
    // For each bank, find the maximum 2-digit joltage by picking exactly 2 batteries
    // The two digits must maintain their relative order (first picked is tens, second is ones)
    input
        .lines()
        .map(|line| {
            let digits: Vec<u32> = line.chars().filter_map(|c| c.to_digit(10)).collect();
            let mut max_joltage = 0i64;

            // Try all pairs (i, j) where i < j
            for i in 0..digits.len() {
                for j in (i + 1)..digits.len() {
                    let joltage = (digits[i] * 10 + digits[j]) as i64;
                    max_joltage = max_joltage.max(joltage);
                }
            }
            max_joltage
        })
        .sum()
}

fn part2(input: &str) -> i64 {
    // TODO: Implement part 2
    let _lines: Vec<&str> = input.lines().collect();
    0
}

fn main() {
    let input = read_input(3);
    let result = run_day(3, &input, part1, part2);
    result.print(3);
}

#[cfg(test)]
mod tests {
    use super::*;
    use common::read_example;

    #[test]
    fn test_part1_example() {
        let input = read_example(3);
        assert_eq!(part1(&input), 357); // 98 + 89 + 78 + 92 = 357
    }

    #[test]
    fn test_part2_example() {
        let input = read_example(3);
        assert_eq!(part2(&input), 0); // TODO: Update expected value
    }

    #[test]
    fn test_part1() {
        let input = read_input(3);
        assert_eq!(part1(&input), 0); // TODO: Update expected value after solving
    }

    #[test]
    fn test_part2() {
        let input = read_input(3);
        assert_eq!(part2(&input), 0); // TODO: Update expected value after solving
    }
}
