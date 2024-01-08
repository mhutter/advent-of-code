use std::str::FromStr;

const MAX_RED: u16 = 12;
const MAX_GREEN: u16 = 13;
const MAX_BLUE: u16 = 14;

#[derive(Debug)]
struct Game {
    id: u16,
    sets: Vec<Set>,
}

impl Game {
    pub fn is_possible(&self) -> bool {
        self.sets
            .iter()
            .all(|set| set.red <= MAX_RED && set.green <= MAX_GREEN && set.blue <= MAX_BLUE)
    }
}

impl FromStr for Game {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (front, back) = s.split_once(": ").expect("game input line");
        let id = front
            .strip_prefix("Game ")
            .expect("'Game ' prefix")
            .parse()
            .expect("numeric game ID");

        let sets = back
            .trim()
            .split("; ")
            .map(Set::from_str)
            .collect::<Result<Vec<_>, _>>()
            .expect("parse all sets");

        Ok(Self { id, sets })
    }
}

#[derive(Debug, Default, Clone, Copy)]
struct Set {
    red: u16,
    green: u16,
    blue: u16,
}

impl FromStr for Set {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.trim();
        let mut set = Self::default();

        for (n, color) in s.split(", ").map(|p| {
            let (n, color) = p.split_once(' ').expect("N COLOR");
            (n.parse::<u16>().expect("number of cubes"), color)
        }) {
            match color {
                "red" => set.red += n,
                "green" => set.green += n,
                "blue" => set.blue += n,
                _ => panic!("unexpected color: `{color}`"),
            }
        }

        Ok(set)
    }
}

impl<'a> std::iter::Sum<&'a Set> for Set {
    fn sum<I: Iterator<Item = &'a Set>>(iter: I) -> Self {
        iter.fold(Self::default(), |acc, e| acc + *e)
    }
}

impl core::ops::Add for Set {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            red: self.red + rhs.red,
            green: self.green + rhs.green,
            blue: self.blue + rhs.blue,
        }
    }
}

pub fn day02p1(input: &str) -> u16 {
    input
        .lines()
        .filter_map(|line| {
            // parse input
            let game = Game::from_str(line).expect("parse game");
            // if the game is possible, return its ID
            game.is_possible().then_some(game.id)
        })
        .sum()
}

pub fn day02p2(_input: &str) -> u16 {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_examples() {
        assert_eq!(8, day02p1(INPUT));
    }

    #[test]
    fn part2_examples() {
        assert_eq!(0, day02p2(INPUT));
    }

    const INPUT: &str = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";
}
