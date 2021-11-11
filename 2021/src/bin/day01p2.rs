use advent_of_code_2021::{day01::day01p2, generate};

fn main() {
    let i = generate::ints(include_str!("../../input/day01.txt"));
    dbg!(day01p2(&i));
}
