use std::collections::HashSet;

pub fn day06p1(input: &str) -> usize {
    let data = input.chars().collect::<Vec<_>>();
    let l = data.len();

    (0..l - 4)
        .find(|&i| data[i..i + 4].iter().collect::<HashSet<_>>().len().eq(&4))
        .expect("find a marker position")
        + 4
}

pub fn day06p2(_input: &str) -> usize {
    0
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
        assert_eq!(0, day06p2(INPUT[0]));
        assert_eq!(0, day06p2(INPUT[1]));
        assert_eq!(0, day06p2(INPUT[2]));
        assert_eq!(0, day06p2(INPUT[3]));
        assert_eq!(0, day06p2(INPUT[4]));
    }

    const INPUT: &[&str] = &[
        "mjqjpqmgbljsphdztnvjfqwrcgsmlb",
        "bvwbjplbgvbhsrlpgdmjqwftvncz",
        "nppdvjthqldpwncqszvftbrmjlhg",
        "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg",
        "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw",
    ];
}
