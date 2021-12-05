/// The bingo module contains all functionality related to the bingo game
mod bingo {
    use std::fmt::Debug;

    /// Game represents a bingo session with the numbers being drawn and the boards available.
    pub struct Game {
        pub numbers: Vec<u16>,
        pub boards: Vec<Board>,

        next: usize,
    }

    impl Game {
        pub fn advance(&mut self) {
            let num = self.numbers[self.next];
            for board in &mut self.boards {
                board.mark(num);
            }
            self.next += 1;
        }

        pub fn any_won(&self) -> Option<(u16, Board)> {
            for board in &self.boards {
                if board.won() {
                    return Some((self.numbers[self.next - 1], *board));
                }
            }

            None
        }
    }

    impl From<&str> for Game {
        fn from(input: &str) -> Self {
            let mut lines = input.lines().into_iter();
            let numbers = lines
                .next()
                .unwrap()
                .split(",")
                .map(|e| e.parse().unwrap())
                .collect();

            let mut boards = Vec::new();

            while let Some(line) = lines.next() {
                assert_eq!("", line);
                let mut numbers = [[Field::Unmarked(0); 5]; 5];

                for i in 0..5 {
                    let line: [Field; 5] = lines
                        .next()
                        .unwrap()
                        .split_ascii_whitespace()
                        .map(|f| Field::Unmarked(f.parse().unwrap()))
                        .collect::<Vec<Field>>()
                        .as_slice()
                        .try_into()
                        .unwrap();

                    numbers[i] = line;
                }

                let board = Board { numbers };
                boards.push(board);
            }

            Self {
                numbers,
                boards,
                next: 0,
            }
        }
    }

    #[derive(PartialEq, Clone, Copy)]
    pub struct Board {
        pub numbers: [[Field; 5]; 5],
    }

    impl Board {
        pub fn mark(&mut self, num: u16) {
            for (i, row) in self.numbers.into_iter().enumerate() {
                for (j, field) in row.into_iter().enumerate() {
                    if Field::Unmarked(num) == field {
                        self.numbers[i][j] = Field::Marked(num);
                    }
                }
            }
        }

        pub fn won(self) -> bool {
            // check rows
            if self.numbers.into_iter().any(|row| {
                row.into_iter().all(|e| match e {
                    Field::Marked(_) => true,
                    _ => false,
                })
            }) {
                return true;
            }

            // check cols
            (0..5).any(|col| {
                (0..5).all(|row| match self.numbers[row][col] {
                    Field::Marked(_) => true,
                    _ => false,
                })
            })
        }

        pub fn score(self) -> u32 {
            let mut score = 0;

            for row in self.numbers {
                for f in row {
                    if let Field::Unmarked(n) = f {
                        score += n as u32;
                    }
                }
            }

            score
        }
    }

    impl From<[[u16; 5]; 5]> for Board {
        fn from(lines: [[u16; 5]; 5]) -> Self {
            let mut numbers = [[Field::Unmarked(0); 5]; 5];
            for i in 0..5 {
                for j in 0..5 {
                    numbers[i][j] = Field::Unmarked(lines[i][j]);
                }
            }

            Self { numbers }
        }
    }

    impl Debug for Board {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "Board {{\n")?;
            for row in self.numbers {
                for field in row {
                    match field {
                        Field::Marked(n) => write!(f, "({:2}) ", n)?,
                        Field::Unmarked(n) => write!(f, " {:2}  ", n)?,
                    }
                }
                write!(f, "\n")?;
            }
            write!(f, "\n}}")
        }
    }

    #[derive(PartialEq, Debug, Clone, Copy)]
    pub enum Field {
        Unmarked(u16),
        Marked(u16),
    }
}

pub fn day04p1(input: &str) -> u32 {
    let mut game = bingo::Game::from(input);

    loop {
        game.advance();
        if let Some((number, board)) = game.any_won() {
            return board.score() * (number as u32);
        }
    }
}

pub fn day04p2(_input: &str) -> u32 {
    0
}

