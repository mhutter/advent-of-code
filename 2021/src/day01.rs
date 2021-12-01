pub fn day01p1(input: &[i32]) -> i32 {
    let mut increases = 0;
    let mut iter = input.iter();
    let mut previous = iter.next().unwrap();

    for next in iter {
        if next > previous {
            increases += 1;
        }
        previous = next;
    }

    increases
}

pub fn day01p2(input: &[i32]) -> i32 {
    input.len() as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_examples() {
        assert_eq!(
            7,
            day01p1(&[199, 200, 208, 210, 200, 207, 240, 269, 260, 263])
        );
    }

    #[test]
    fn part2_examples() {
        assert_eq!(0, day01p2(&[]));
    }
}
