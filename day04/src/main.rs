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
                    if nr >= 0
                        && nr < rows as i64
                        && nc >= 0
                        && nc < cols as i64
                        && grid[nr as usize][nc as usize] == '@'
                    {
                        adjacent += 1;
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

fn count_adjacent(grid: &[Vec<char>], r: usize, c: usize) -> i32 {
    let rows = grid.len();
    let cols = grid[0].len();
    let mut adjacent = 0;

    for dr in -1..=1 {
        for dc in -1..=1 {
            if dr == 0 && dc == 0 {
                continue;
            }
            let nr = r as i64 + dr;
            let nc = c as i64 + dc;
            if nr >= 0
                && nr < rows as i64
                && nc >= 0
                && nc < cols as i64
                && grid[nr as usize][nc as usize] == '@'
            {
                adjacent += 1;
            }
        }
    }
    adjacent
}

fn part2(input: &str) -> i64 {
    let mut grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let rows = grid.len();
    let cols = if rows > 0 { grid[0].len() } else { 0 };

    let mut total_removed = 0;

    loop {
        // Find all accessible rolls (fewer than 4 adjacent)
        let mut to_remove = Vec::new();

        for r in 0..rows {
            for c in 0..cols {
                if grid[r][c] == '@' && count_adjacent(&grid, r, c) < 4 {
                    to_remove.push((r, c));
                }
            }
        }

        if to_remove.is_empty() {
            break;
        }

        // Remove all accessible rolls
        for (r, c) in &to_remove {
            grid[*r][*c] = '.';
        }

        total_removed += to_remove.len() as i64;
    }

    total_removed
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
        assert_eq!(part2(&input), 43);
    }

    #[test]
    fn test_part1() {
        let input = read_input(4);
        assert_eq!(part1(&input), 1409);
    }

    #[test]
    fn test_part2() {
        let input = read_input(4);
        assert_eq!(part2(&input), 8366);
    }
}
