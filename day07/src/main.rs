use common::{read_input, run_day};
use std::collections::HashSet;

fn part1(input: &str) -> i64 {
    let grid: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();
    let rows = grid.len();
    let cols = if rows > 0 { grid[0].len() } else { 0 };

    // Find starting position S
    let mut start_col = 0;
    for row in grid.iter() {
        for (c, &ch) in row.iter().enumerate() {
            if ch == 'S' {
                start_col = c;
                break;
            }
        }
    }

    // Track active beam columns at each row level
    // Start with beam at S column, moving down from row 0
    let mut beams: HashSet<usize> = HashSet::new();
    beams.insert(start_col);

    let mut splits = 0;

    // Process each row starting from row 1 (after S)
    for row_idx in 1..rows {
        let mut new_beams: HashSet<usize> = HashSet::new();

        for &col in &beams {
            let ch = grid[row_idx][col];
            if ch == '^' {
                // Split: beam stops, creates left and right beams
                splits += 1;
                if col > 0 {
                    new_beams.insert(col - 1);
                }
                if col + 1 < cols {
                    new_beams.insert(col + 1);
                }
            } else {
                // Continue downward
                new_beams.insert(col);
            }
        }

        beams = new_beams;
    }

    splits
}

fn part2(input: &str) -> i64 {
    // TODO: Implement part 2
    let _lines: Vec<&str> = input.lines().collect();
    0
}

fn main() {
    let input = read_input(7);
    let result = run_day(7, &input, part1, part2);
    result.print(7);
}

#[cfg(test)]
mod tests {
    use super::*;
    use common::read_example;

    #[test]
    fn test_part1_example() {
        let input = read_example(7);
        assert_eq!(part1(&input), 21);
    }

    #[test]
    fn test_part2_example() {
        let input = read_example(7);
        assert_eq!(part2(&input), 0); // TODO: Update expected value
    }

    #[test]
    fn test_part1() {
        let input = read_input(7);
        assert_eq!(part1(&input), 0); // TODO: Update expected value after solving
    }

    #[test]
    fn test_part2() {
        let input = read_input(7);
        assert_eq!(part2(&input), 0); // TODO: Update expected value after solving
    }
}
