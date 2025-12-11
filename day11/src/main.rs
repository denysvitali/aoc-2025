use common::{read_input, run_day};
use std::collections::HashMap;

fn parse_graph(input: &str) -> HashMap<&str, Vec<&str>> {
    let mut graph: HashMap<&str, Vec<&str>> = HashMap::new();

    for line in input.lines() {
        let parts: Vec<&str> = line.split(": ").collect();
        if parts.len() != 2 {
            continue;
        }
        let source = parts[0];
        let targets: Vec<&str> = parts[1].split_whitespace().collect();
        graph.insert(source, targets);
    }

    graph
}

fn count_paths<'a>(
    graph: &HashMap<&'a str, Vec<&'a str>>,
    current: &'a str,
    target: &str,
    memo: &mut HashMap<&'a str, i64>,
) -> i64 {
    if current == target {
        return 1;
    }

    if let Some(&count) = memo.get(current) {
        return count;
    }

    let count = if let Some(neighbors) = graph.get(current) {
        neighbors
            .iter()
            .map(|&next| count_paths(graph, next, target, memo))
            .sum()
    } else {
        0
    };

    memo.insert(current, count);
    count
}

fn part1(input: &str) -> i64 {
    let graph = parse_graph(input);
    let mut memo: HashMap<&str, i64> = HashMap::new();
    count_paths(&graph, "you", "out", &mut memo)
}

fn count_paths_with_required<'a>(
    graph: &HashMap<&'a str, Vec<&'a str>>,
    current: &'a str,
    target: &str,
    visited_dac: bool,
    visited_fft: bool,
    memo: &mut HashMap<(&'a str, bool, bool), i64>,
) -> i64 {
    let now_dac = visited_dac || current == "dac";
    let now_fft = visited_fft || current == "fft";

    if current == target {
        return if now_dac && now_fft { 1 } else { 0 };
    }

    let key = (current, now_dac, now_fft);
    if let Some(&count) = memo.get(&key) {
        return count;
    }

    let count = if let Some(neighbors) = graph.get(current) {
        neighbors
            .iter()
            .map(|&next| count_paths_with_required(graph, next, target, now_dac, now_fft, memo))
            .sum()
    } else {
        0
    };

    memo.insert(key, count);
    count
}

fn part2(input: &str) -> i64 {
    let graph = parse_graph(input);
    let mut memo: HashMap<(&str, bool, bool), i64> = HashMap::new();
    count_paths_with_required(&graph, "svr", "out", false, false, &mut memo)
}

fn main() {
    let input = read_input(11);
    let result = run_day(11, &input, part1, part2);
    result.print(11);
}

#[cfg(test)]
mod tests {
    use super::*;
    use common::read_example;

    #[test]
    fn test_part1_example() {
        let input = read_example(11);
        assert_eq!(part1(&input), 5);
    }

    #[test]
    fn test_part2_example() {
        let input = std::fs::read_to_string("input/example2.txt").unwrap();
        assert_eq!(part2(&input), 2);
    }

    #[test]
    fn test_part1() {
        let input = read_input(11);
        assert_eq!(part1(&input), 599);
    }

    #[test]
    fn test_part2() {
        let input = read_input(11);
        assert_eq!(part2(&input), 393474305030400);
    }
}
