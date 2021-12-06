pub fn day06p1(mut input: Vec<u8>) -> usize {
    for _ in 0..80 {
        let mut new_fish = Vec::new();

        for fish in input.iter_mut() {
            if *fish == 0 {
                *fish = 6;
                new_fish.push(8);
            } else {
                *fish -= 1;
            }
        }

        input.append(&mut new_fish);
    }

    input.len()
}

pub fn day06p2(input: Vec<u8>) -> usize {
    input.len()
}

#[cfg(test)]
mod tests {
    use common::generate;

    use super::*;
    const INPUT: &str = "3,4,3,1,2";

    #[test]
    fn part1_examples() {
        let input = generate::int_list(INPUT);
        assert_eq!(5934, day06p1(input));
    }

    #[test]
    fn part2_examples() {
        let input = generate::int_list(INPUT);
        assert_eq!(0, day06p2(input));
    }
}
