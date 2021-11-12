pub fn day12p1(input: &[i32]) -> i32 {
    input.len() as i32
}

pub fn day12p2(input: &[i32]) -> i32 {
    input.len() as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_examples() {
        assert_eq!(0, day12p1(&[]))
    }

    #[test]
    fn part2_examples() {
        assert_eq!(0, day12p2(&[]))
    }
}
