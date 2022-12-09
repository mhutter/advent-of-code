use std::collections::HashSet;

use log::debug;
use rope::*;

mod rope {
    use std::str::Lines;

    /// An iterator over a given set of motions
    ///
    /// Since we have to simulate every single step, this iterator will yield every direction the
    /// indicated amount of times, before it will parse the next line. (eg. `R 2` will yield
    /// `Direction::R` twice.)
    pub struct Movements<'a> {
        lines: Lines<'a>,
        current_dir: Option<Direction>,
        remaining: usize,
    }

    impl<'a> From<&'a str> for Movements<'a> {
        fn from(s: &'a str) -> Self {
            Self {
                lines: s.lines(),
                current_dir: None,
                remaining: 0,
            }
        }
    }

    impl<'a> Iterator for Movements<'a> {
        type Item = Direction;

        fn next(&mut self) -> Option<Self::Item> {
            // check if there are still remaining moves
            if self.remaining > 0 {
                self.remaining -= 1;
                return Some(self.current_dir.expect("a direction for remaining moves"));
            }

            if let Some(line) = self.lines.next() {
                let (dir, amt) = line.split_once(' ').expect("whitespace");
                self.current_dir = Some(Direction::from(dir));
                self.remaining = amt.parse::<usize>().expect("number") - 1;
                return self.current_dir;
            }

            None
        }
    }

    #[derive(Debug, Clone, Copy)]
    pub enum Direction {
        D,
        L,
        R,
        U,
    }
    impl From<&str> for Direction {
        fn from(s: &str) -> Self {
            match s {
                "D" => Self::D,
                "L" => Self::L,
                "R" => Self::R,
                "U" => Self::U,
                _ => panic!("invalid direction: {s}"),
            }
        }
    }

    #[derive(Debug, Default)]
    pub struct End {
        x: isize,
        y: isize,
    }
    impl std::fmt::Display for End {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{}/{}", self.x, self.y)
        }
    }
    impl End {
        pub fn coords(&self) -> (isize, isize) {
            (self.x, self.y)
        }

        pub fn move_step(&mut self, direction: Direction) {
            match direction {
                Direction::D => self.y -= 1,
                Direction::L => self.x -= 1,
                Direction::R => self.x += 1,
                Direction::U => self.y += 1,
            };
        }

        /// Move `self` towards `other`, respecting the rules outlined in
        /// https://adventofcode.com/2022/day/9
        pub fn move_towards(&mut self, other: &Self) {
            let dx = other.x - self.x;
            let dy = other.y - self.y;

            // If we're within 1 of `other`, no movement is required
            if dx.abs() <= 1 && dy.abs() <= 1 {
                return;
            }

            //
            match (dx, dy) {
                // movement on 1 axis
                (2, 0) => self.x += 1,
                (0, 2) => self.y += 1,
                (-2, 0) => self.x -= 1,
                (0, -2) => self.y -= 1,
                // if there was a diagonal movement, one of the deltas must be 1 or -1, and the
                // other must be 2 or -2.
                (2, _) => {
                    self.x += 1;
                    self.y = other.y
                }
                (_, 2) => {
                    self.y += 1;
                    self.x = other.x;
                }
                (-2, _) => {
                    self.x -= 1;
                    self.y = other.y;
                }
                (_, -2) => {
                    self.y -= 1;
                    self.x = other.x;
                }
                // other cases do not exist according to problem statement
                _ => panic!("{self} {other}"),
            }
        }
    }
}

pub fn day09p1(input: &str) -> usize {
    let mut visited = HashSet::from([(0, 0)]);
    let (mut head, mut tail) = (End::default(), End::default());

    Movements::from(input).for_each(|direction| {
        debug!("BEFORE at {} {}", &head, &tail);
        head.move_step(direction);
        tail.move_towards(&head);
        visited.insert(tail.coords());
        debug!("AFTER  at {} {}", &head, &tail);
    });

    visited.len()
}

pub fn day09p2(_input: &str) -> usize {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_examples() {
        assert_eq!(13, day09p1(INPUT));
    }

    #[test]
    fn part2_examples() {
        assert_eq!(0, day09p2(INPUT));
    }

    const INPUT: &str = "R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2";
}
