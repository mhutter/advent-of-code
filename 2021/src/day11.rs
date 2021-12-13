struct Octopus {
    energy_level: u8,
    flashed_this_round: bool,
}

impl From<char> for Octopus {
    fn from(c: char) -> Self {
        Self {
            energy_level: c.to_digit(10).unwrap() as u8,
            flashed_this_round: false,
        }
    }
}

struct Grid {
    map: Vec<Vec<Octopus>>,
    size_x: usize,
    size_y: usize,
}

impl From<&str> for Grid {
    fn from(input: &str) -> Self {
        let map: Vec<Vec<Octopus>> = input
            .lines()
            .map(|line| line.chars().map(Octopus::from).collect())
            .collect();
        let size_y = map.len();
        let size_x = map.get(0).unwrap().len();

        Self {
            map,
            size_x,
            size_y,
        }
    }
}

impl Grid {
    /// Reset `flashed_this_round` and increase `energy_level` by 1 for all
    fn new_round(&mut self) {
        for l in self.map.iter_mut() {
            for o in l.iter_mut() {
                if o.energy_level > 9 {
                    // must have been flashed in the previous round
                    o.energy_level = 0;
                }
                o.flashed_this_round = false;
                o.energy_level += 1;
            }
        }
    }

    fn flash_all(&mut self) -> Option<usize> {
        let mut flashed = 0;
        let mut to_increase: Vec<(usize, usize)> = Vec::new();

        for (y, line) in self.map.iter_mut().enumerate() {
            for (x, octopus) in line.iter_mut().enumerate() {
                if octopus.energy_level > 9 && !octopus.flashed_this_round {
                    flashed += 1;
                    octopus.flashed_this_round = true;
                    // top left
                    if y > 0 && x > 0 {
                        to_increase.push((x - 1, y - 1));
                    }
                    // top
                    if y > 0 {
                        to_increase.push((x, y - 1));
                    }
                    // top right
                    if y > 0 && x + 1 < self.size_x {
                        to_increase.push((x + 1, y - 1));
                    }
                    // left
                    if x > 0 {
                        to_increase.push((x - 1, y));
                    }
                    // right
                    if x + 1 < self.size_x {
                        to_increase.push((x + 1, y));
                    }
                    // bottom left
                    if y + 1 < self.size_y && x > 0 {
                        to_increase.push((x - 1, y + 1));
                    }
                    // bottom
                    if y + 1 < self.size_y {
                        to_increase.push((x, y + 1));
                    }
                    // bottom right
                    if y + 1 < self.size_y && x + 1 < self.size_x {
                        to_increase.push((x + 1, y + 1));
                    }
                }
            }
        }

        // TODO: go through to_increase;
        for (x, y) in to_increase {
            let o = self.map.get_mut(y).unwrap().get_mut(x).unwrap();
            o.energy_level += 1;
        }

        if flashed > 0 {
            Some(flashed)
        } else {
            None
        }
    }
}

pub fn day11p1(input: &str) -> usize {
    let mut grid = Grid::from(input);
    let mut num_flashes = 0;

    for _ in 0..100 {
        grid.new_round();
        while let Some(flashed) = grid.flash_all() {
            num_flashes += flashed;
        }
    }

    num_flashes
}

pub fn day11p2(input: &str) -> usize {
    let mut grid = Grid::from(input);
    let mut round = 0;

    loop {
        round += 1;
        grid.new_round();
        let mut flashes_this_round = 0;
        while let Some(flashed) = grid.flash_all() {
            flashes_this_round += flashed;
        }
        if flashes_this_round == grid.size_x * grid.size_y {
            return round;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_examples() {
        assert_eq!(1656, day11p1(INPUT));
    }

    #[test]
    fn part2_examples() {
        assert_eq!(195, day11p2(INPUT));
    }

    const INPUT: &str = "5483143223
2745854711
5264556173
6141336146
6357385478
4167524645
2176841721
6882881134
4846848554
5283751526";
}
