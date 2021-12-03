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

pub fn day03p2(input: &str) -> u32 {
    let (values, size) = generate(input);
    let oxygen_generator = get_rating(values.clone(), size, false);
    let co2_scrubber = get_rating(values.clone(), size, true);

    oxygen_generator * co2_scrubber
}

fn get_rating(mut values: Vec<u32>, size: usize, invert: bool) -> u32 {
    let flip = if invert { 1 } else { 0 };
    for pos in (0..size).rev() {
        let mask = 1 << pos;
        let filter = (most_common_in_pos(values.clone(), pos) ^ flip) << pos;

        values = values
            .into_iter()
            .filter(|v| (v & mask) == filter)
            .collect();

        if values.len() == 1 {
            return *values.first().unwrap();
        }
    }

    panic!("did not find a value!")
}

/// Convert a string containing a list of binary numbers to a list of u32.
///
/// Returns the list itself as well as the size (number of binary digits) in the first entry.
///
/// Example:
///
/// ```
/// # use advent_of_code_2021::day03::generate;
/// assert_eq!((vec![0b00100, 0b11110], 5), generate("00100\n11110\n"))
/// ```
pub fn generate(input: &str) -> (Vec<u32>, usize) {
    let mut lines = input.lines().peekable();
    // Bit size of values
    let size = lines.peek().unwrap().len();
    // All values as u32
    let values = lines.map(|l| u32::from_str_radix(l, 2).unwrap()).collect();

    (values, size)
}

/// Find the most common value in the bit position (zero-indexed from the RIGHT) given.
///
/// Example:
///
/// ```
/// # use advent_of_code_2021::day03::most_common_in_pos;
/// let values = vec![0b00100, 0b11110, 0b10110, 0b10111, 0b10101, 0b01111, 0b00111, 0b11100, 0b10000, 0b11001, 0b00010, 0b01010];
/// assert_eq!(1, most_common_in_pos(values.clone(), 4));
/// assert_eq!(0, most_common_in_pos(values.clone(), 3));
/// assert_eq!(1, most_common_in_pos(values.clone(), 2));
/// assert_eq!(1, most_common_in_pos(values.clone(), 1));
/// assert_eq!(0, most_common_in_pos(values.clone(), 0));
/// ```
pub fn most_common_in_pos(values: Vec<u32>, pos: usize) -> u32 {
    let digits: Vec<u32> = values.into_iter().map(|v| (v >> pos) & 1).collect();
    let zeros = digits.iter().filter(|d| **d == 0).count();
    let ones = digits.iter().filter(|d| **d == 1).count();

    if zeros > ones {
        0
    } else {
        1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str =
        "00100\n11110\n10110\n10111\n10101\n01111\n00111\n11100\n10000\n11001\n00010\n01010\n";

    #[test]
    fn part1_examples() {
        assert_eq!(198, day03p1(INPUT));
    }

    #[test]
    fn part2_examples() {
        assert_eq!(230, day03p2(INPUT));
    }
}
