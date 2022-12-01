use std::{cmp::Reverse, str::Lines};

struct ElvenCalories<'a>(Lines<'a>);

impl<'a> ElvenCalories<'a> {
    fn new(input: &'a str) -> Self {
        Self(input.lines())
    }
}

impl Iterator for ElvenCalories<'_> {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        let mut acc = 0;

        while let Some(line) = self.0.next() {
            if line.is_empty() {
                break;
            }

            acc += line.parse::<u64>().unwrap();
        }

        match acc {
            0 => None,
            _ => Some(acc),
        }
    }
}

fn get_top(input: &str, n: usize) -> u64 {
    let mut calories: Vec<u64> = ElvenCalories::new(input).collect();
    calories.sort_by_key(|&n| Reverse(n));
    calories.get(0..n).unwrap().into_iter().sum()
}

pub fn day01p1(input: &str) -> u64 {
    get_top(input, 1)
}

pub fn day01p2(input: &str) -> u64 {
    get_top(input, 3)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_examples() {
        assert_eq!(24000, day01p1(INPUT));
    }

    #[test]
    fn part2_examples() {
        assert_eq!(45000, day01p2(INPUT));
    }

    const INPUT: &str = "1000
2000
3000

4000

5000
6000

7000
8000
9000

10000";
}
