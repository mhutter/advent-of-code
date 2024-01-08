use std::collections::HashMap;

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

pub fn day01p2(input: &str) -> u32 {
    const WORDS: [&str; 9] = [
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];

    // build lookup table
    let words = WORDS
        .iter()
        .enumerate()
        .map(|(i, &w)| (w, (i + 1).to_string()))
        .collect::<HashMap<_, _>>();

    // convienience
    #[inline]
    fn starts_with_word(i: &str) -> Option<&'static str> {
        WORDS.iter().find(|&w| i.starts_with(w)).copied()
    }

    // replace all words by digits
    let mut input = String::from(input);
    // longest word: 5 letters
    for i in 0..(input.len() - 5) {
        if let Some(word) = starts_with_word(&input[i..]) {
            input.replace_range(i..(i + 1), words.get(word).unwrap());
        }
    }

    day01p1(&input)
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
        const INPUT: &str = "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen";
        assert_eq!(281, day01p2(INPUT));
    }
}
