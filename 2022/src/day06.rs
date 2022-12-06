use std::collections::HashSet;

fn day06(input: &str, window_size: usize) -> usize {
    let data = input.chars().collect::<Vec<_>>();
    let l = data.len();

    (0..l - window_size)
        .find(|&i| {
            data[i..i + window_size]
                .iter()
                .collect::<HashSet<_>>()
                .len()
                .eq(&window_size)
        })
        .expect("find a marker position")
        + window_size
}

pub fn day06p1(input: &str) -> usize {
    day06(input, 4)
}

pub fn day06p2(input: &str) -> usize {
    day06(input, 14)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_examples() {
        assert_eq!(7, day06p1(INPUT[0]));
        assert_eq!(5, day06p1(INPUT[1]));
        assert_eq!(6, day06p1(INPUT[2]));
        assert_eq!(10, day06p1(INPUT[3]));
        assert_eq!(11, day06p1(INPUT[4]));
    }

    #[test]
    fn part2_examples() {
        assert_eq!(19, day06p2(INPUT[0]));
        assert_eq!(23, day06p2(INPUT[1]));
        assert_eq!(23, day06p2(INPUT[2]));
        assert_eq!(29, day06p2(INPUT[3]));
        assert_eq!(26, day06p2(INPUT[4]));
    }

    const INPUT: &[&str] = &[
        "mjqjpqmgbljsphdztnvjfqwrcgsmlb",
        "bvwbjplbgvbhsrlpgdmjqwftvncz",
        "nppdvjthqldpwncqszvftbrmjlhg",
        "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg",
        "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw",
    ];
}
