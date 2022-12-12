use hill::*;

mod hill {
    use std::collections::{HashMap, HashSet, VecDeque};

    pub type Map = Vec<Vec<u8>>;

    #[derive(Debug)]
    pub struct Puzzle {
        map: Map,
        start: (usize, usize),
        dest: (usize, usize),
    }

    impl From<&str> for Puzzle {
        fn from(s: &str) -> Self {
            let mut start = (0, 0);
            let mut dest = (0, 0);

            let mut map: Map = s
                .lines()
                .map(|line| line.chars().map(|c| c as u8).collect())
                .collect();

            for (y, row) in map.iter_mut().enumerate() {
                for (x, c) in row.iter_mut().enumerate() {
                    match c {
                        b'S' => {
                            start = (x, y);
                            *c = b'a';
                        }
                        b'E' => {
                            dest = (x, y);
                            *c = b'z';
                        }
                        _ => {}
                    }
                }
            }

            Self { map, start, dest }
        }
    }

    impl Puzzle {
        pub fn calculate_steps(&self) -> i32 {
            let mut visited = HashSet::with_capacity(self.map.len());
            let mut distances = HashMap::with_capacity(self.map.len());
            distances.insert(self.start, 0);
            let mut queue = VecDeque::new();
            queue.push_back(self.start);

            while let Some(coords) = queue.pop_front() {
                if visited.contains(&coords) {
                    continue;
                }
                let distance = *distances.get(&coords).unwrap();
                let (x, y) = coords;
                let current_height = self.map[y][x];

                // inspect neighbors
                let mut neighbors = HashSet::from([(x + 1, y), (x, y + 1)]);
                if x > 0 {
                    neighbors.insert((x - 1, y));
                }
                if y > 0 {
                    neighbors.insert((x, y - 1));
                }

                neighbors
                    .iter()
                    // select the ones that exist
                    .for_each(|&neighbor_coords| {
                        let (nx, ny) = neighbor_coords;
                        if let Some(neighbor_height) = self.map.get(ny).and_then(|r| r.get(nx)) {
                            if *neighbor_height <= current_height + 1 {
                                let neighbor_distance =
                                    distances.entry(neighbor_coords).or_insert(i32::MAX);
                                if *neighbor_distance > distance {
                                    *neighbor_distance = distance + 1;
                                }
                                queue.push_back(neighbor_coords);
                            }
                        };
                    });

                visited.insert(coords);
            }

            *distances
                .get(&self.dest)
                .expect("get distance value for DEST")
        }
    }
}

pub fn day12p1(input: &str) -> i32 {
    let p: Puzzle = input.into();
    p.calculate_steps()
}

pub fn day12p2(_input: &str) -> usize {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_examples() {
        assert_eq!(31, day12p1(INPUT));
    }

    #[test]
    fn part2_examples() {
        assert_eq!(0, day12p2(INPUT));
    }

    const INPUT: &str = "Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi";
}
