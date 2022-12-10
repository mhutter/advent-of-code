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

            self.lines.next().map(|line| {
                let (dir, amt) = line.split_once(' ').expect("whitespace");
                let dir = Direction::from(dir);
                self.current_dir = Some(dir);
                self.remaining = amt.parse::<usize>().expect("number") - 1;
                dir
            })
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

    #[derive(Default, Clone, Copy)]
    pub struct End {
        x: isize,
        y: isize,
    }
    impl std::fmt::Debug for End {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "({}/{})", self.x, self.y)
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

        /// Move `self` towards `head`, respecting the rules outlined in
        /// https://adventofcode.com/2022/day/9
        pub fn move_towards(&mut self, head: &Self) {
            let dx = head.x - self.x;
            let dy = head.y - self.y;

            // If we're within 1 of `other`, no movement is required
            if dx.abs() <= 1 && dy.abs() <= 1 {
                return;
            }

            // movement is limited, so our goal is to reduce dx and dy by 1 each
            self.x += dx.signum();
            self.y += dy.signum();
        }
    }
}

pub fn day09p1(input: &str) -> usize {
    let (mut head, mut tail) = (End::default(), End::default());

    Movements::from(input)
        .map(|direction| {
            debug!("BEFORE at {head:?} {tail:?}");
            head.move_step(direction);
            tail.move_towards(&head);
            debug!("AFTER  at {head:?} {tail:?}");

            tail.coords()
        })
        .collect::<HashSet<_>>()
        .len()
}

pub fn day09p2(input: &str) -> usize {
    let mut knots = [End::default(); 10];

    Movements::from(input)
        .map(|direction| {
            debug!("BEFORE {knots:?}");

            // move the head
            knots[0].move_step(direction);

            // move rest
            for i in 1..(knots.len()) {
                knots[i].move_towards(&knots[i - 1].clone());
            }

            debug!("AFTER  {knots:?}");
            knots.last().unwrap().coords()
        })
        .collect::<HashSet<_>>()
        .len()
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
        let _ = env_logger::builder().is_test(true).try_init();

        assert_eq!(1, day09p2(INPUT), "small input");
        assert_eq!(36, day09p2(INPUT2), "big input");
    }

    const INPUT: &str = "R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2";

    const INPUT2: &str = "R 5
U 8
L 8
D 3
R 17
D 10
L 25
U 20";
}
