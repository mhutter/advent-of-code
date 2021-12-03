pub fn day03p1(input: &str) -> u32 {
    let mut pos = Vec::new();
    let mut n = 0;

    for line in input.lines() {
        n += 1;
        for (i, c) in line.char_indices() {
            let v = match c {
                '0' => 0,
                '1' => 1,
                _ => panic!("Invalid character: {}", &c),
            };

            if let Some(e) = pos.get_mut(i) {
                *e += v;
            } else {
                pos.push(v);
            }
        }
    }

    let mut gamma: u32 = 0;
    let mut epsilon: u32 = 0;

    for p in pos {
        gamma = gamma << 1;
        epsilon = epsilon << 1;
        if p > (n / 2) {
            gamma += 1;
        } else {
            epsilon += 1;
        }
    }

    gamma * epsilon
}

pub fn day03p2(input: &str) -> i32 {
    input.len() as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str =
        "00100\n11110\n10110\n10111\n10101\n01111\n00111\n11100\n10000\n11001\n00010\n01010";

    #[test]
    fn part1_examples() {
        assert_eq!(198, day03p1(INPUT));
    }

    #[test]
    fn part2_examples() {
        assert_eq!(0, day03p2(INPUT));
    }
}
