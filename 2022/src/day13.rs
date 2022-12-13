use std::ops::Not;

use distress::*;

mod distress {
    use std::cmp::Ordering;

    use nom::{
        branch::alt,
        bytes::complete::tag,
        character,
        character::complete::{char, newline},
        combinator::map,
        multi::separated_list0,
        sequence::{delimited, separated_pair},
        IResult,
    };

    type List = Vec<Packet>;

    #[derive(Debug)]
    pub struct Pair(Packet, Packet);

    #[derive(Debug, PartialEq, Eq, Clone)]
    pub enum Packet {
        Num(u64),
        List(List),
    }

    impl Packet {
        pub fn parse(i: &str) -> IResult<&str, Self> {
            alt((
                map(character::complete::u64, Self::Num),
                map(Self::parse_list, Self::List),
            ))(i)
        }

        fn parse_list(i: &str) -> IResult<&str, List> {
            delimited(
                char('['),
                separated_list0(tag(","), Packet::parse),
                char(']'),
            )(i)
        }
    }

    impl std::fmt::Display for Packet {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                Self::Num(n) => write!(f, "{n}"),
                Self::List(l) => {
                    write!(
                        f,
                        "[{}]",
                        l.iter()
                            .map(|e| e.to_string())
                            .collect::<Vec<_>>()
                            .join(", ")
                    )
                }
            }
        }
    }

    impl Ord for Packet {
        fn cmp(&self, other: &Self) -> std::cmp::Ordering {
            match (self, other) {
                (Self::Num(lhs), Self::Num(rhs)) => lhs.cmp(rhs),
                (Self::List(lhs), Self::List(rhs)) => {
                    // if any of the item in L is < R

                    for (l, r) in lhs.iter().zip(rhs) {
                        if l == r {
                            continue;
                        }
                        return l.cmp(r);
                    }

                    // all are equal
                    if lhs.len() <= rhs.len() {
                        Ordering::Less
                    } else {
                        Ordering::Greater
                    }
                }
                (Self::List(_), Self::Num(n)) => self.cmp(&Self::List(vec![Self::Num(*n)])),
                (Self::Num(n), Self::List(_)) => Self::List(vec![Self::Num(*n)]).cmp(other),
            }
        }
    }
    impl PartialOrd for Packet {
        fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
            Some(self.cmp(other))
        }
    }

    impl Pair {
        pub fn parse(i: &str) -> IResult<&str, Self> {
            map(
                separated_pair(Packet::parse, newline, Packet::parse),
                Self::from,
            )(i)
        }

        pub fn is_sorted(&self) -> bool {
            let Self(left, right) = self;
            left < right
        }
    }

    impl From<(Packet, Packet)> for Pair {
        fn from(t: (Packet, Packet)) -> Self {
            Self(t.0, t.1)
        }
    }
}

pub fn day13p1(input: &str) -> usize {
    input
        .split("\n\n")
        .map(|i| Pair::parse(i).unwrap().1)
        .enumerate()
        .filter_map(|(i, pair)| pair.is_sorted().then_some(i + 1))
        .sum()
}

pub fn day13p2(input: &str) -> usize {
    let divider_packets = vec![
        Packet::parse("[[2]]").unwrap().1,
        Packet::parse("[[6]]").unwrap().1,
    ];
    let mut data = input
        .lines()
        .filter_map(|line| {
            line.is_empty()
                .not()
                .then(|| Packet::parse(line).unwrap().1)
        })
        .collect::<Vec<_>>();

    data.append(&mut divider_packets.clone());
    data.sort();

    divider_packets
        .iter()
        .map(|p| data.iter().position(|i| i == p).unwrap() + 1)
        .product()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_examples() {
        assert_eq!(13, day13p1(INPUT));
    }

    #[test]
    fn part2_examples() {
        assert_eq!(140, day13p2(INPUT));
    }

    const INPUT: &str = "[1,1,3,1,1]
[1,1,5,1,1]

[[1],[2,3,4]]
[[1],4]

[9]
[[8,7,6]]

[[4,4],4,4]
[[4,4],4,4,4]

[7,7,7,7]
[7,7,7]

[]
[3]

[[[]]]
[[]]

[1,[2,[3,[4,[5,6,7]]]],8,9]
[1,[2,[3,[4,[5,6,0]]]],8,9]";
}
