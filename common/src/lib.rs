//! Common utilities for Advent of Code 2025

use std::fmt::Display;
use std::fs;
use std::path::Path;
use std::time::{Duration, Instant};

/// Read input file for a given day
pub fn read_input(day: u8) -> String {
    let paths = [
        format!("day{:02}/input/input.txt", day),
        "input/input.txt".to_string(),
    ];
    for path in &paths {
        if let Ok(content) = fs::read_to_string(path) {
            return content;
        }
    }
    panic!("Failed to read input file for day {:02}", day)
}

/// Read example input file for a given day
pub fn read_example(day: u8) -> String {
    let paths = [
        format!("day{:02}/input/example.txt", day),
        "input/example.txt".to_string(),
    ];
    for path in &paths {
        if let Ok(content) = fs::read_to_string(path) {
            return content;
        }
    }
    panic!("Failed to read example file for day {:02}", day)
}

/// Read input from a specific path
pub fn read_input_from_path<P: AsRef<Path>>(path: P) -> String {
    fs::read_to_string(path.as_ref())
        .unwrap_or_else(|_| panic!("Failed to read file: {}", path.as_ref().display()))
}

/// Result type for a day's solution
pub struct DayResult<T: Display> {
    pub part1: T,
    pub part2: T,
    pub part1_time: Duration,
    pub part2_time: Duration,
}

impl<T: Display> DayResult<T> {
    pub fn print(&self, day: u8) {
        println!("=== Day {:02} ===", day);
        println!("Part 1: {} ({:?})", self.part1, self.part1_time);
        println!("Part 2: {} ({:?})", self.part2, self.part2_time);
        println!(
            "Total time: {:?}",
            self.part1_time + self.part2_time
        );
    }
}

/// Time a function and return its result with elapsed time
pub fn timed<T, F: FnOnce() -> T>(f: F) -> (T, Duration) {
    let start = Instant::now();
    let result = f();
    let elapsed = start.elapsed();
    (result, elapsed)
}

/// Run both parts of a day's solution with timing
pub fn run_day<T, F1, F2>(_day: u8, input: &str, part1: F1, part2: F2) -> DayResult<T>
where
    T: Display,
    F1: FnOnce(&str) -> T,
    F2: FnOnce(&str) -> T,
{
    let (p1, t1) = timed(|| part1(input));
    let (p2, t2) = timed(|| part2(input));

    DayResult {
        part1: p1,
        part2: p2,
        part1_time: t1,
        part2_time: t2,
    }
}

/// Parse input lines into a vector
pub fn parse_lines(input: &str) -> Vec<&str> {
    input.lines().collect()
}

/// Parse input into a vector of parsed values
pub fn parse_lines_as<T, F>(input: &str, parser: F) -> Vec<T>
where
    F: Fn(&str) -> T,
{
    input.lines().map(parser).collect()
}

/// Parse input as a grid of characters
pub fn parse_grid(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|line| line.chars().collect()).collect()
}

/// Parse input as a grid of digits
pub fn parse_digit_grid(input: &str) -> Vec<Vec<u32>> {
    input
        .lines()
        .map(|line| line.chars().filter_map(|c| c.to_digit(10)).collect())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_lines() {
        let input = "line1\nline2\nline3";
        assert_eq!(parse_lines(input), vec!["line1", "line2", "line3"]);
    }

    #[test]
    fn test_parse_grid() {
        let input = "abc\ndef";
        let grid = parse_grid(input);
        assert_eq!(grid, vec![vec!['a', 'b', 'c'], vec!['d', 'e', 'f']]);
    }

    #[test]
    fn test_parse_digit_grid() {
        let input = "123\n456";
        let grid = parse_digit_grid(input);
        assert_eq!(grid, vec![vec![1, 2, 3], vec![4, 5, 6]]);
    }
}
