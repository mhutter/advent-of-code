use std::collections::HashSet;

#[aoc_generator(day1)]
fn generator(input: &str) -> Vec<i32> {
    input.lines().map(|l| l.parse::<i32>().unwrap()).collect()
}

#[aoc(day1, part1)]
pub fn part1(input: &[i32]) -> i32 {
    input.iter().sum()
}

#[aoc(day1, part2)]
pub fn part2(input: &[i32]) -> i32 {
    let mut seen = HashSet::new();
    let mut sum = 0;
    seen.insert(sum);

    loop {
        for i in input {
            sum += i;
            if !seen.insert(sum) {
                return sum;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_examples() {
        assert_eq!(3, part1(&[1, -2, 3, 1]));
        assert_eq!(3, part1(&[1, 1, 1]));
        assert_eq!(0, part1(&[1, 1, -2]));
        assert_eq!(-6, part1(&[-1, -2, -3]));
    }

    #[test]
    fn part2_examples() {
        assert_eq!(2, part2(&[1, -2, 3, 1]));
        assert_eq!(0, part2(&[1, -1]));
        assert_eq!(10, part2(&[3, 3, 4, -2, -4]));
        assert_eq!(5, part2(&[-6, 3, 8, 5, -6]));
        assert_eq!(14, part2(&[7, 7, -2, -7, -4]));
    }
}
