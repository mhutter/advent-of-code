pub fn day01p1(input: &str) -> u32 {
    input
        .lines()
        .map(|line| {
            let digits = line
                .chars()
                .filter(char::is_ascii_digit)
                .map(|c| c as u8 - b'0')
                .collect::<Vec<_>>();

            (digits.first().unwrap() * 10 + digits.last().unwrap()) as u32
        })
        .sum()
}

pub fn day01p2(_input: &str) -> u32 {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_examples() {
        const INPUT: &str = "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet";
        assert_eq!(142, day01p1(INPUT));
    }

    #[test]
    fn part2_examples() {
        const INPUT: &str = "";
        assert_eq!(0, day01p2(INPUT));
    }
}
