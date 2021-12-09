use std::collections::HashSet;

struct Heightmap {
    rows: usize,
    cols: usize,

    map: Vec<Vec<u8>>,
}

type Position = (usize, usize, u8);

impl From<&str> for Heightmap {
    fn from(input: &str) -> Self {
        let map: Vec<Vec<u8>> = input
            .lines()
            .map(|l| l.chars().map(|c| c.to_digit(10).unwrap() as u8).collect())
            .collect();

        let rows = map.len();
        let cols = map[0].len();

        Self { rows, cols, map }
    }
}

impl Heightmap {
    fn low_points(&self) -> Vec<Position> {
        let mut positions = Vec::new();

        for (y, row) in self.map.iter().enumerate() {
            for (x, val) in row.iter().enumerate() {
                if (y > 0 && self.map[y - 1][x] <= *val)
                    || ((y + 1) < self.rows && self.map[y + 1][x] < *val)
                    || (x > 0 && self.map[y][x - 1] <= *val)
                    || ((x + 1) < self.cols && self.map[y][x + 1] < *val)
                {
                    continue;
                }

                positions.push((x, y, *val));
            }
        }

        positions
    }

    fn value_at(&self, x: usize, y: usize) -> u8 {
        self.map[y][x]
    }
}

#[derive(Debug)]
struct Basin {
    origin: (usize, usize),
    parts: HashSet<(usize, usize)>,
}

impl Basin {
    fn new(x: usize, y: usize) -> Self {
        Self {
            origin: (x, y),
            parts: HashSet::new(),
        }
    }

    fn size(&self) -> usize {
        self.parts.len()
    }

    fn explore(&mut self, map: &Heightmap) {
        self.explore_pos(self.origin.0, self.origin.1, map)
    }

    fn explore_pos(&mut self, x: usize, y: usize, map: &Heightmap) {
        let pos = (x, y);

        if map.value_at(x, y) > 8 || self.parts.contains(&pos) {
            // invalid position, not part of any basin or already visited
            return;
        }
        self.parts.insert(pos);

        if x > 0 {
            self.explore_pos(x - 1, y, map);
        }
        if x + 1 < map.cols {
            self.explore_pos(x + 1, y, map);
        }
        if y > 0 {
            self.explore_pos(x, y - 1, map)
        }
        if y + 1 < map.rows {
            self.explore_pos(x, y + 1, map);
        }
    }
}

pub fn day09p1(input: &str) -> u32 {
    let map = Heightmap::from(input);
    map.low_points()
        .into_iter()
        .map(|(_, _, val)| (val + 1) as u32)
        .sum()
}

pub fn day09p2(input: &str) -> usize {
    let map = Heightmap::from(input);
    let mut basin_sizes: Vec<usize> = map
        .low_points()
        .into_iter()
        .map(|(x, y, _)| {
            let mut basin = Basin::new(x, y);
            basin.explore(&map);
            basin.size()
        })
        .collect();

    basin_sizes.sort();
    basin_sizes.into_iter().rev().take(3).product()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_examples() {
        assert_eq!(15, day09p1(INPUT));
    }

    #[test]
    fn part2_examples() {
        assert_eq!(1134, day09p2(INPUT));
    }

    const INPUT: &str = "2199943210
3987894921
9856789892
8767896789
9899965678";
}
