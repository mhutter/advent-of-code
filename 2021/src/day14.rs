use std::collections::HashMap;

#[derive(Debug)]
struct Polymer {
    pairs: HashMap<(char, char), usize>,
}

impl Polymer {
    fn new(input: &str) -> Self {
        let polymer_line: &[char] = &input.chars().collect::<Vec<_>>();
        let mut pairs: HashMap<(char, char), usize> = HashMap::new();
        for w in polymer_line.windows(2) {
            let e = pairs.entry((w[0], w[1])).or_insert(0);
            *e += 1;
        }

        Self { pairs }
    }

    fn apply(&mut self, rules: &HashMap<(char, char), char>) {
        let mut pairs: HashMap<(char, char), usize> = HashMap::new();

        for (&(l, r), count) in self.pairs.iter() {
            if let Some(&insert) = rules.get(&(l, r)) {
                let el = pairs.entry((l, insert)).or_insert(0);
                *el += count;
                let er = pairs.entry((insert, r)).or_insert(0);
                *er += count;
            } else {
                let e = pairs.entry((l, r)).or_insert(0);
                *e += count;
            }
        }
        self.pairs = pairs;
    }

    fn element_counts(&self) -> HashMap<char, usize> {
        let mut out = HashMap::new();

        for (&(l, r), count) in self.pairs.iter() {
            let el = out.entry(l).or_insert(0);
            *el += count;
            let er = out.entry(r).or_insert(0);
            *er += count;
        }

        // to count the elements in a polymer by knowing only the pairs:
        // - count all elements in each par
        // - add 1 to the first and last element in the chain (the only ones that will have an odd count)
        // - divide the number by 2
        for (_, count) in out.iter_mut() {
            if *count & 1 == 1 {
                // count is odd
                *count += 1;
            }

            *count /= 2;
        }

        out
    }

    fn most_and_leat_common_elements(&self) -> (usize, usize) {
        let counts = self.element_counts();

        (
            *counts.values().max().unwrap(),
            *counts.values().min().unwrap(),
        )
    }
}

fn day14(input: &str, n: usize) -> usize {
    let mut lines = input.lines();

    let mut polymer = Polymer::new(lines.next().unwrap());
    assert!(lines.next().unwrap().is_empty());

    let rules: HashMap<(char, char), char> = lines
        .map(|line| {
            let parts: Vec<&str> = line.split(" -> ").collect();
            let key = parts[0].chars().collect::<Vec<_>>();
            ((key[0], key[1]), parts[1].chars().next().unwrap())
        })
        .collect();

    for _ in 0..n {
        polymer.apply(&rules);
    }

    let (most, least) = polymer.most_and_leat_common_elements();
    most - least
}
pub fn day14p1(input: &str) -> usize {
    day14(input, 10)
}

pub fn day14p2(input: &str) -> usize {
    day14(input, 40)
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
        assert_eq!(2188189693529, day14p2(INPUT));
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
