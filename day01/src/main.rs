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
    let mut position: i64 = 50;
    let mut count: i64 = 0;

    for line in input.lines() {
        let direction = &line[0..1];
        let distance: i64 = line[1..].parse().unwrap();

        let new_position = match direction {
            "L" => position - distance,
            "R" => position + distance,
            _ => panic!("Unknown direction: {}", direction),
        };

        // Count how many times we pass through 0
        // For movement from position to new_position (before modulo):
        // - Moving left (decreasing): we pass 0 when crossing from 0 to -1 (which is 99)
        // - Moving right (increasing): we pass 0 when crossing from 99 to 100 (which is 0)

        // Calculate crossings based on raw positions (before modulo)
        // Every time we cross a multiple of 100 boundary, we pass through 0

        let (low, high) = if position <= new_position {
            (position, new_position)
        } else {
            (new_position, position)
        };

        // Count multiples of 100 in the range (low, high]
        // We want to count 0, 100, 200, -100, -200, etc. that are in range
        // But we need to be careful: landing exactly on 0 counts, passing through 0 counts

        // Simpler approach: how many times do we cross a multiple of 100?
        // floor(high / 100) - floor((low) / 100) gives us the count if low < high
        // But we need to handle the case where low itself is a multiple of 100

        // Actually, let's think differently:
        // The dial passes through 0 each time we complete a "lap"
        // Plus if we end on 0

        // Number of complete crossings of 0:
        // (high - 1) / 100 - (low - 1) / 100 when moving through positive direction
        // But this gets complicated with negative numbers

        // Simpler: count how many multiples of 100 are strictly between low and high,
        // plus 1 if high is exactly a multiple of 100

        let crossings = if low == high {
            0
        } else {
            // Use div_euclid for correct floor division with negatives
            let low_div = (low - 1).div_euclid(100);
            let high_div = (high - 1).div_euclid(100);
            high_div - low_div
        };

        count += crossings;
        position = new_position.rem_euclid(100);
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
        assert_eq!(part2(&input), 6595);
    }
}
