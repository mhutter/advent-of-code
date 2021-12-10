use std::{collections::HashMap, fmt::Display, hash::Hash};

#[derive(Debug, PartialEq, Eq, Hash)]
struct Point {
    pub x: u32,
    pub y: u32,
}

impl Point {
    fn from_to_x(&self, other: &Point) -> (u32, u32) {
        if self.x > other.x {
            return (other.x, self.x);
        }

        (self.x, other.x)
    }

    fn from_to_y(&self, other: &Point) -> (u32, u32) {
        if self.y > other.y {
            return (other.y, self.y);
        }

        (self.y, other.y)
    }

    fn range_x(&self, other: &Point) -> Vec<u32> {
        if self.x > other.x {
            return (other.x..=self.x).rev().collect();
        }

        (self.x..=other.x).collect()
    }

    fn range_y(&self, other: &Point) -> Vec<u32> {
        if self.y > other.y {
            return (other.y..=self.y).rev().collect();
        }

        (self.y..=other.y).collect()
    }
}

impl From<&str> for Point {
    fn from(point: &str) -> Self {
        let mut coords = point.split(',').map(|p| p.parse::<u32>().unwrap());
        Point {
            x: coords.next().unwrap(),
            y: coords.next().unwrap(),
        }
    }
}

#[derive(Debug)]
struct Map {
    data: HashMap<Point, u16>,
}

impl Map {
    fn new() -> Self {
        Self {
            data: HashMap::new(),
        }
    }

    fn increase(&mut self, p: Point) {
        let counter = self.data.entry(p).or_insert(0);
        *counter += 1;
    }

    fn count_fields_with_at_least(&self, n: u16) -> usize {
        self.data.values().filter(|c| c >= &&n).count()
    }
}

impl Display for Map {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let size_x = self.data.keys().map(|p| p.x).max().unwrap() + 1;
        let size_y = self.data.keys().map(|p| p.y).max().unwrap() + 1;

        for y in 0..size_y {
            for x in 0..size_x {
                let key = Point { x, y };
                let p = self.data.get(&key);
                match p {
                    Some(n) => write!(f, "{}", n)?,
                    None => write!(f, ".")?,
                };
            }

            writeln!(f)?
        }

        Ok(())
    }
}

pub fn day05p1(input: &str) -> usize {
    let lines = input.lines().map(|line| {
        let mut points = line.split(" -> ").map(Point::from);
        (points.next().unwrap(), points.next().unwrap())
    });

    let mut map = Map::new();
    for (p0, p1) in lines {
        if p0.x == p1.x {
            let (from, to) = p0.from_to_y(&p1);

            for y in from..=to {
                let p = Point { x: p0.x, y };
                map.increase(p);
            }
        }

        if p0.y == p1.y {
            let (from, to) = p0.from_to_x(&p1);

            for x in from..=to {
                let p = Point { x, y: p0.y };
                map.increase(p);
            }
        }
    }

    map.count_fields_with_at_least(2)
}

pub fn day05p2(input: &str) -> usize {
    let lines = input.lines().map(|line| {
        let mut points = line.split(" -> ").map(Point::from);
        (points.next().unwrap(), points.next().unwrap())
    });

    let mut map = Map::new();

    for (p0, p1) in lines {
        if p0.x == p1.x {
            let (from, to) = p0.from_to_y(&p1);

            for y in from..=to {
                let p = Point { x: p0.x, y };
                map.increase(p);
            }

            continue;
        }

        if p0.y == p1.y {
            let (from, to) = p0.from_to_x(&p1);

            for x in from..=to {
                let p = Point { x, y: p0.y };
                map.increase(p);
            }

            continue;
        }

        let mut range_y = p0.range_y(&p1).into_iter();
        for x in p0.range_x(&p1) {
            let y = range_y.next().unwrap();
            map.increase(Point { x, y });
        }
    }

    map.count_fields_with_at_least(2)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_examples() {
        assert_eq!(5, day05p1(INPUT));
    }

    #[test]
    fn part2_examples() {
        assert_eq!(12, day05p2(INPUT));
    }

    #[test]
    fn map() {
        let mut map = Map::new();
        map.increase(Point { x: 0, y: 0 });
        assert_eq!(Some(&1), map.data.get(&Point { x: 0, y: 0 }));
        assert_eq!(None, map.data.get(&Point { x: 1, y: 0 }));
        assert_eq!(None, map.data.get(&Point { x: 0, y: 1 }));
    }

    const INPUT: &str = "0,9 -> 5,9
8,0 -> 0,8
9,4 -> 3,4
2,2 -> 2,1
7,0 -> 7,4
6,4 -> 2,0
0,9 -> 2,9
3,4 -> 1,4
0,0 -> 8,8
5,5 -> 8,2";
}
