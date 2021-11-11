pub fn day01p1(input: &[char]) -> i64 {
    let mut floor = 0;

    for c in input {
        match c {
            '(' => floor += 1,
            ')' => floor -= 1,
            _ => panic!("unexpected input: {}", c),
        }
    }

    floor
}

pub fn day01p2(input: &[char]) -> i64 {
    let mut floor = 0;
    let mut pos = 0;

    for c in input {
        pos += 1;
        match c {
            '(' => floor += 1,
            ')' => floor -= 1,
            _ => panic!("unexpected input: {}", c),
        }

        if floor < 0 {
            return pos;
        }
    }

    panic!("did not enter basement")
}

#[cfg(test)]
mod tests {
    use common::generate::chars;

    use super::*;

    #[test]
    fn part1_examples() {
        assert_eq!(0, day01p1(&chars("(())")));
        assert_eq!(0, day01p1(&chars("()()")));
        assert_eq!(3, day01p1(&chars("(((")));
        assert_eq!(3, day01p1(&chars("(()(()(")));
        assert_eq!(3, day01p1(&chars("))(((((")));
        assert_eq!(-1, day01p1(&chars("())")));
        assert_eq!(-1, day01p1(&chars("))(")));
        assert_eq!(-3, day01p1(&chars(")))")));
        assert_eq!(-3, day01p1(&chars(")())())")));
    }

    #[test]
    fn part2_examples() {
        assert_eq!(1, day01p2(&chars(")")));
        assert_eq!(5, day01p2(&chars("()())")));
    }
}
