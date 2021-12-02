pub fn day02p1(input: &str) -> i32 {
    let mut pos = 0;
    let mut depth = 0;

    for direction in parse_input(input) {
        match direction {
            Direction::Forward(amount) => pos += amount,
            Direction::Down(amount) => depth += amount,
            Direction::Up(amount) => depth -= amount,
        }
    }

    pos * depth
}

pub fn day02p2(input: &str) -> i32 {
    input.len() as i32
}

#[derive(Debug, PartialEq)]
enum Direction {
    Forward(i32),
    Down(i32),
    Up(i32),
}

impl From<&str> for Direction {
    fn from(line: &str) -> Self {
        let mut parts = line.split(' ');
        let direction = parts.next().unwrap();
        let amount: i32 = parts.next().unwrap().parse().unwrap();

        match direction {
            "forward" => Self::Forward(amount),
            "down" => Self::Down(amount),
            "up" => Self::Up(amount),
            _ => panic!("invalid direction: {}", direction),
        }
    }
}

fn parse_input(input: &str) -> Vec<Direction> {
    input.lines().map(|line| line.into()).collect()
}

#[cfg(test)]
mod tests {
    use std::vec;

    use super::Direction::*;
    use super::*;

    const INPUT: &str = "forward 5\ndown 5\nforward 8\nup 3\ndown 8\nforward 2\n";

    #[test]
    fn part1_examples() {
        assert_eq!(150, day02p1(INPUT));
    }

    #[test]
    fn part2_examples() {
        assert_eq!(0, day02p2(INPUT));
    }

    #[test]
    fn test_parse_input() {
        let expected = vec![Forward(5), Down(5), Forward(8), Up(3), Down(8), Forward(2)];
        assert_eq!(expected, parse_input(INPUT));
    }
}
