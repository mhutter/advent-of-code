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
    pub fn is_safe(&self) -> bool {
        find_unsafe(&self.0).is_none()
    }

    pub fn is_safe_dampened(&self) -> bool {
        if find_unsafe(&self.0).is_none() {
            return true;
        }

        for i in 0..(self.0.len()) {
            let mut dampened_report = self.0.clone();
            dampened_report.remove(i);
            if find_unsafe(&dampened_report).is_none() {
                return true;
            };
        }
        false
    }
}

fn find_unsafe(report: &[u8]) -> Option<usize> {
    let dir = if report[0] < report[1] {
        Dir::Inc
    } else {
        Dir::Dec
    };
    for i in 0..(report.len() - 1) {
        let (l, r) = (report[i], report[i + 1]);

        if !(1..=3).contains(&l.abs_diff(r))
            || (dir == Dir::Inc && l > r)
            || (dir == Dir::Dec && l < r)
        {
            return Some(i);
        }
    }

    None
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

pub fn day02p2(input: &str) -> usize {
    let reports: Vec<_> = input
        .lines()
        .map(|line| Report::from_str(line).unwrap())
        .collect();

    // since we want to mutate the reports; we can't just `filter` them since this would mean we'd
    // reuse them later in the iterator.
    reports
        .into_iter()
        .filter_map(|r| r.is_safe_dampened().then_some(()))
        .count()
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
        assert_eq!(4, day02p2(INPUT));
    }

    const INPUT: &str = "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9";
}
