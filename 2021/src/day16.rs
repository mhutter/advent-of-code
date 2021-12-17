pub fn day16p1(_input: &str) -> usize {
    0
}

pub fn day16p2(_input: &str) -> usize {
    0
}

pub fn decode_hex(input: &str) -> Vec<u8> {
    let ascii = input.bytes();
    let mut out = Vec::with_capacity(input.len() / 2);

    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_examples() {
        assert_eq!(0, day16p1(INPUT));
    }

    #[test]
    fn part2_examples() {
        assert_eq!(0, day16p2(INPUT));
    }

    #[test]
    fn test_decode_hex() {
        assert_eq!(vec![] as Vec<u8>, decode_hex(""));
    }

    const INPUT: &str = "";
}
