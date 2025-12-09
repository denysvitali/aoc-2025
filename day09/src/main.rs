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

// Represents a segment of the polygon boundary (either horizontal or vertical)
#[derive(Debug, Clone)]
struct Segment {
    x1: i64,
    y1: i64,
    x2: i64,
    y2: i64,
    is_vertical: bool,
}

fn build_segments(red_tiles: &[(i64, i64)]) -> Vec<Segment> {
    let mut segments = Vec::new();

    for i in 0..red_tiles.len() {
        let (x1, y1) = red_tiles[i];
        let (x2, y2) = red_tiles[(i + 1) % red_tiles.len()];

        let is_vertical = x1 == x2;
        segments.push(Segment {
            x1: x1.min(x2),
            y1: y1.min(y2),
            x2: x1.max(x2),
            y2: y1.max(y2),
            is_vertical,
        });
    }

    segments
}

// Check if a rectangle is fully inside the polygon (including boundary)
// Uses the fact that the polygon is a simple rectilinear polygon
fn rectangle_inside_polygon(
    rect_min_x: i64,
    rect_min_y: i64,
    rect_max_x: i64,
    rect_max_y: i64,
    segments: &[Segment],
) -> bool {
    // For a rectilinear polygon, we check if all 4 corners are inside
    // and no segment crosses through the interior of the rectangle

    // Check corners using ray casting (count crossings to the right)
    let corners = [
        (rect_min_x, rect_min_y),
        (rect_max_x, rect_min_y),
        (rect_min_x, rect_max_y),
        (rect_max_x, rect_max_y),
    ];

    for &(cx, cy) in &corners {
        if !point_inside_or_on_boundary(cx, cy, segments) {
            return false;
        }
    }

    // Check that no vertical segment cuts through the rectangle horizontally
    // (i.e., a vertical segment with x in (rect_min_x, rect_max_x) and y range overlapping)
    for seg in segments {
        if seg.is_vertical {
            // Vertical segment at x = seg.x1
            if seg.x1 > rect_min_x && seg.x1 < rect_max_x {
                // Check if it overlaps vertically with rectangle
                if seg.y1 < rect_max_y && seg.y2 > rect_min_y {
                    return false;
                }
            }
        } else {
            // Horizontal segment at y = seg.y1
            if seg.y1 > rect_min_y && seg.y1 < rect_max_y {
                // Check if it overlaps horizontally with rectangle
                if seg.x1 < rect_max_x && seg.x2 > rect_min_x {
                    return false;
                }
            }
        }
    }

    true
}

fn point_inside_or_on_boundary(px: i64, py: i64, segments: &[Segment]) -> bool {
    // Check if point is on any segment
    for seg in segments {
        if seg.is_vertical {
            if px == seg.x1 && py >= seg.y1 && py <= seg.y2 {
                return true;
            }
        } else if py == seg.y1 && px >= seg.x1 && px <= seg.x2 {
            return true;
        }
    }

    // Ray casting: count vertical segments to the right of the point
    // that the horizontal ray at py would cross
    let mut crossings = 0;

    for seg in segments {
        if seg.is_vertical {
            // Vertical segment from (seg.x1, seg.y1) to (seg.x1, seg.y2)
            // Ray goes from (px, py) to the right
            if seg.x1 > px && py > seg.y1 && py <= seg.y2 {
                crossings += 1;
            }
        }
    }

    crossings % 2 == 1
}

fn part2(input: &str) -> i64 {
    let red_tiles = parse_tiles(input);
    let segments = build_segments(&red_tiles);

    let mut max_area = 0;

    for i in 0..red_tiles.len() {
        for j in (i + 1)..red_tiles.len() {
            let (x1, y1) = red_tiles[i];
            let (x2, y2) = red_tiles[j];

            let min_x = x1.min(x2);
            let max_x = x1.max(x2);
            let min_y = y1.min(y2);
            let max_y = y1.max(y2);

            if rectangle_inside_polygon(min_x, min_y, max_x, max_y, &segments) {
                let area = (max_x - min_x + 1) * (max_y - min_y + 1);
                max_area = max_area.max(area);
            }
        }
    }

    max_area
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
        assert_eq!(part2(&input), 24);
    }

    #[test]
    fn test_part1() {
        let input = read_input(9);
        assert_eq!(part1(&input), 4744899849);
    }

    #[test]
    fn test_part2() {
        let input = read_input(9);
        assert_eq!(part2(&input), 1540192500);
    }
}
