pub fn day02p1(input: &[i64]) -> i64 {
    input.len() as i64
}

pub fn day02p2(input: &[i64]) -> i64 {
    input.len() as i64
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_examples() {
        assert_eq!(0, day02p1(&[]))
    }

    #[test]
    fn part2_examples() {
        assert_eq!(0, day02p2(&[]))
    }
}
