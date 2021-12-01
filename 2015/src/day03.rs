use std::collections::HashSet;

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
struct Santa(i32, i32);

impl Santa {
    fn new() -> Self {
        Self(0, 0)
    }

    fn relocate(&mut self, direction: &char) {
        match direction {
            '^' => self.1 += 1,
            'v' => self.1 -= 1,
            '>' => self.0 += 1,
            '<' => self.0 -= 1,
            _ => panic!("unexpected input: {}", direction),
        }
    }
}

pub fn day03p1(input: &[char]) -> usize {
    let mut santa = Santa::new();
    let mut visited = HashSet::new();
    visited.insert(santa);

    for direction in input {
        santa.relocate(direction);
        visited.insert(santa);
    }

    visited.len()
}

pub fn day03p2(input: &[char]) -> usize {
    let mut santa = [Santa::new(), Santa::new()];
    let mut visited = HashSet::new();
    visited.insert(santa[0]);

    for (i, direction) in input.iter().enumerate() {
        santa[i % 2].relocate(direction);
        visited.insert(santa[i % 2]);
    }

    visited.len()
}

#[cfg(test)]
mod tests {
    use common::generate::chars;

    use super::*;

    #[test]
    fn part1_examples() {
        assert_eq!(2, day03p1(&chars(">")));
        assert_eq!(4, day03p1(&chars("^>v<")));
        assert_eq!(2, day03p1(&chars("^v^v^v^v^v")));
    }

    #[test]
    fn part2_examples() {
        assert_eq!(3, day03p2(&chars("^v")));
        assert_eq!(3, day03p2(&chars("^>v<")));
        assert_eq!(11, day03p2(&chars("^v^v^v^v^v")));
    }
}
