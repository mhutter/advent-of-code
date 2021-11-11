use std::collections::HashSet;

pub fn day03p1(input: &[char]) -> i64 {
    let (mut x, mut y) = (0, 0);
    let mut visited = HashSet::new();
    visited.insert((x, y));

    for c in input {
        match c {
            '^' => y += 1,
            'v' => y -= 1,
            '>' => x += 1,
            '<' => x -= 1,
            _ => panic!("unexpected input: {}", c),
        }
        visited.insert((x, y));
    }

    visited.len() as i64
}

pub fn day03p2(input: &[char]) -> i64 {
    input.len() as i64
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
        assert_eq!(0, day03p2(&[]))
    }
}
