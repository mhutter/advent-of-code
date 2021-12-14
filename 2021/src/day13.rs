use std::{collections::HashSet, fmt::Debug};

enum Fold {
    X(usize),
    Y(usize),
}

impl Debug for Fold {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::X(n) => write!(f, "Fold::X({})", n),
            Self::Y(n) => write!(f, "Fold::Y({})", n),
        }
    }
}

impl From<&str> for Fold {
    fn from(line: &str) -> Self {
        // fold along y=7
        // fold along x=5
        let parts: Vec<&str> = line
            .split_ascii_whitespace()
            .last()
            .unwrap()
            .split('=')
            .collect();
        match parts[0] {
            "x" => Self::X(parts[1].parse().unwrap()),
            "y" => Self::Y(parts[1].parse().unwrap()),
            _ => panic!("Invalid instuction: {}", &line),
        }
    }
}

struct Sheet {
    dots: HashSet<(usize, usize)>,
}

impl Debug for Sheet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let max_x = self.dots.iter().map(|d| d.0).max().unwrap();
        let max_y = self.dots.iter().map(|d| d.1).max().unwrap();

        writeln!(f, "Sheet {{")?;

        for y in 0..=max_y {
            let line: String = (0..=max_x)
                .map(|x| {
                    if self.dots.contains(&(x, y)) {
                        '#'
                    } else {
                        '.'
                    }
                })
                .collect();

            writeln!(f, "{}", &line)?;
        }

        writeln!(f, "}}")
    }
}

impl Sheet {
    fn new() -> Self {
        Self {
            dots: HashSet::new(),
        }
    }

    fn add_dot(&mut self, line: &str) {
        let parts: Vec<&str> = line.split(',').collect();
        self.dots
            .insert((parts[0].parse().unwrap(), parts[1].parse().unwrap()));
    }

    fn apply(&mut self, instruction: Fold) {
        let to_add = match instruction {
            Fold::X(n) => self.fold_left(n),
            Fold::Y(n) => self.fold_up(n),
        };

        for p in to_add {
            self.dots.insert(p);
        }
    }

    fn fold_up(&mut self, n: usize) -> Vec<(usize, usize)> {
        let mut to_remove: Vec<(usize, usize)> = Vec::new();
        let mut to_add: Vec<(usize, usize)> = Vec::new();

        self.dots.iter().for_each(|(x, y)| {
            if y > &n {
                to_remove.push((*x, *y));
            }
        });

        for (x, y) in &to_remove {
            to_add.push((*x, (n - (*y - n))));
        }

        for p in to_remove {
            self.dots.remove(&p);
        }

        to_add
    }

    fn fold_left(&mut self, n: usize) -> Vec<(usize, usize)> {
        let mut to_remove: Vec<(usize, usize)> = Vec::new();
        let mut to_add: Vec<(usize, usize)> = Vec::new();

        self.dots.iter().for_each(|(x, y)| {
            if x > &n {
                to_remove.push((*x, *y));
            }
        });

        for (x, y) in &to_remove {
            to_add.push(((n - (*x - n)), *y));
        }

        for p in to_remove {
            self.dots.remove(&p);
        }

        to_add
    }

    fn num_dots(&self) -> usize {
        self.dots.len()
    }
}

pub fn day13p1(input: &str) -> usize {
    let mut lines = input.lines();
    let mut sheet = Sheet::new();

    for line in lines.by_ref() {
        if line.is_empty() {
            break;
        }
        sheet.add_dot(line);
    }

    let instruction: Fold = lines.next().unwrap().into();
    sheet.apply(instruction);
    sheet.num_dots()
}

pub fn day13p2(_input: &str) -> usize {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_examples() {
        assert_eq!(17, day13p1(INPUT));
    }

    #[test]
    fn part2_examples() {
        assert_eq!(0, day13p2(INPUT));
    }

    const INPUT: &str = "6,10
0,14
9,10
0,3
10,4
4,11
6,0
6,12
4,1
0,13
10,12
3,4
3,0
8,4
1,10
2,14
8,10
9,0

fold along y=7
fold along x=5";
}
