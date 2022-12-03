use std::collections::HashSet;

pub fn day03p1(input: &[u8]) -> usize {
    let mut sum = 0;

    for line in input.split(|&c| c == b'\n') {
        // note that str.len() is the length in `bytes`, not `char`s, but since the problem
        // statement only includes ASCII characters that's okay
        let n = line.len() / 2;

        let mut left = HashSet::new();

        // record the left half
        for c in &line[..n] {
            left.insert(c);
        }

        // search the match
        for c in &line[n..] {
            if !left.contains(&c) {
                continue;
            }

            println!("match: {}", *c as char);

            // match, calculate priority & add to sum!
            // In ASCII, uppercase letters have decimal values 65-90, and lowercase 97-122.
            let prio = match c {
                65..=90 => c - (65 - 26),
                97..=122 => c - 97,
                _ => panic!("Invalid byte: {c:?}"),
            };

            println!("prio: {prio}");

            sum += prio as usize + 1;

            // if the item appears multiple times we still only count it once, so abort here
            break;
        }
    }

    sum
}

pub fn day03p2(_input: &[u8]) -> usize {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_examples() {
        assert_eq!(157, day03p1(INPUT));
    }

    #[test]
    fn part2_examples() {
        assert_eq!(0, day03p2(INPUT));
    }

    const INPUT: &[u8] = b"vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw";
}
