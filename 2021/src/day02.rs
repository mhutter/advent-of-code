pub fn day02p1(input: &str) -> i32 {
    let mut pos = 0;
    let mut depth = 0;

    for line in input.lines() {
        let mut parts = line.split(' ');
        let direction = parts.next().unwrap();
        let amount: i32 = parts.next().unwrap().parse().unwrap();

        match direction {
            "forward" => pos += amount,
            "down" => depth += amount,
            "up" => depth -= amount,
            _ => panic!("unexpected direction {}", direction),
        }
    }

    pos * depth
}

pub fn day02p2(input: &str) -> i32 {
    input.len() as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "forward 5\ndown 5\nforward 8\nup 3\ndown 8\nforward 2\n";

    #[test]
    fn part1_examples() {
        assert_eq!(150, day02p1(INPUT));
    }

    #[test]
    fn part2_examples() {
        assert_eq!(0, day02p2(INPUT));
    }
}
