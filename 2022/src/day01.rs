use std::{cmp::Reverse, str::Lines};

/// An iterator that yields the sum of calories for each elf.
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

        // Let's assume that if there were no calories added, no more lines were left.
        match acc {
            0 => None,
            _ => Some(acc),
        }
    }
}

/// Calculate the sum of the top `n` Elfs
fn get_top(input: &str, n: usize) -> u64 {
    // collect list of calorie totals
    let mut calories: Vec<u64> = ElvenCalories::new(input).collect();

    // sort from biggest to smallest
    calories.sort_by_key(|&n| Reverse(n));

    // get the sum of the top `n` elements
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
