use std::str::FromStr;

struct Report(Vec<u8>);

impl FromStr for Report {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let data = s
            .split_whitespace()
            .map(|w| w.parse::<u8>().expect("valid u8"))
            .collect();
        Ok(Self(data))
    }
}

impl Report {
    fn is_safe(&self) -> bool {
        let dir = if self.0[0] < self.0[1] {
            Dir::Inc
        } else {
            Dir::Dec
        };
        for i in 0..(self.0.len() - 1) {
            let (l, r) = (self.0[i], self.0[i + 1]);

            if !(1..=3).contains(&l.abs_diff(r))
                || (dir == Dir::Inc && l > r)
                || (dir == Dir::Dec && l < r)
            {
                return false;
            }
        }
        true
    }
}

#[derive(PartialEq, Eq)]
enum Dir {
    Inc,
    Dec,
}

pub fn day02p1(input: &str) -> usize {
    let reports: Vec<_> = input
        .lines()
        .map(|line| Report::from_str(line).unwrap())
        .collect();

    reports.into_iter().filter(|r| r.is_safe()).count()
}

pub fn day02p2(_input: &str) -> u32 {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_examples() {
        assert_eq!(2, day02p1(INPUT));
    }

    #[test]
    fn part2_examples() {
        assert_eq!(0, day02p2(INPUT));
    }

    const INPUT: &str = "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9";
}
