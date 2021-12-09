pub fn day09p1(input: &str) -> u32 {
    let lines: Vec<Vec<u32>> = input
        .lines()
        .map(|l| l.chars().map(|c| c.to_digit(10).unwrap()).collect())
        .collect();
    let mut sum = 0;

    let num_lines = lines.len();
    let num_rows = lines[0].len();

    for (l, line) in lines.iter().enumerate() {
        for (r, val) in line.iter().enumerate() {
            if (l > 0 && lines[l - 1][r] <= *val)
                || (l < (num_lines - 1) && lines[l + 1][r] < *val)
                || (r > 0 && lines[l][r - 1] <= *val)
                || (r < (num_rows - 1) && lines[l][r + 1] < *val)
            {
                continue;
            }

            sum += val + 1;
        }
    }

    sum
}

pub fn day09p2(input: &str) -> i32 {
    input.len() as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_examples() {
        assert_eq!(15, day09p1(INPUT));
    }

    #[test]
    fn part2_examples() {
        assert_eq!(0, day09p2(INPUT));
    }

    const INPUT: &str = "2199943210
3987894921
9856789892
8767896789
9899965678";
}
