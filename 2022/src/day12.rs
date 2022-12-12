use hill::*;

mod hill {
    use std::collections::{HashSet, VecDeque};

    pub type Map = Vec<Vec<Node>>;

    /// A square on the map
    #[derive(Debug, Clone, Copy)]
    pub struct Node {
        height: u8,
        distance: Option<i32>,
    }

    impl From<char> for Node {
        fn from(c: char) -> Self {
            Self {
                height: c as u8,
                distance: None,
            }
        }
    }

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
                .map(|line| line.chars().map(Node::from).collect())
                .collect();

            for (y, row) in map.iter_mut().enumerate() {
                for (x, n) in row.iter_mut().enumerate() {
                    match n.height {
                        b'S' => {
                            start = (x, y);
                            n.height = b'a';
                        }
                        b'E' => {
                            dest = (x, y);
                            n.height = b'z';
                            n.distance = Some(0);
                        }
                        _ => {}
                    }
                }
            }

            Self { map, start, dest }
        }
    }

    impl Puzzle {
        pub fn calculate_steps(&mut self) {
            let mut visited = HashSet::with_capacity(self.map.len());
            let mut queue = VecDeque::new();
            queue.push_back(self.dest);

            while let Some(coords) = queue.pop_front() {
                if visited.contains(&coords) {
                    continue;
                }
                let (x, y) = coords;
                let current = self.map[y][x];
                let distance = current.distance.unwrap();

                // inspect neighbors
                let mut neighbors = HashSet::from([(x + 1, y), (x, y + 1)]);
                if x > 0 {
                    neighbors.insert((x - 1, y));
                }
                if y > 0 {
                    neighbors.insert((x, y - 1));
                }

                for (nx, ny) in neighbors {
                    if let Some(mut neighbor) = self.map.get_mut(ny).and_then(|r| r.get_mut(nx)) {
                        if neighbor.height >= current.height - 1 {
                            let neighbor_distance = neighbor.distance.unwrap_or(i32::MAX);
                            if neighbor_distance > distance {
                                neighbor.distance = Some(distance + 1);
                            }
                            queue.push_back((nx, ny));
                        }
                    };
                }

                visited.insert(coords);
            }
        }

        pub fn shortest_s_to_e(&self) -> i32 {
            let (x, y) = self.start;
            self.map[y][x].distance.unwrap()
        }

        pub fn shortest_a_to_e(&self) -> i32 {
            self.map
                .iter()
                .flat_map(|r| {
                    r.iter().filter_map(|n| match n.height {
                        b'a' => n.distance,
                        _ => None,
                    })
                })
                .min()
                .unwrap()
        }
    }
}

pub fn day12p1(input: &str) -> i32 {
    let mut p: Puzzle = input.into();
    p.calculate_steps();
    p.shortest_s_to_e()
}

pub fn day12p2(input: &str) -> i32 {
    let mut p: Puzzle = input.into();
    p.calculate_steps();
    p.shortest_a_to_e()
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
        assert_eq!(29, day12p2(INPUT));
    }

    const INPUT: &str = "Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi";
}
