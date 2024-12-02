use std::collections::HashMap;

pub fn day01p1(input: &str) -> u32 {
    let mut left = Vec::new();
    let mut right = Vec::new();
    for line in input.lines() {
        let (l, r) = line.split_once(' ').unwrap();
        left.push(l.trim().parse::<u32>().unwrap());
        right.push(r.trim().parse::<u32>().unwrap());
    }

    left.sort_unstable();
    right.sort_unstable();

    left.into_iter()
        .zip(right)
        .map(|(l, r)| l.abs_diff(r))
        .sum()
}

pub fn day01p2(input: &str) -> u32 {
    let mut left = Vec::new();
    let mut right: HashMap<u32, u32> = HashMap::new();
    for line in input.lines() {
        let (l, r) = line.split_once(' ').unwrap();
        let l: u32 = l.trim().parse().unwrap();
        let r: u32 = r.trim().parse().unwrap();
        left.push(l);
        *right.entry(r).or_default() += 1;
    }

    left.into_iter()
        .map(|l| l * right.get(&l).copied().unwrap_or_default())
        .sum()
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
        assert_eq!(31, day01p2(INPUT));
    }

    const INPUT: &str = "3   4
4   3
2   5
1   3
3   9
3   3";
}
