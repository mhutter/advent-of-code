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
    let mut increases = 0;
    let mut previous = input[0] + input[1] + input[2];

    for i in 0..input.len() {
        if i < 3 {
            continue;
        }

        let sum = input[i] + input[i - 1] + input[i - 2];
        if sum > previous {
            increases += 1;
        }
        previous = sum;
    }

    increases
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &[i32] = &[199, 200, 208, 210, 200, 207, 240, 269, 260, 263];

    #[test]
    fn part1_examples() {
        assert_eq!(7, day01p1(INPUT));
    }

    #[test]
    fn part2_examples() {
        assert_eq!(5, day01p2(INPUT));
    }
}
