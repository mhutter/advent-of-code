fn parse(input: &str) -> Vec<u64> {
    let mut elves: Vec<u64> = vec![0];

    for line in input.lines() {
        if line.is_empty() {
            elves.push(0);
            continue;
        }

        let calories = line.parse::<u64>().unwrap();
        *elves.last_mut().unwrap() += calories;
    }

    elves.sort();
    elves.reverse();
    elves
}

pub fn day01p1(input: &str) -> u64 {
    *parse(input).first().unwrap()
}

pub fn day01p2(input: &str) -> u64 {
    parse(input).get(0..=2).unwrap().into_iter().sum()
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
