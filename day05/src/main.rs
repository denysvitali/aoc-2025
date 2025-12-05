use common::{read_input, run_day};
use std::ops::RangeInclusive;

fn parse_input(input: &str) -> (Vec<RangeInclusive<i64>>, Vec<i64>) {
    let parts: Vec<&str> = input.split("\n\n").collect();

    let ranges: Vec<RangeInclusive<i64>> = parts[0]
        .lines()
        .map(|line| {
            let nums: Vec<i64> = line.split('-').map(|n| n.parse().unwrap()).collect();
            nums[0]..=nums[1]
        })
        .collect();

    let ingredients: Vec<i64> = parts[1]
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| line.parse().unwrap())
        .collect();

    (ranges, ingredients)
}

fn is_fresh(id: i64, ranges: &[RangeInclusive<i64>]) -> bool {
    ranges.iter().any(|range| range.contains(&id))
}

fn part1(input: &str) -> i64 {
    let (ranges, ingredients) = parse_input(input);
    ingredients
        .iter()
        .filter(|&&id| is_fresh(id, &ranges))
        .count() as i64
}

fn part2(input: &str) -> i64 {
    // TODO: Implement part 2
    let _lines: Vec<&str> = input.lines().collect();
    0
}

fn main() {
    let input = read_input(5);
    let result = run_day(5, &input, part1, part2);
    result.print(5);
}

#[cfg(test)]
mod tests {
    use super::*;
    use common::read_example;

    #[test]
    fn test_part1_example() {
        let input = read_example(5);
        assert_eq!(part1(&input), 3);
    }

    #[test]
    fn test_part2_example() {
        let input = read_example(5);
        assert_eq!(part2(&input), 0); // TODO: Update expected value
    }

    #[test]
    fn test_part1() {
        let input = read_input(5);
        assert_eq!(part1(&input), 0); // TODO: Update expected value after solving
    }

    #[test]
    fn test_part2() {
        let input = read_input(5);
        assert_eq!(part2(&input), 0); // TODO: Update expected value after solving
    }
}
