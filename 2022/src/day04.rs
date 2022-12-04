use cleanup::*;

mod cleanup {
    /// A pair of section assignments
    pub struct Pair(Assignment, Assignment);

    impl Pair {
        /// Determine whether one of the Assignments fully contains the other
        pub fn has_full_overlap(&self) -> bool {
            let Self(Assignment(lf, lt), Assignment(rf, rt)) = self;

            let lr = lf..=lt;
            let rr = rf..=rt;

            (lr.contains(&rf) && lr.contains(&rt)) || (rr.contains(&lf) && rr.contains(&lt))
        }
    }

    impl From<&str> for Pair {
        fn from(s: &str) -> Self {
            let (l, r) = s.split_once(',').unwrap();

            Self(l.into(), r.into())
        }
    }

    /// A section assignment
    pub struct Assignment(usize, usize);

    impl From<&str> for Assignment {
        fn from(s: &str) -> Self {
            let (l, r) = s.split_once('-').unwrap();

            Self(l.parse().unwrap(), r.parse().unwrap())
        }
    }
}

pub fn day04p1(input: &str) -> usize {
    input
        .lines()
        .map(Pair::from)
        .filter(Pair::has_full_overlap)
        .count()
}

pub fn day04p2(_input: &str) -> usize {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_examples() {
        assert_eq!(2, day04p1(INPUT));
    }

    #[test]
    fn part2_examples() {
        assert_eq!(0, day04p2(INPUT));
    }

    const INPUT: &str = "2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8";
}
