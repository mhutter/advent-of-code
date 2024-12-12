pub fn day04p1(input: &str) -> u32 {
    let input: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();
    let rows = input.len();
    let cols = input[0].len();

    let mut sum = 0;

    // Directions
    // - right
    // - left
    // - down
    // - up
    // - bottom-right
    // - bottom-left
    // - top-right
    // - top-left

    // Check left-to-right and right-to-left
    for row in input.iter() {
        for col in 0..(cols - 3) {
            match row[col..(col + 4)] {
                ['X', 'M', 'A', 'S'] | ['S', 'A', 'M', 'X'] => sum += 1,
                _ => {}
            }
        }
    }
    // expected: 5

    // Check downward and upward
    for row in 0..(rows - 3) {
        for col in 0..cols {
            let candidate = (row..(row + 4)).map(|r| input[r][col]).collect::<Vec<_>>();
            match candidate.as_slice() {
                ['X', 'M', 'A', 'S'] | ['S', 'A', 'M', 'X'] => sum += 1,
                _ => {}
            }
        }
    }
    // expected: 3

    // check top-left to bottom-right
    for row in 0..(rows - 3) {
        for col in 0..(cols - 3) {
            let candidate = (row..(row + 4))
                .zip(col..(col + 4))
                .map(|(r, c)| input[r][c])
                .collect::<Vec<_>>();
            match candidate.as_slice() {
                ['X', 'M', 'A', 'S'] | ['S', 'A', 'M', 'X'] => sum += 1,
                _ => {}
            }
        }
    }

    // top-right to bottom-left
    for row in 0..(rows - 3) {
        for col in 0..(cols - 3) {
            let candidate = (row..(row + 4))
                .zip((col..(col + 4)).rev())
                .map(|(r, c)| input[r][c])
                .collect::<Vec<_>>();
            match candidate.as_slice() {
                ['X', 'M', 'A', 'S'] | ['S', 'A', 'M', 'X'] => sum += 1,
                _ => {}
            }
        }
    }

    sum
}

pub fn day04p2(input: &str) -> u32 {
    let input: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();
    let rows = input.len();
    let cols = input[0].len();
    let mut sum = 0;

    for row in 1..(rows - 1) {
        for col in 1..(cols - 1) {
            if input[row][col] != 'A' {
                continue;
            }

            let tlbr = (input[row - 1][col - 1], input[row + 1][col + 1]);
            let bltr = (input[row + 1][col - 1], input[row - 1][col + 1]);

            if (tlbr == ('M', 'S') || tlbr == ('S', 'M'))
                && (bltr == ('M', 'S') || bltr == ('S', 'M'))
            {
                sum += 1;
            }
        }
    }

    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_examples() {
        assert_eq!(18, day04p1(INPUT));
    }

    #[test]
    fn part2_examples() {
        assert_eq!(9, day04p2(INPUT));
    }

    const INPUT: &str = "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX";
}
