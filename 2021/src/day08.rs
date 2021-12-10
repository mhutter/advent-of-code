use std::collections::HashMap;

const DEFAULT_WIRING: [&str; 10] = [
    "abcefg", "cf", "acdeg", "acdfg", "bcdf", "abdfg", "abdefg", "acf", "abcdefg", "abcdfg",
];

pub fn day08p1(input: &str) -> usize {
    input
        .lines()
        .map(|line| {
            line.split(" | ")
                .nth(1)
                .unwrap()
                .split(' ')
                .collect::<Vec<&str>>()
        })
        .flatten()
        .filter(|e| matches!(e.len(), 2 | 4 | 3 | 7))
        .count()
}

pub fn day08p2(input: &str) -> u32 {
    let digit_for_score = calc_default_score_map();

    input
        .lines()
        .map(|line| {
            let io: Vec<&str> = line.split(" | ").collect();
            let input: Vec<&str> = io[0].split(' ').collect();
            let output = io[1].split(' ');
            let sps = score_per_segment(&input);

            output
                .map(|signal| {
                    digit_for_score
                        .get(
                            &signal
                                .chars()
                                .map(|segment| sps.get(&segment).unwrap())
                                .sum(),
                        )
                        .unwrap()
                })
                .fold(0, |acc: u32, e| acc * 10 + (*e as u32))
        })
        .sum()
}

fn score_per_segment(segments: &[&str]) -> HashMap<char, i32> {
    let mut scores = HashMap::from([
        ('a', 0),
        ('b', 0),
        ('c', 0),
        ('d', 0),
        ('e', 0),
        ('f', 0),
        ('g', 0),
    ]);
    for &segments in segments {
        for segment in segments.chars() {
            let e = scores.get_mut(&segment).unwrap();
            *e += 1;
        }
    }

    scores
}

fn calc_default_score_map() -> HashMap<i32, u8> {
    let default_score = score_per_segment(&DEFAULT_WIRING);
    (0..=9)
        .map(|digit| {
            (
                DEFAULT_WIRING[digit]
                    .chars()
                    .map(|s| default_score.get(&s).unwrap())
                    .sum(),
                digit as u8,
            )
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_examples() {
        assert_eq!(26, day08p1(INPUT));
    }

    #[test]
    fn part2_examples() {
        assert_eq!(61229, day08p2(INPUT));
    }

    const INPUT: &str =
        "be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe
edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc
fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg
fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb
aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea
fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb
dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe
bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef
egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb
gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce
";
}
