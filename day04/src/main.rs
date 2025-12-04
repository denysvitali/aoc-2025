use common::{read_input, run_day};

fn part1(input: &str) -> i64 {
    let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let rows = grid.len();
    let cols = if rows > 0 { grid[0].len() } else { 0 };

    let mut count = 0;

    for r in 0..rows {
        for c in 0..cols {
            if grid[r][c] != '@' {
                continue;
            }

            // Count adjacent paper rolls (8 directions)
            let mut adjacent = 0;
            for dr in -1..=1 {
                for dc in -1..=1 {
                    if dr == 0 && dc == 0 {
                        continue;
                    }
                    let nr = r as i64 + dr;
                    let nc = c as i64 + dc;
                    if nr >= 0 && nr < rows as i64 && nc >= 0 && nc < cols as i64 {
                        if grid[nr as usize][nc as usize] == '@' {
                            adjacent += 1;
                        }
                    }
                }
            }

            // Accessible if fewer than 4 adjacent rolls
            if adjacent < 4 {
                count += 1;
            }
        }
    }

    count
}

fn part2(input: &str) -> i64 {
    // TODO: Implement part 2
    let _lines: Vec<&str> = input.lines().collect();
    0
}

fn main() {
    let input = read_input(4);
    let result = run_day(4, &input, part1, part2);
    result.print(4);
}

#[cfg(test)]
mod tests {
    use super::*;
    use common::read_example;

    #[test]
    fn test_part1_example() {
        let input = read_example(4);
        assert_eq!(part1(&input), 13);
    }

    #[test]
    fn test_part2_example() {
        let input = read_example(4);
        assert_eq!(part2(&input), 0); // TODO: Update expected value
    }

    #[test]
    fn test_part1() {
        let input = read_input(4);
        assert_eq!(part1(&input), 0); // TODO: Update expected value after solving
    }

    #[test]
    fn test_part2() {
        let input = read_input(4);
        assert_eq!(part2(&input), 0); // TODO: Update expected value after solving
    }
}
