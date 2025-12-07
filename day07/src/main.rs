use common::{read_input, run_day};
use std::collections::HashSet;

fn part1(input: &str) -> i64 {
    let grid: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();

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
    // Beams at same position merge into one
    let mut beams: HashSet<usize> = HashSet::new();
    beams.insert(start_col);

    let mut splits = 0;

    // Process each row starting from row 1 (after S)
    for row in grid.iter().skip(1) {
        let row_len = row.len();
        let mut new_beams: HashSet<usize> = HashSet::new();

        for &col in &beams {
            // Handle ragged lines - if column is beyond this row, beam exits
            if col >= row_len {
                continue;
            }
            let ch = row[col];
            if ch == '^' {
                // Split: beam stops, creates left and right beams
                splits += 1;
                if col > 0 {
                    new_beams.insert(col - 1);
                }
                new_beams.insert(col + 1);
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
    use std::collections::HashMap;

    let grid: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();

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

    // Track number of timelines at each column position
    // When a particle hits a splitter, each timeline splits into 2
    let mut timelines: HashMap<usize, i64> = HashMap::new();
    timelines.insert(start_col, 1);

    // Process each row starting from row 1 (after S)
    for row in grid.iter().skip(1) {
        let row_len = row.len();
        let mut new_timelines: HashMap<usize, i64> = HashMap::new();

        for (&col, &count) in &timelines {
            // Handle ragged lines - if column is beyond this row, timelines exit
            if col >= row_len {
                continue;
            }
            let ch = row[col];
            if ch == '^' {
                // Split: each timeline becomes 2 (one left, one right)
                if col > 0 {
                    *new_timelines.entry(col - 1).or_insert(0) += count;
                }
                *new_timelines.entry(col + 1).or_insert(0) += count;
            } else {
                // Continue downward
                *new_timelines.entry(col).or_insert(0) += count;
            }
        }

        timelines = new_timelines;
    }

    // Sum all timelines
    timelines.values().sum()
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
        assert_eq!(part2(&input), 40);
    }

    #[test]
    fn test_part1() {
        let input = read_input(7);
        assert_eq!(part1(&input), 1535);
    }

    #[test]
    fn test_part2() {
        let input = read_input(7);
        assert_eq!(part2(&input), 4404709551015);
    }
}
