fn parse_input(input: &str) -> (Vec<u32>, Vec<u32>) {
    let mut left = Vec::new();
    let mut right = Vec::new();
    for line in input.lines() {
        let (l, r) = line.split_once(' ').unwrap();
        left.push(l.trim().parse().unwrap());
        right.push(r.trim().parse().unwrap());
    }

    left.sort_unstable();
    right.sort_unstable();

    (left, right)
}
pub fn day01p1(input: &str) -> u32 {
    let (left, right) = parse_input(input);

    left.into_iter()
        .zip(right)
        .map(|(l, r)| l.abs_diff(r))
        .sum()
}

pub fn day01p2(_input: &str) -> u32 {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_examples() {
        assert_eq!(11, day01p1(INPUT));
    }

    #[test]
    fn part2_examples() {
        assert_eq!(0, day01p2(INPUT));
    }

    const INPUT: &str = "3   4
4   3
2   5
1   3
3   9
3   3";
}
