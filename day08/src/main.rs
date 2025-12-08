use common::{read_input, run_day};

#[derive(Debug, Clone, Copy)]
struct Point {
    x: i64,
    y: i64,
    z: i64,
}

impl Point {
    fn distance_squared(&self, other: &Point) -> i64 {
        let dx = self.x - other.x;
        let dy = self.y - other.y;
        let dz = self.z - other.z;
        dx * dx + dy * dy + dz * dz
    }
}

struct UnionFind {
    parent: Vec<usize>,
    rank: Vec<usize>,
    size: Vec<usize>,
}

impl UnionFind {
    fn new(n: usize) -> Self {
        UnionFind {
            parent: (0..n).collect(),
            rank: vec![0; n],
            size: vec![1; n],
        }
    }

    fn find(&mut self, x: usize) -> usize {
        if self.parent[x] != x {
            self.parent[x] = self.find(self.parent[x]);
        }
        self.parent[x]
    }

    fn union(&mut self, x: usize, y: usize) -> bool {
        let root_x = self.find(x);
        let root_y = self.find(y);

        if root_x == root_y {
            return false; // Already in same circuit
        }

        // Union by rank
        if self.rank[root_x] < self.rank[root_y] {
            self.parent[root_x] = root_y;
            self.size[root_y] += self.size[root_x];
        } else if self.rank[root_x] > self.rank[root_y] {
            self.parent[root_y] = root_x;
            self.size[root_x] += self.size[root_y];
        } else {
            self.parent[root_y] = root_x;
            self.size[root_x] += self.size[root_y];
            self.rank[root_x] += 1;
        }

        true
    }

    fn get_circuit_sizes(&mut self) -> Vec<usize> {
        let n = self.parent.len();
        let mut sizes = Vec::new();
        for i in 0..n {
            if self.find(i) == i {
                sizes.push(self.size[i]);
            }
        }
        sizes.sort_by(|a, b| b.cmp(a)); // Sort descending
        sizes
    }
}

fn parse_input(input: &str) -> Vec<Point> {
    input
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| {
            let parts: Vec<i64> = line.split(',').map(|s| s.parse().unwrap()).collect();
            Point {
                x: parts[0],
                y: parts[1],
                z: parts[2],
            }
        })
        .collect()
}

fn solve(input: &str, num_connections: usize) -> i64 {
    let points = parse_input(input);
    let n = points.len();

    // Generate all pairs with their distances
    let mut pairs: Vec<(i64, usize, usize)> = Vec::new();
    for i in 0..n {
        for j in (i + 1)..n {
            let dist = points[i].distance_squared(&points[j]);
            pairs.push((dist, i, j));
        }
    }

    // Sort pairs by distance
    pairs.sort_by_key(|&(dist, _, _)| dist);

    // Use Union-Find to connect pairs
    let mut uf = UnionFind::new(n);
    let mut connections_made = 0;

    for (_, i, j) in pairs {
        if connections_made >= num_connections {
            break;
        }
        // Try to connect (even if already connected, count it)
        uf.union(i, j);
        connections_made += 1;
    }

    // Get circuit sizes and multiply top 3
    let sizes = uf.get_circuit_sizes();
    let mut result: i64 = 1;
    for i in 0..3.min(sizes.len()) {
        result *= sizes[i] as i64;
    }

    result
}

fn part1(input: &str) -> i64 {
    solve(input, 1000)
}

fn part2(input: &str) -> i64 {
    // TODO: Implement part 2
    let _lines: Vec<&str> = input.lines().collect();
    0
}

fn main() {
    let input = read_input(8);
    let result = run_day(8, &input, part1, part2);
    result.print(8);
}

#[cfg(test)]
mod tests {
    use super::*;
    use common::read_example;

    #[test]
    fn test_part1_example() {
        let input = read_example(8);
        // After 10 connections: 5 * 4 * 2 = 40
        assert_eq!(solve(&input, 10), 40);
    }

    #[test]
    fn test_part2_example() {
        let input = read_example(8);
        assert_eq!(part2(&input), 0); // TODO: Update expected value
    }

    #[test]
    fn test_part1() {
        let input = read_input(8);
        assert_eq!(part1(&input), 0); // TODO: Update expected value after solving
    }

    #[test]
    fn test_part2() {
        let input = read_input(8);
        assert_eq!(part2(&input), 0); // TODO: Update expected value after solving
    }
}
