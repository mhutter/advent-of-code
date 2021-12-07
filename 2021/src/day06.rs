fn day06(input: Vec<u8>, days: usize) -> u64 {
    let mut fish = [0; 10];

    for i in input {
        fish[i as usize] += 1;
    }

    for _ in 0..days {
        for i in 0..=9 {
            if i == 0 {
                // spawn new fish
                fish[9] = fish[i];
                // reset fish
                fish[7] += fish[i];

                continue;
            }

            // advance all other fish
            fish[i - 1] = fish[i]
        }
        fish[9] = 0;
    }

    fish.into_iter().sum()
}

pub fn day06p1(input: Vec<u8>) -> u64 {
    day06(input, 80)
}

pub fn day06p2(input: Vec<u8>) -> u64 {
    day06(input, 256)
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
        assert_eq!(26984457539, day06p2(input));
    }
}
