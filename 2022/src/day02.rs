enum Hand {
    Rock,
    Paper,
    Scissors,
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

pub fn day02p1(input: &str) -> usize {
    input
        .lines()
        .map(|line| {
            // parse data
            let mut chars = line.chars();
            let enemy: Hand = chars.next().unwrap().into();
            chars.next().unwrap();
            let own: Hand = chars.next().unwrap().into();

            // determine score for the shape
            let shape_score = match own {
                Hand::Rock => 1,
                Hand::Paper => 2,
                Hand::Scissors => 3,
            };

            // determine outcome
            let outcome = match own {
                Hand::Rock => match enemy {
                    Hand::Rock => Outcome::Draw,
                    Hand::Paper => Outcome::Lost,
                    Hand::Scissors => Outcome::Win,
                },
                Hand::Paper => match enemy {
                    Hand::Rock => Outcome::Win,
                    Hand::Paper => Outcome::Draw,
                    Hand::Scissors => Outcome::Lost,
                },
                Hand::Scissors => match enemy {
                    Hand::Rock => Outcome::Lost,
                    Hand::Paper => Outcome::Win,
                    Hand::Scissors => Outcome::Draw,
                },
            };

            let outcome_score = match outcome {
                Outcome::Lost => 0,
                Outcome::Draw => 3,
                Outcome::Win => 6,
            };

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
        assert_eq!(0, day02p2(INPUT));
    }

    const INPUT: &str = "A Y\nB X\nC Z";
}
