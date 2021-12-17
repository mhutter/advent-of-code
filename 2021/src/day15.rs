#[derive(Debug)]
struct Cave {
    max: usize,
    map: Vec<Vec<u32>>,
}

impl From<&str> for Cave {
    fn from(input: &str) -> Self {
        let map: Vec<Vec<u32>> = input
            .lines()
            .map(|line| line.chars().map(|c| c.to_digit(10).unwrap()).collect())
            .collect();
        let max = map.len() - 1;

        Self { max, map }
    }
}

impl Cave {
    fn visit(&self, x: usize, y: usize, acc: u32) -> u32 {
        let max = self.max;
        let acc = acc + self.map[y][x];

        if x == max && y == max {
            // reached the end!
            return acc;
        }

        let mut min = u32::MAX;
        if x < max {
            min = self.visit(x + 1, y, acc);
        }
        if y < max {
            let v = self.visit(x, y + 1, acc);
            if v < min {
                min = v;
            }
        }

        min
    }
}

pub fn day15p1(input: &str) -> u32 {
    let cave = Cave::from(input);
    cave.visit(0, 0, 0) - cave.map[0][0]
}

pub fn day15p2(_input: &str) -> usize {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_examples() {
        assert_eq!(40, day15p1(INPUT));
    }

    #[test]
    fn part2_examples() {
        assert_eq!(0, day15p2(INPUT));
    }

    const INPUT: &str = "1163751742
1381373672
2136511328
3694931569
7463417111
1319128137
1359912421
3125421639
1293138521
2311944581
";
}
