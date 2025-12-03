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
    // Select exactly 12 batteries to form the maximum 12-digit number
    // Greedy approach: at each position, pick the largest digit possible
    // while ensuring enough digits remain for the rest
    input
        .lines()
        .map(|line| {
            let digits: Vec<u64> = line
                .chars()
                .filter_map(|c| c.to_digit(10).map(|d| d as u64))
                .collect();
            let n = digits.len();
            let k = 12; // need to pick 12 digits

            let mut result: u64 = 0;
            let mut start = 0; // current starting position to search from

            for i in 0..k {
                // Need to pick (k - i) more digits including this one
                // So we can search up to index n - (k - i)
                let remaining_needed = k - i;
                let end = n - remaining_needed; // inclusive end position we can pick from

                // Find the maximum digit in range [start, end]
                let mut max_digit = 0;
                let mut max_pos = start;
                for (pos, &digit) in digits.iter().enumerate().take(end + 1).skip(start) {
                    if digit > max_digit {
                        max_digit = digit;
                        max_pos = pos;
                    }
                }

                result = result * 10 + max_digit;
                start = max_pos + 1; // next search starts after the picked position
            }

            result as i64
        })
        .sum()
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
        assert_eq!(part2(&input), 3121910778619); // 987654321111 + 811111111119 + 434234234278 + 888911112111
    }

    #[test]
    fn test_part1() {
        let input = read_input(3);
        assert_eq!(part1(&input), 17405);
    }

    #[test]
    fn test_part2() {
        let input = read_input(3);
        assert_eq!(part2(&input), 171990312704598);
    }
}
