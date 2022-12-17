use flow::*;

/// Specific types for Day 17
///
/// Today's problem acts on a grid system, and we're using a cartesian coordinate system for this.
pub mod flow {
    use std::collections::HashSet;

    use common::GridCoord;

    /// The type used for the coordinate system
    pub type N = usize;

    /// Rock shapes falling from the ceiling as per problem statement
    pub const SHAPES: [Shape; 5] = [
        Shape(&[(0, 0), (1, 0), (2, 0), (3, 0)]),
        Shape(&[(1, 0), (0, 1), (1, 1), (2, 1), (1, 2)]),
        Shape(&[(0, 0), (1, 0), (2, 0), (2, 1), (2, 2)]),
        Shape(&[(0, 0), (0, 1), (0, 2), (0, 3)]),
        Shape(&[(0, 0), (0, 1), (1, 0), (1, 1)]),
    ];

    /// GridCoord which represents movement alogn the X axis
    pub const MOVE_X: GridCoord<N> = GridCoord::new(1, 0);
    /// GridCoord which represents movement alogn the Y axis
    pub const MOVE_Y: GridCoord<N> = GridCoord::new(0, 1);

    /// A shape of rocks
    ///
    /// The list of tuples describe where space is filled with rock. Its "position" is its lower
    /// left corner.
    #[derive(Clone, Copy)]
    pub struct Shape(&'static [(N, N)]);

    impl Shape {
        pub fn positions(&self, pos: GridCoord<N>) -> Vec<GridCoord<N>> {
            self.0
                .iter()
                .map(|b| pos + GridCoord::new(b.0, b.1))
                .collect()
        }

        pub fn width(&self) -> N {
            self.0.iter().map(|b| b.0).max().unwrap() + 1
        }
        pub fn height(&self) -> N {
            self.0.iter().map(|b| b.1).max().unwrap() + 1
        }
    }

    impl std::fmt::Display for Shape {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            let xmax = self.0.iter().map(|p| p.0).max().unwrap();
            let ymax = self.0.iter().map(|p| p.1).max().unwrap();
            for y in (0..=ymax).rev() {
                let line = (0..=xmax)
                    .map(|x| if self.0.contains(&(x, y)) { '#' } else { ' ' })
                    .collect::<String>();

                writeln!(f, "{line}")?;
            }
            Ok(())
        }
    }

    pub enum Direction {
        Left,
        Right,
    }

    impl From<char> for Direction {
        fn from(c: char) -> Self {
            match c {
                '<' => Self::Left,
                '>' => Self::Right,
                _ => panic!("Unexpected direction: {c}"),
            }
        }
    }

    pub trait DirectionIter: Iterator<Item = Direction> {}
    impl<T: Iterator<Item = Direction>> DirectionIter for T {}
    pub trait ShapeIter: Iterator<Item = Shape> {}
    impl<T: Iterator<Item = Shape>> ShapeIter for T {}

    /// A chamber of the given width
    ///
    /// x == 0 is the left wall, x == width+1 is the right wall
    /// y == 0 is the floor
    pub struct Chamber<D: DirectionIter, S: ShapeIter> {
        /// Chamber width
        width: N,

        /// Height of the highest rock so far
        highest_rock: N,

        /// Where the rocks are
        occupied: HashSet<GridCoord<N>>,

        /// the jet pattern
        jet_pattern: D,
        shapes: S,
    }

    impl<D: DirectionIter, S: ShapeIter> std::fmt::Display for Chamber<D, S> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            for y in (1..=self.highest_rock).rev() {
                let line = (1..=self.width)
                    .map(|x| match self.occupied.contains(&GridCoord::new(x, y)) {
                        true => '#',
                        false => '.',
                    })
                    .collect::<String>();
                writeln!(f, "|{line}|")?;
            }

            let line = (0..self.width).map(|_| '-').collect::<String>();
            writeln!(f, "+{line}+")
        }
    }

    impl<D: DirectionIter, S: ShapeIter> Chamber<D, S> {
        pub fn new(width: N, jet_pattern: D, shapes: S) -> Self {
            Self {
                width,
                highest_rock: 0,
                occupied: HashSet::new(),
                jet_pattern,
                shapes,
            }
        }

        /// test whether the shape at the given position would overlap with any rocks already
        /// placed, or if the shape would be out of bounds of the cavern.
        fn there_is_overlap(&self, pos: GridCoord<N>, shape: Shape) -> bool {
            pos.y < 1  // hit the floor
                || pos.x < 1  // hit the left wall
                || (pos.x+shape.width()) > (self.width+1)  // hit the right wall
                || shape  // hit any existing rocks
                    .positions(pos)
                    .iter()
                    .any(|p| self.occupied.contains(p))
        }

        /// Add the given shape at the given position to the 'occupied' list
        fn add(&mut self, pos: GridCoord<N>, shape: Shape) {
            for p in shape.positions(pos) {
                self.occupied.insert(p);
            }
        }

        /// place a rock according to the rules of the puzzle
        pub fn place_rock(&mut self) {
            let shape = self.shapes.next().unwrap();
            let mut pos = GridCoord::new(3, self.highest_rock + 4);

            loop {
                // move left or right
                let new_pos = match self.jet_pattern.next().unwrap() {
                    Direction::Left => pos - MOVE_X,
                    Direction::Right => pos + MOVE_X,
                };
                if !self.there_is_overlap(new_pos, shape) {
                    pos = new_pos;
                }

                // move down
                let new_pos = pos - MOVE_Y;
                if self.there_is_overlap(new_pos, shape) {
                    break;
                }
                pos = new_pos;
            }

            // add SHAPE to OCCUPIED
            self.add(pos, shape);
            // update cavern height
            self.highest_rock = self.highest_rock.max(pos.y + shape.height() - 1)
        }

        pub fn rock_tower_height(&self) -> usize {
            self.highest_rock
        }
    }
}

pub fn day17p1(input: &'static str) -> usize {
    let jet_pattern = input.trim().chars().map(Direction::from).cycle();
    let shapes = SHAPES.into_iter().cycle();
    let mut chamber = Chamber::new(7, jet_pattern, shapes);

    // for i in 1..=10 {
    //     chamber.place_rock();
    //     println!("Chamber after rock {i}\n{chamber}");
    // }
    for _ in 0..2022 {
        chamber.place_rock();
    }

    chamber.rock_tower_height()
}

pub fn day17p2(_input: &str) -> usize {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_examples() {
        assert_eq!(3068, day17p1(INPUT));
    }

    #[test]
    fn part2_examples() {
        assert_eq!(0, day17p2(INPUT));
    }

    const INPUT: &str = ">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>";
}
