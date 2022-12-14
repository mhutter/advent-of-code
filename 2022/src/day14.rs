use regolith::*;

mod regolith {
    use std::collections::HashMap;

    const DOWN: Coord = Coord { x: 0, y: 1 };
    const DOWN_LEFT: Coord = Coord { x: -1, y: 1 };
    const DOWN_RIGHT: Coord = Coord { x: 1, y: 1 };

    pub type CoordAxis = isize;

    pub struct Grid {
        map: HashMap<Coord, Material>,
        sand_source: Coord,

        min_x: CoordAxis,
        max_x: CoordAxis,
        max_y: CoordAxis,
    }

    impl Grid {
        pub fn place_sand(&mut self) -> bool {
            let mut sand = self.sand_source;

            while self.move_sand(&mut sand) {
                if sand.y >= self.max_y {
                    return false;
                }
            }

            self.map.insert(sand, Material::Sand);
            true
        }

        fn move_sand(&self, sand: &mut Coord) -> bool {
            [DOWN, DOWN_LEFT, DOWN_RIGHT].into_iter().any(|dir| {
                if self.map.get(&(*sand + dir)).is_none() {
                    *sand += dir;
                    true
                } else {
                    false
                }
            })
        }
    }

    impl std::fmt::Debug for Grid {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            writeln!(
                f,
                "Grid<min_x={} max_x={} max_y={}>",
                self.min_x, self.max_x, self.max_y,
            )?;

            for y in 0..=self.max_y {
                let mut line = String::with_capacity((self.max_x - self.min_x + 1) as usize);

                for x in self.min_x..=self.max_x {
                    let c = Coord::from((x, y));

                    if c == self.sand_source {
                        line.push('+');
                        continue;
                    }

                    line.push(match self.map.get(&c) {
                        Some(Material::Rock) => '#',
                        Some(Material::Sand) => 'o',
                        None => '.',
                    });
                }

                writeln!(f, "{line}")?;
            }
            Ok(())
        }
    }

    impl From<&str> for Grid {
        fn from(s: &str) -> Self {
            let mut min_x = 500;
            let mut max_x = 500;
            let mut max_y = 0;
            let mut map = HashMap::new();

            for line in s.lines() {
                let coords = line.split(" -> ").map(Coord::from).collect::<Vec<_>>();

                for w in coords.windows(2) {
                    for c in w[0].straight_path_to(w[1]) {
                        // record grid bounds
                        min_x = min_x.min(c.x);
                        max_x = max_x.max(c.x);
                        max_y = max_y.max(c.y);

                        map.insert(c, Material::Rock);
                    }
                }
            }

            Self {
                map,
                sand_source: "500,0".into(),
                min_x,
                max_x,
                max_y,
            }
        }
    }

    #[derive(Debug)]
    pub enum Material {
        Rock,
        Sand,
    }

    /// A coordinate on a 2d grid
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub struct Coord {
        x: CoordAxis,
        y: CoordAxis,
    }

    impl Coord {
        pub fn straight_path_to(self, other: Self) -> StraightPath {
            if self.x != other.x && self.y != other.y {
                panic!("X or Y axis must be identical, got {self:?} and {other:?}");
            }

            StraightPath {
                next: Some(self),
                dst: other,
            }
        }
    }

    impl std::ops::Add for Coord {
        type Output = Self;

        fn add(self, rhs: Self) -> Self::Output {
            Self {
                x: self.x + rhs.x,
                y: self.y + rhs.y,
            }
        }
    }
    impl std::ops::AddAssign for Coord {
        fn add_assign(&mut self, rhs: Self) {
            self.x += rhs.x;
            self.y += rhs.y;
        }
    }

    impl From<&str> for Coord {
        fn from(s: &str) -> Self {
            let (x, y) = s.split_once(',').expect("coords with format `X,Y`");
            Self {
                x: x.parse().unwrap(),
                y: y.parse().unwrap(),
            }
        }
    }

    impl From<(CoordAxis, CoordAxis)> for Coord {
        fn from((x, y): (CoordAxis, CoordAxis)) -> Self {
            Self { x, y }
        }
    }

    /// An iterator for all the fields between two coordinates that share either their X or Y axis
    #[derive(Debug)]
    pub struct StraightPath {
        next: Option<Coord>,
        dst: Coord,
    }

    impl Iterator for StraightPath {
        type Item = Coord;

        fn next(&mut self) -> Option<Self::Item> {
            if let Some(mut next) = self.next.take() {
                let curr = next;

                if curr != self.dst {
                    // move 1 step towards `dst`
                    next.x += (self.dst.x - next.x).signum();
                    next.y += (self.dst.y - next.y).signum();

                    self.next = Some(next);
                }

                Some(curr)
            } else {
                None
            }
        }
    }
}

pub fn day14p1(input: &str) -> usize {
    let mut grid = Grid::from(input);

    let mut n = 0;
    while grid.place_sand() {
        n += 1;
    }

    println!("Final layout after {n} steps: {grid:?}");

    n
}

pub fn day14p2(_input: &str) -> usize {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_examples() {
        assert_eq!(24, day14p1(INPUT));
    }

    #[test]
    fn part2_examples() {
        assert_eq!(0, day14p2(INPUT));
    }

    const INPUT: &str = "498,4 -> 498,6 -> 496,6
503,4 -> 502,4 -> 502,9 -> 494,9";
}