#[cfg(test)]
mod tests {
    use super::bingo::{Field, Game};
    use super::*;

    const INPUT: &str = "7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

22 13 17 11  0
 8  2 23  4 24
21  9 14 16  7
 6 10  3 18  5
 1 12 20 15 19

3 15  0  2 22
 9 18 13 17  5
19  8  7 25 23
20 11 10 24  4
14 21 16 12  6

14 21 17 24  4
10 16 15  9 19
18  8 23 26 20
22 11 13  6  5
 2  0 12  3  7
";

    #[test]
    fn part1_examples() {
        assert_eq!(4512, day04p1(INPUT));
    }

    #[test]
    fn part2_examples() {
        assert_eq!(0, day04p2(INPUT));
    }

    fn test_board() -> bingo::Board {
        bingo::Board::from([
            [1, 2, 3, 4, 5],
            [6, 7, 8, 9, 10],
            [11, 12, 13, 14, 15],
            [16, 17, 18, 19, 20],
            [21, 22, 23, 24, 25],
        ])
    }

    #[test]
    fn test_game_from_str() {
        let game = bingo::Game::from(INPUT);
        assert_eq!(
            vec![
                7, 4, 9, 5, 11, 17, 23, 2, 0, 14, 21, 24, 10, 16, 13, 6, 15, 25, 12, 22, 18, 20, 8,
                19, 3, 26, 1
            ],
            game.numbers
        );

        assert_eq!(
            bingo::Board::from([
                [22, 13, 17, 11, 0],
                [8, 2, 23, 4, 24],
                [21, 9, 14, 16, 7],
                [6, 10, 3, 18, 5],
                [1, 12, 20, 15, 19]
            ]),
            game.boards[0]
        );
        assert_eq!(
            bingo::Board::from([
                [3, 15, 0, 2, 22],
                [9, 18, 13, 17, 5],
                [19, 8, 7, 25, 23],
                [20, 11, 10, 24, 4],
                [14, 21, 16, 12, 6]
            ]),
            game.boards[1]
        );
        assert_eq!(
            bingo::Board::from([
                [14, 21, 17, 24, 4],
                [10, 16, 15, 9, 19],
                [18, 8, 23, 26, 20],
                [22, 11, 13, 6, 5],
                [2, 0, 12, 3, 7]
            ]),
            game.boards[2]
        );
    }

    #[test]
    fn test_board_mark() {
        let mut expected = bingo::Board::from([
            [14, 21, 17, 24, 4],
            [10, 16, 15, 9, 19],
            [18, 8, 23, 26, 20],
            [22, 11, 13, 6, 5],
            [2, 0, 12, 3, 7],
        ]);

        let mut actual = expected.clone();
        actual.mark(14);
        actual.mark(8);
        actual.mark(0);
        expected.numbers[0][0] = Field::Marked(14);
        expected.numbers[2][1] = Field::Marked(8);
        expected.numbers[4][1] = Field::Marked(0);

        assert_eq!(expected, actual);
    }

    #[test]
    fn test_game_advance() {
        let mut game = Game::from(INPUT);
        let mut board0 = game.boards[0].clone();
        let mut board1 = game.boards[1].clone();
        let mut board2 = game.boards[2].clone();

        game.advance();

        board0.numbers[2][4] = Field::Marked(7);
        board1.numbers[2][2] = Field::Marked(7);
        board2.numbers[4][4] = Field::Marked(7);

        assert_eq!(board0, game.boards[0]);
        assert_eq!(board1, game.boards[1]);
        assert_eq!(board2, game.boards[2]);
    }

    #[test]
    fn test_board_won() {
        let board = test_board();
        assert!(!board.won());

        let mut board = test_board();
        board.mark(1);
        board.mark(2);
        board.mark(3);
        board.mark(4);
        board.mark(5);
        assert!(board.won());

        let mut board = test_board();
        board.mark(2);
        board.mark(7);
        board.mark(12);
        board.mark(17);
        board.mark(22);
        assert!(board.won());
    }
}
