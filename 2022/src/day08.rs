use std::collections::HashMap;

pub fn day08p1(input: &str) -> usize {
    // parse data into a HashMap where the coordinates are the key, and the height is the value
    let map: HashMap<(usize, usize), char> = input
        .lines()
        .enumerate()
        .flat_map(|(j, row)| row.chars().enumerate().map(move |(i, c)| ((i, j), c)))
        .collect();

    // determine dimensions of map
    let imax = *map.keys().map(|(i, _)| i).max().expect("imax");
    let jmax = *map.keys().map(|(_, j)| j).max().expect("jmax");

    map.iter()
        // Filter for trees that are visible
        .filter(|&(&(i, j), &tree)| {
            // Trees on the edge
            i == 0
                || j == 0
                || i == imax
                || j == jmax
                // Trees visible from one side
                || (0..i).all(|i| *map.get(&(i, j)).unwrap() < tree)
                || ((i + 1)..=imax).all(|i| *map.get(&(i, j)).unwrap() < tree)
                || (0..j).all(|j| *map.get(&(i, j)).unwrap() < tree)
                || ((j + 1)..=jmax).all(|j| *map.get(&(i, j)).unwrap() < tree)
        })
        // count what is left
        .count()
}

pub fn day08p2(_input: &str) -> usize {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_examples() {
        assert_eq!(21, day08p1(INPUT));
    }

    #[test]
    fn zeroes() {
        assert_eq!(8, day08p1("111\n101\n111"));
        assert_eq!(9, day08p1("000\n010\n000"));
    }

    #[test]
    fn one_side() {
        assert_eq!(9, day08p1("949\n959\n999"), "top");
        assert_eq!(9, day08p1("999\n459\n999"), "left");
        assert_eq!(9, day08p1("999\n954\n999"), "right");
        assert_eq!(9, day08p1("999\n959\n949"), "bottom");
    }

    #[test]
    fn part2_examples() {
        assert_eq!(0, day08p2(INPUT));
    }

    const INPUT: &str = "30373
25512
65332
33549
35390";
}
