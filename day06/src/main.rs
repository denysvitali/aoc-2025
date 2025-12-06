use common::{read_input, run_day};

fn part1(input: &str) -> i64 {
    let lines: Vec<&str> = input.lines().collect();
    if lines.is_empty() {
        return 0;
    }

    // Find the maximum line length to handle ragged lines
    let max_len = lines.iter().map(|l| l.len()).max().unwrap_or(0);

    // Pad all lines to the same length
    let padded_lines: Vec<String> = lines
        .iter()
        .map(|l| format!("{:width$}", l, width = max_len))
        .collect();

    // The last line contains the operators
    let operator_line = &padded_lines[padded_lines.len() - 1];
    let number_lines = &padded_lines[..padded_lines.len() - 1];

    // Find problem boundaries by looking for columns that are all spaces in number lines
    // Problems are separated by full columns of spaces

    let mut problems: Vec<(Vec<i64>, char)> = Vec::new();
    let mut col = 0;

    while col < max_len {
        // Skip separator columns (all spaces in number lines)
        while col < max_len
            && number_lines
                .iter()
                .all(|l| l.chars().nth(col).unwrap_or(' ') == ' ')
        {
            col += 1;
        }

        if col >= max_len {
            break;
        }

        // Find the end of this problem (next all-space column or end)
        let start_col = col;
        while col < max_len
            && !number_lines
                .iter()
                .all(|l| l.chars().nth(col).unwrap_or(' ') == ' ')
        {
            col += 1;
        }
        let end_col = col;

        // Extract the numbers from this problem
        let mut numbers: Vec<i64> = Vec::new();
        for line in number_lines {
            let segment: String = line
                .chars()
                .skip(start_col)
                .take(end_col - start_col)
                .collect();
            let trimmed = segment.trim();
            if !trimmed.is_empty() {
                if let Ok(n) = trimmed.parse::<i64>() {
                    numbers.push(n);
                }
            }
        }

        // Find the operator for this problem (should be in the operator line within this column range)
        let op_segment: String = operator_line
            .chars()
            .skip(start_col)
            .take(end_col - start_col)
            .collect();
        let operator = op_segment
            .chars()
            .find(|&c| c == '+' || c == '*')
            .unwrap_or('+');

        if !numbers.is_empty() {
            problems.push((numbers, operator));
        }
    }

    // Calculate the result for each problem
    let mut grand_total: i64 = 0;
    for (numbers, op) in problems {
        let result = match op {
            '+' => numbers.iter().sum(),
            '*' => numbers.iter().product(),
            _ => 0,
        };
        grand_total += result;
    }

    grand_total
}

fn part2(input: &str) -> i64 {
    // TODO: Implement part 2
    let _lines: Vec<&str> = input.lines().collect();
    0
}

fn main() {
    let input = read_input(6);
    let result = run_day(6, &input, part1, part2);
    result.print(6);
}

#[cfg(test)]
mod tests {
    use super::*;
    use common::read_example;

    #[test]
    fn test_part1_example() {
        let input = read_example(6);
        assert_eq!(part1(&input), 4277556);
    }

    #[test]
    fn test_part2_example() {
        let input = read_example(6);
        assert_eq!(part2(&input), 0); // TODO: Update expected value
    }

    #[test]
    fn test_part1() {
        let input = read_input(6);
        assert_eq!(part1(&input), 0); // TODO: Update expected value after solving
    }

    #[test]
    fn test_part2() {
        let input = read_input(6);
        assert_eq!(part2(&input), 0); // TODO: Update expected value after solving
    }
}
