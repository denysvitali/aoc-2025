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

    for _ in 0..4 {
        orientations.insert(current.clone());
        current = rotate_90(&current);
    }

    current = flip_horizontal(shape);
    for _ in 0..4 {
        orientations.insert(current.clone());
        current = rotate_90(&current);
    }

    orientations.into_iter().collect()
}

fn parse_input(input: &str) -> (Vec<Vec<Shape>>, Vec<(usize, usize, Vec<usize>)>) {
    let parts: Vec<&str> = input.split("\n\n").collect();

    let mut shapes: Vec<Vec<Shape>> = Vec::new();
    let shapes_section = parts[0..parts.len() - 1].join("\n\n");

    for shape_block in shapes_section.split("\n\n") {
        let lines: Vec<&str> = shape_block.lines().collect();
        if lines.is_empty() {
            continue;
        }
        let shape_lines: Vec<&str> = lines[1..].to_vec();
        let shape = parse_shape(&shape_lines);
        let orientations = get_all_orientations(&shape);
        shapes.push(orientations);
    }

    let mut regions = Vec::new();
    let regions_section = parts[parts.len() - 1];

    for line in regions_section.lines() {
        if line.is_empty() {
            continue;
        }
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

// Find the first empty cell in row-major order
fn find_first_empty(grid: &[Vec<bool>], width: usize, height: usize) -> Option<(usize, usize)> {
    for y in 0..height {
        for x in 0..width {
            if !grid[y][x] {
                return Some((x, y));
            }
        }
    }
    None
}

// Check if shape can be placed with offset at (pos_x, pos_y)
fn can_place_shape(
    grid: &[Vec<bool>],
    shape: &Shape,
    pos_x: i32,
    pos_y: i32,
    width: usize,
    height: usize,
) -> bool {
    for &(dx, dy) in shape {
        let x = pos_x + dx;
        let y = pos_y + dy;
        if x < 0 || y < 0 || x >= width as i32 || y >= height as i32 {
            return false;
        }
        if grid[y as usize][x as usize] {
            return false;
        }
    }
    true
}

fn place_shape(grid: &mut [Vec<bool>], shape: &Shape, pos_x: i32, pos_y: i32) {
    for &(dx, dy) in shape {
        let x = (pos_x + dx) as usize;
        let y = (pos_y + dy) as usize;
        grid[y][x] = true;
    }
}

fn remove_shape(grid: &mut [Vec<bool>], shape: &Shape, pos_x: i32, pos_y: i32) {
    for &(dx, dy) in shape {
        let x = (pos_x + dx) as usize;
        let y = (pos_y + dy) as usize;
        grid[y][x] = false;
    }
}

// Get all ways to place a shape so that it covers a specific cell (target_x, target_y)
fn placements_covering_cell(
    shape: &Shape,
    target_x: usize,
    target_y: usize,
    grid: &[Vec<bool>],
    width: usize,
    height: usize,
) -> Vec<(i32, i32)> {
    let mut placements = Vec::new();

    for &(dx, dy) in shape {
        let pos_x = target_x as i32 - dx;
        let pos_y = target_y as i32 - dy;

        if can_place_shape(grid, shape, pos_x, pos_y, width, height) {
            placements.push((pos_x, pos_y));
        }
    }

    placements
}

// Solve using first-empty-cell strategy (exact fit required)
fn solve_exact(
    grid: &mut Vec<Vec<bool>>,
    shapes_remaining: &mut Vec<usize>,
    all_orientations: &[Vec<Shape>],
    width: usize,
    height: usize,
) -> bool {
    // Find first empty cell
    let (target_x, target_y) = match find_first_empty(grid, width, height) {
        Some(pos) => pos,
        None => return true, // Grid is full, success!
    };

    // Try each shape type that has remaining pieces
    for shape_idx in 0..shapes_remaining.len() {
        if shapes_remaining[shape_idx] == 0 {
            continue;
        }

        // Try each orientation
        for orientation in &all_orientations[shape_idx] {
            let placements =
                placements_covering_cell(orientation, target_x, target_y, grid, width, height);

            for (pos_x, pos_y) in placements {
                place_shape(grid, orientation, pos_x, pos_y);
                shapes_remaining[shape_idx] -= 1;

                if solve_exact(grid, shapes_remaining, all_orientations, width, height) {
                    return true;
                }

                shapes_remaining[shape_idx] += 1;
                remove_shape(grid, orientation, pos_x, pos_y);
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
    // All shapes have same size
    let shape_size = all_shapes[0][0].len();
    let total_cells: usize = width * height;
    let needed_cells: usize = counts.iter().sum::<usize>() * shape_size;

    // Basic area check - must have enough space
    if needed_cells > total_cells {
        return false;
    }

    // If exact fit is required (area matches exactly), use efficient solver
    if needed_cells == total_cells {
        let mut shapes_remaining: Vec<usize> = counts.to_vec();
        let mut grid = vec![vec![false; width]; height];
        return solve_exact(&mut grid, &mut shapes_remaining, all_shapes, width, height);
    }

    // For non-exact fits, we need a different approach
    // This is expensive but necessary for correctness
    let mut shapes_remaining: Vec<usize> = counts.to_vec();
    let mut grid = vec![vec![false; width]; height];
    solve_with_gaps(&mut grid, &mut shapes_remaining, all_shapes, width, height)
}

// Solve when gaps are allowed - simpler backtracking
fn solve_with_gaps(
    grid: &mut Vec<Vec<bool>>,
    shapes_remaining: &mut Vec<usize>,
    all_orientations: &[Vec<Shape>],
    width: usize,
    height: usize,
) -> bool {
    // Check if all shapes are placed
    let total_remaining: usize = shapes_remaining.iter().sum();
    if total_remaining == 0 {
        return true;
    }

    // Find first shape type with remaining pieces
    let shape_idx = shapes_remaining.iter().position(|&c| c > 0).unwrap();

    // Try each orientation
    for orientation in &all_orientations[shape_idx] {
        // Try each position
        for y in 0..height {
            for x in 0..width {
                if can_place_shape(grid, orientation, x as i32, y as i32, width, height) {
                    place_shape(grid, orientation, x as i32, y as i32);
                    shapes_remaining[shape_idx] -= 1;

                    if solve_with_gaps(grid, shapes_remaining, all_orientations, width, height) {
                        return true;
                    }

                    shapes_remaining[shape_idx] += 1;
                    remove_shape(grid, orientation, x as i32, y as i32);
                }
            }
        }
    }

    false
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
        assert_eq!(part1(&input), 599);
    }

    #[test]
    fn test_part2() {
        let input = read_input(12);
        assert_eq!(part2(&input), 0); // TODO: Update expected value after solving
    }
}
