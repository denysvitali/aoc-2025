use common::{read_input, run_day};

fn part1(input: &str) -> i64 {
    let mut position: i64 = 50;
    let mut count = 0;

    for line in input.lines() {
        let direction = &line[0..1];
        let distance: i64 = line[1..].parse().unwrap();

        match direction {
            "L" => position -= distance,
            "R" => position += distance,
            _ => panic!("Unknown direction: {}", direction),
        }

        // Wrap around to 0-99 range
        position = position.rem_euclid(100);

        if position == 0 {
            count += 1;
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
    let input = read_input(1);
    let result = run_day(1, &input, part1, part2);
    result.print(1);
}

#[cfg(test)]
mod tests {
    use super::*;
    use common::read_example;

    #[test]
    fn test_part1_example() {
        let input = read_example(1);
        assert_eq!(part1(&input), 3);
    }

    #[test]
    fn test_part2_example() {
        let input = read_example(1);
        assert_eq!(part2(&input), 0); // TODO: Update expected value
    }

    #[test]
    fn test_part1() {
        let input = read_input(1);
        assert_eq!(part1(&input), 1105);
    }

    #[test]
    fn test_part2() {
        let input = read_input(1);
        assert_eq!(part2(&input), 0); // TODO: Update expected value after solving
    }
}
