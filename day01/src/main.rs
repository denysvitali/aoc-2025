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
    let mut position: i64 = 50; // Raw position (not modulo), starts at 50
    let mut count: i64 = 0;

    for line in input.lines() {
        let direction = &line[0..1];
        let distance: i64 = line[1..].parse().unwrap();

        let new_position = match direction {
            "L" => position - distance,
            "R" => position + distance,
            _ => panic!("Unknown direction: {}", direction),
        };

        // Count multiples of 100 in range [low, high], excluding start position
        let (low, high) = if position <= new_position {
            (position, new_position)
        } else {
            (new_position, position)
        };

        // Count multiples of 100 in [low, high]
        // = floor(high/100) - floor((low-1)/100)
        let multiples_in_range = high.div_euclid(100) - (low - 1).div_euclid(100);

        // Exclude start position if it's on a multiple of 100 (we don't count starting there)
        let exclude_start = if position.rem_euclid(100) == 0 { 1 } else { 0 };

        let crossings = (multiples_in_range - exclude_start).max(0);

        count += crossings;
        position = new_position; // Keep raw position for next iteration
    }

    count
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
        assert_eq!(part2(&input), 6);
    }

    #[test]
    fn test_part1() {
        let input = read_input(1);
        assert_eq!(part1(&input), 1105);
    }

    #[test]
    fn test_part2() {
        let input = read_input(1);
        assert_eq!(part2(&input), 6599);
    }
}
