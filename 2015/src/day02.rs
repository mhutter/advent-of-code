use std::str::FromStr;

pub fn day02p1(input: &str) -> i32 {
    input.lines().map(paper_for).sum()
}

pub fn day02p2(input: &str) -> i32 {
    input.lines().map(ribbon_for).sum()
}

struct Sizes(i32, i32, i32);

impl FromStr for Sizes {
    type Err = std::num::ParseIntError;

    /// Parse a string of the form "1x2x3" into an instance of 'Sizes'
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split('x');
        let l = parts.next().unwrap().parse()?;
        let w = parts.next().unwrap().parse()?;
        let h = parts.next().unwrap().parse()?;

        Ok(Self(l, w, h))
    }
}

fn paper_for(dimensions: &str) -> i32 {
    let Sizes(l, w, h) = dimensions.parse().unwrap();
    let s = [l * w, l * h, w * h];

    let total: i32 = s.iter().sum();
    let slack = s.iter().min().unwrap();

    slack + 2 * total
}

fn ribbon_for(dimensions: &str) -> i32 {
    let Sizes(l, w, h) = dimensions.parse().unwrap();
    let mut s = [l, w, h];
    s.sort();

    let total = 2 * (s[0] + s[1]);
    let bow: i32 = s.iter().product();

    total + bow
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_examples() {
        assert_eq!(58, paper_for("2x3x4"));
        assert_eq!(43, paper_for("1x1x10"));
        assert_eq!(58 + 43, day02p1("2x3x4\n1x1x10"));
    }

    #[test]
    fn part2_examples() {
        assert_eq!(34, ribbon_for("2x3x4"));
        assert_eq!(14, ribbon_for("1x1x10"));
        assert_eq!(34 + 14, day02p2("2x3x4\n1x1x10"));
    }
}
