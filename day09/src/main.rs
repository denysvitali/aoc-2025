use common::{read_input, run_day};

fn parse_tiles(input: &str) -> Vec<(i64, i64)> {
    input
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| {
            let parts: Vec<&str> = line.split(',').collect();
            let x = parts[0].trim().parse::<i64>().unwrap();
            let y = parts[1].trim().parse::<i64>().unwrap();
            (x, y)
        })
        .collect()
}

fn part1(input: &str) -> i64 {
    let tiles = parse_tiles(input);

    // For any pair of red tiles as opposite corners, the rectangle area is
    // the number of tiles in the rectangle, which is (|x2 - x1| + 1) * (|y2 - y1| + 1)
    // We need to find the maximum such area
    let mut max_area = 0;

    for i in 0..tiles.len() {
        for j in (i + 1)..tiles.len() {
            let (x1, y1) = tiles[i];
            let (x2, y2) = tiles[j];
            let area = ((x2 - x1).abs() + 1) * ((y2 - y1).abs() + 1);
            max_area = max_area.max(area);
        }
    }

    max_area
}

fn part2(input: &str) -> i64 {
    // TODO: Implement part 2
    let _lines: Vec<&str> = input.lines().collect();
    0
}

fn main() {
    let input = read_input(9);
    let result = run_day(9, &input, part1, part2);
    result.print(9);
}

#[cfg(test)]
mod tests {
    use super::*;
    use common::read_example;

    #[test]
    fn test_part1_example() {
        let input = read_example(9);
        assert_eq!(part1(&input), 50);
    }

    #[test]
    fn test_part2_example() {
        let input = read_example(9);
        assert_eq!(part2(&input), 0); // TODO: Update expected value
    }

    #[test]
    fn test_part1() {
        let input = read_input(9);
        assert_eq!(part1(&input), 0); // TODO: Update expected value after solving
    }

    #[test]
    fn test_part2() {
        let input = read_input(9);
        assert_eq!(part2(&input), 0); // TODO: Update expected value after solving
    }
}
