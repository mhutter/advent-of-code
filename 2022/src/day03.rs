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

            let prio = prio_for(c);

            println!("prio: {prio}");

            sum += prio;

            // if the item appears multiple times we still only count it once, so abort here
            break;
        }
    }

    sum
}

/// calculate prio for the given character
///
/// In ASCII, uppercase letters have decimal values 65-90, and lowercase 97-122.
fn prio_for(c: &u8) -> usize {
    let p = match c {
        65..=90 => c - (65 - 26),
        97..=122 => c - 97,
        _ => panic!("Invalid byte: {c:?}"),
    };
    p as usize + 1
}

pub fn day03p2(input: &[u8]) -> usize {
    let mut i = 0;
    let lines: Vec<&[u8]> = input.split(|&c| c == b'\n').collect();
    let mut sum = 0;

    loop {
        if i >= lines.len() {
            break;
        }

        for c in lines[i] {
            if lines[i + 1].contains(c) && lines[i + 2].contains(c) {
                sum += prio_for(c);
                break;
            }
        }

        i += 3;
    }

    sum
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
        assert_eq!(70, day03p2(INPUT));
    }

    const INPUT: &[u8] = b"vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw";
}
