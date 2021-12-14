use std::collections::HashMap;

fn apply(polymer: &str, rules: &HashMap<&str, char>) -> String {
    let chars: Vec<char> = polymer.chars().collect();
    let mut out: Vec<char> = Vec::new();

    for i in 0..(chars.len() - 1) {
        let curr = chars[i];
        let next = chars[i + 1];

        out.push(curr);
        if let Some(insert) = rules.get(&[curr, next].iter().collect::<String>().as_str()) {
            out.push(*insert);
        }
    }
    out.push(*chars.last().unwrap());

    out.into_iter().collect()
}

fn most_and_leat_common_elements(polymer: &str) -> (usize, usize) {
    let mut count: HashMap<char, usize> = HashMap::new();
    for c in polymer.chars() {
        let e = count.entry(c).or_insert(0);
        *e += 1;
    }

    (
        *count.values().max().unwrap(),
        *count.values().min().unwrap(),
    )
}

pub fn day14p1(input: &str) -> usize {
    let mut lines = input.lines();
    let mut polymer = lines.next().unwrap().to_string();
    lines.next();

    let rules: HashMap<&str, char> = lines
        .map(|line| {
            let parts: Vec<&str> = line.split(" -> ").collect();
            (parts[0], parts[1].chars().next().unwrap())
        })
        .collect();

    for _ in 0..10 {
        polymer = apply(&polymer, &rules);
    }

    let (most, least) = most_and_leat_common_elements(&polymer);
    most - least
}

pub fn day14p2(_input: &str) -> usize {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_examples() {
        assert_eq!(1588, day14p1(INPUT));
    }

    #[test]
    fn part2_examples() {
        assert_eq!(0, day14p2(INPUT));
    }

    const INPUT: &str = "NNCB

CH -> B
HH -> N
CB -> H
NH -> C
HB -> C
HC -> B
HN -> C
NN -> C
BH -> H
NC -> B
NB -> B
BN -> B
BB -> N
BC -> B
CC -> N
CN -> C
";
}
