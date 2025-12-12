use common::{read_input, run_day};
use std::collections::HashSet;

type Point = (i32, i32);
type Shape = Vec<Point>;

fn parse_shape(lines: &[&str]) -> Shape {
    let mut points = Vec::new();
    for (y, line) in lines.iter().enumerate() {
        for (x, ch) in line.chars().enumerate() {
            if ch == '#' {
                points.push((x as i32, y as i32));
            }
        }
    }
    normalize_shape(&points)
}

fn normalize_shape(points: &[Point]) -> Shape {
    if points.is_empty() {
        return Vec::new();
    }
    let min_x = points.iter().map(|p| p.0).min().unwrap();
    let min_y = points.iter().map(|p| p.1).min().unwrap();
    let mut normalized: Vec<Point> = points.iter().map(|p| (p.0 - min_x, p.1 - min_y)).collect();
    normalized.sort();
    normalized
}

fn rotate_90(shape: &Shape) -> Shape {
    // (x, y) -> (y, -x) but we normalize after
    let rotated: Vec<Point> = shape.iter().map(|&(x, y)| (y, -x)).collect();
    normalize_shape(&rotated)
}

fn flip_horizontal(shape: &Shape) -> Shape {
    let flipped: Vec<Point> = shape.iter().map(|&(x, y)| (-x, y)).collect();
    normalize_shape(&flipped)
}

fn get_all_orientations(shape: &Shape) -> Vec<Shape> {
    let mut orientations = HashSet::new();
    let mut current = shape.clone();

    // 4 rotations
    for _ in 0..4 {
        orientations.insert(current.clone());
        current = rotate_90(&current);
    }

    // Flip and 4 more rotations
    current = flip_horizontal(shape);
    for _ in 0..4 {
        orientations.insert(current.clone());
        current = rotate_90(&current);
    }

    orientations.into_iter().collect()
}

fn parse_input(input: &str) -> (Vec<Vec<Shape>>, Vec<(usize, usize, Vec<usize>)>) {
    let parts: Vec<&str> = input.split("\n\n").collect();

    // Parse shapes section
    let mut shapes: Vec<Vec<Shape>> = Vec::new();
    let shapes_section = parts[0..parts.len() - 1].join("\n\n");

    for shape_block in shapes_section.split("\n\n") {
        let lines: Vec<&str> = shape_block.lines().collect();
        if lines.is_empty() {
            continue;
        }
        // First line is "N:" where N is the index
        let shape_lines: Vec<&str> = lines[1..].to_vec();
        let shape = parse_shape(&shape_lines);
        let orientations = get_all_orientations(&shape);
        shapes.push(orientations);
    }

    // Parse regions section (last part)
    let mut regions = Vec::new();
    let regions_section = parts[parts.len() - 1];

    for line in regions_section.lines() {
        if line.is_empty() {
            continue;
        }
        // Format: "WxH: n0 n1 n2 ..."
        let colon_pos = line.find(':').unwrap();
        let dims_part = &line[..colon_pos];
        let counts_part = &line[colon_pos + 1..].trim();

        let dims: Vec<usize> = dims_part.split('x').map(|s| s.parse().unwrap()).collect();
        let width = dims[0];
        let height = dims[1];

        let counts: Vec<usize> = counts_part
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();

        regions.push((width, height, counts));
    }

    (shapes, regions)
}

fn can_place_shape(
    grid: &[Vec<bool>],
    shape: &Shape,
    pos_x: usize,
    pos_y: usize,
    width: usize,
    height: usize,
) -> bool {
    for &(dx, dy) in shape {
        let x = pos_x as i32 + dx;
        let y = pos_y as i32 + dy;
        if x < 0 || y < 0 || x >= width as i32 || y >= height as i32 {
            return false;
        }
        if grid[y as usize][x as usize] {
            return false;
        }
    }
    true
}

fn place_shape(grid: &mut [Vec<bool>], shape: &Shape, pos_x: usize, pos_y: usize) {
    for &(dx, dy) in shape {
        let x = (pos_x as i32 + dx) as usize;
        let y = (pos_y as i32 + dy) as usize;
        grid[y][x] = true;
    }
}

fn remove_shape(grid: &mut [Vec<bool>], shape: &Shape, pos_x: usize, pos_y: usize) {
    for &(dx, dy) in shape {
        let x = (pos_x as i32 + dx) as usize;
        let y = (pos_y as i32 + dy) as usize;
        grid[y][x] = false;
    }
}

fn solve(
    grid: &mut Vec<Vec<bool>>,
    shapes_to_place: &[(usize, &Vec<Shape>)], // (shape_index, orientations)
    idx: usize,
    width: usize,
    height: usize,
) -> bool {
    if idx >= shapes_to_place.len() {
        return true;
    }

    let (_shape_idx, orientations) = &shapes_to_place[idx];

    for orientation in orientations.iter() {
        for y in 0..height {
            for x in 0..width {
                if can_place_shape(grid, orientation, x, y, width, height) {
                    place_shape(grid, orientation, x, y);
                    if solve(grid, shapes_to_place, idx + 1, width, height) {
                        return true;
                    }
                    remove_shape(grid, orientation, x, y);
                }
            }
        }
    }

    false
}

fn can_fit_all_presents(
    width: usize,
    height: usize,
    counts: &[usize],
    all_shapes: &[Vec<Shape>],
) -> bool {
    // Build list of shapes to place (with repetition)
    let mut shapes_to_place: Vec<(usize, &Vec<Shape>)> = Vec::new();
    for (shape_idx, &count) in counts.iter().enumerate() {
        for _ in 0..count {
            shapes_to_place.push((shape_idx, &all_shapes[shape_idx]));
        }
    }

    if shapes_to_place.is_empty() {
        return true;
    }

    // Sort by number of orientations (fewer orientations first for better pruning)
    shapes_to_place.sort_by_key(|(_, orientations)| orientations.len());

    let mut grid = vec![vec![false; width]; height];
    solve(&mut grid, &shapes_to_place, 0, width, height)
}

fn part1(input: &str) -> i64 {
    let (shapes, regions) = parse_input(input);

    let mut count = 0;
    for (width, height, counts) in &regions {
        if can_fit_all_presents(*width, *height, counts, &shapes) {
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
    let input = read_input(12);
    let result = run_day(12, &input, part1, part2);
    result.print(12);
}

#[cfg(test)]
mod tests {
    use super::*;
    use common::read_example;

    #[test]
    fn test_part1_example() {
        let input = read_example(12);
        assert_eq!(part1(&input), 2);
    }

    #[test]
    fn test_part2_example() {
        let input = read_example(12);
        assert_eq!(part2(&input), 0); // TODO: Update expected value
    }

    #[test]
    fn test_part1() {
        let input = read_input(12);
        assert_eq!(part1(&input), 0); // TODO: Update expected value after solving
    }

    #[test]
    fn test_part2() {
        let input = read_input(12);
        assert_eq!(part2(&input), 0); // TODO: Update expected value after solving
    }
}
