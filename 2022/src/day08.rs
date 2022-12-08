use std::collections::HashMap;

type Map = HashMap<(usize, usize), char>;
type Tree = char;

fn parse(input: &str) -> (Map, usize, usize) {
    // parse data into a HashMap where the coordinates are the key, and the height is the value
    let map: HashMap<(usize, usize), Tree> = input
        .lines()
        .enumerate()
        .flat_map(|(y, row)| row.chars().enumerate().map(move |(x, c)| ((x, y), c)))
        .collect();

    // determine dimensions of map
    let xmax = *map.keys().map(|(x, _)| x).max().expect("imax");
    let ymax = *map.keys().map(|(_, y)| y).max().expect("jmax");

    (map, xmax, ymax)
}

pub fn day08p1(input: &str) -> usize {
    let (map, xmax, ymax) = parse(input);

    map.iter()
        // Filter for trees that are visible
        .filter(|&(&(x, y), &tree)| {
            // Trees on the edge
            x == 0
                || y == 0
                || x == xmax
                || y == ymax
                // Trees visible from one side
                || (0..x).all(|x| *map.get(&(x, y)).unwrap() < tree)
                || ((x + 1)..=xmax).all(|x| *map.get(&(x, y)).unwrap() < tree)
                || (0..y).all(|y|*map.get(&(x, y)).unwrap() < tree)
                || ((y + 1)..=ymax).all(|y| *map.get(&(x, y)).unwrap() < tree)
        })
        // count what is left
        .count()
}

trait ViewingDistanceExt {
    fn viewing_distance_x(self, y: usize, map: &Map, tree: &Tree) -> usize;
    fn viewing_distance_y(self, x: usize, map: &Map, tree: &Tree) -> usize;
}

impl<T> ViewingDistanceExt for T
where
    T: Iterator<Item = usize> + Clone,
{
    fn viewing_distance_x(self, y: usize, map: &Map, tree: &Tree) -> usize {
        self.clone()
            .enumerate()
            .find_map(|(n, x)| map.get(&(x, y)).unwrap().ge(tree).then_some(n + 1))
            .unwrap_or(self.count())
    }

    fn viewing_distance_y(self, x: usize, map: &Map, tree: &Tree) -> usize {
        self.clone()
            .enumerate()
            .find_map(|(n, y)| map.get(&(x, y)).unwrap().ge(tree).then_some(n + 1))
            .unwrap_or(self.count())
    }
}

pub fn day08p2(input: &str) -> usize {
    let (map, xmax, ymax) = parse(input);

    map.iter()
        // Calculate the "scenic score" for each tree
        .map(|(&(x, y), tree)| {
            if x == 0 || x == xmax || y == 0 || y == ymax {
                return 0;
            }

            (0..x).rev().viewing_distance_x(y, &map, tree)
                * ((x + 1)..=xmax).viewing_distance_x(y, &map, tree)
                * (0..y).rev().viewing_distance_y(x, &map, tree)
                * ((y + 1)..=ymax).viewing_distance_y(x, &map, tree)
        })
        .max()
        .expect("max score")
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
        assert_eq!(8, day08p2(INPUT));
    }

    const INPUT: &str = "30373
25512
65332
33549
35390";
}
