use std::collections::HashMap;

use bez::*;

mod bez {
    use std::fmt::Write;

    /// The type for the x and y values of the coordinate system
    pub type N = isize;

    #[derive(Debug)]
    pub struct Sensor {
        pub pos: Coords,
        pub beacon: Coords,
        pub range: usize,
    }

    impl Sensor {
        pub fn parse(i: &str) -> Self {
            let (l, r) = i.split_once(": ").expect(": ");
            let pos = Coords::parse(l.expect_prefix("Sensor at "));
            let beacon = Coords::parse(r.expect_prefix("closest beacon is at "));
            let range = pos.distance_to(&beacon);

            Self { pos, beacon, range }
        }
    }

    impl std::fmt::Display for Sensor {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(
                f,
                "Sensor at {}: closest beacon is at {}",
                self.pos, self.beacon
            )
        }
    }

    #[derive(Debug, Hash, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
    pub struct Coords {
        pub x: N,
        pub y: N,
    }

    impl Coords {
        pub fn new(x: N, y: N) -> Self {
            Self { x, y }
        }

        pub fn parse(i: &str) -> Self {
            let (l, r) = i
                .split_once(", ")
                .expect("coordinates to be separated by `, `");

            let x = l
                .strip_prefix("x=")
                .map(|x| x.parse::<N>().expect("a number"))
                .expect("x coordinate");
            let y = r
                .strip_prefix("y=")
                .map(|y| y.parse::<N>().expect("a number"))
                .expect("y coordinate");

            Self::new(x, y)
        }

        pub fn distance_to(&self, other: &Self) -> usize {
            self.x.abs_diff(other.x) + self.y.abs_diff(other.y)
        }
    }

    impl std::fmt::Display for Coords {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "x={}, y={}", self.x, self.y)
        }
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub enum GridField {
        Sensor,
        Beacon,
        Nothing,
    }

    impl std::fmt::Display for GridField {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.write_char((*self).into())
        }
    }

    impl From<GridField> for char {
        fn from(f: GridField) -> Self {
            match f {
                GridField::Sensor => 'S',
                GridField::Beacon => 'B',
                GridField::Nothing => '#',
            }
        }
    }

    trait ExpectPrefixExt {
        fn expect_prefix(self, prefix: &str) -> Self;
    }
    impl ExpectPrefixExt for &str {
        fn expect_prefix(self, prefix: &str) -> Self {
            self.strip_prefix(prefix).expect(prefix)
        }
    }
}

pub fn day15p1(input: &str, y: N) -> usize {
    let mut row: HashMap<isize, GridField> = HashMap::new();

    for s in input.lines().map(Sensor::parse) {
        let distance_to_y = s.pos.y.abs_diff(y);

        if distance_to_y > s.range {
            continue;
        }

        let dx = (s.range - distance_to_y) as isize;

        for x in (s.pos.x - dx)..=(s.pos.x + dx) {
            let curr = Coords::new(x, y);
            let field = if curr == s.pos {
                GridField::Sensor
            } else if curr == s.beacon {
                GridField::Beacon
            } else {
                GridField::Nothing
            };

            let ele = row.entry(x).or_insert(field);
            // if the current field is not `Nothing`, overwrite whatever was in there
            if field != GridField::Nothing {
                *ele = field
            }
        }
    }

    row.into_values()
        .filter(|&f| f != GridField::Beacon)
        .count()
}

pub fn day15p2(_input: &str) -> usize {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_examples() {
        assert_eq!(26, day15p1(INPUT, 10));
    }

    #[test]
    fn part2_examples() {
        assert_eq!(0, day15p2(INPUT));
    }

    const INPUT: &str = "Sensor at x=2, y=18: closest beacon is at x=-2, y=15
Sensor at x=9, y=16: closest beacon is at x=10, y=16
Sensor at x=13, y=2: closest beacon is at x=15, y=3
Sensor at x=12, y=14: closest beacon is at x=10, y=16
Sensor at x=10, y=20: closest beacon is at x=10, y=16
Sensor at x=14, y=17: closest beacon is at x=10, y=16
Sensor at x=8, y=7: closest beacon is at x=2, y=10
Sensor at x=2, y=0: closest beacon is at x=2, y=10
Sensor at x=0, y=11: closest beacon is at x=2, y=10
Sensor at x=20, y=14: closest beacon is at x=25, y=17
Sensor at x=17, y=20: closest beacon is at x=21, y=22
Sensor at x=16, y=7: closest beacon is at x=15, y=3
Sensor at x=14, y=3: closest beacon is at x=15, y=3
Sensor at x=20, y=1: closest beacon is at x=15, y=3";
}
