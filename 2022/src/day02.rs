enum Hand {
    Rock,
    Paper,
    Scissors,
}

impl Hand {
    /// Calcluate the score for the shape.
    ///
    /// 1 for Rock, 2 for Paper, and 3 for Scissors
    pub fn score(&self) -> usize {
        match self {
            Self::Rock => 1,
            Self::Paper => 2,
            Self::Scissors => 3,
        }
    }

    /// Calcluate the outcome vs the given hand
    pub fn outcome_vs(&self, other: &Self) -> Outcome {
        match self {
            Hand::Rock => match other {
                Hand::Rock => Outcome::Draw,
                Hand::Paper => Outcome::Lost,
                Hand::Scissors => Outcome::Win,
            },
            Hand::Paper => match other {
                Hand::Rock => Outcome::Win,
                Hand::Paper => Outcome::Draw,
                Hand::Scissors => Outcome::Lost,
            },
            Hand::Scissors => match other {
                Hand::Rock => Outcome::Lost,
                Hand::Paper => Outcome::Win,
                Hand::Scissors => Outcome::Draw,
            },
        }
    }
}

impl From<char> for Hand {
    fn from(c: char) -> Self {
        match c {
            'A' | 'X' => Self::Rock,
            'B' | 'Y' => Self::Paper,
            'C' | 'Z' => Self::Scissors,
            _ => panic!("Invalid hand: {c}"),
        }
    }
}

enum Outcome {
    Lost,
    Draw,
    Win,
}

impl Outcome {
    /// Calculate the score for the given outcome
    ///
    /// 0 if you lost, 3 if the round was a draw, and 6 if you won
    pub fn score(&self) -> usize {
        match self {
            Self::Lost => 0,
            Self::Draw => 3,
            Self::Win => 6,
        }
    }
}

fn parse(line: &str) -> (Hand, Hand) {
    let mut chars = line.chars();
    let a: Hand = chars.next().unwrap().into();
    chars.next().unwrap();
    let b: Hand = chars.next().unwrap().into();
    (a, b)
}

pub fn day02p1(input: &str) -> usize {
    input
        .lines()
        .map(|line| {
            // parse data
            let (enemy, own) = parse(line);

            // determine score for the shape
            let shape_score = own.score();

            // determine outcome
            let outcome = own.outcome_vs(&enemy);
            let outcome_score = outcome.score();

            shape_score + outcome_score
        })
        .sum()
}

pub fn day02p2(_input: &str) -> usize {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_examples() {
        assert_eq!(15, day02p1(INPUT));
    }

    #[test]
    fn part2_examples() {
        assert_eq!(12, day02p2(INPUT));
    }

    const INPUT: &str = "A Y\nB X\nC Z";
}
