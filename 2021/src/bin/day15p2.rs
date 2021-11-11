use advent_of_code_2021::{day15::day15p2, generate};

fn main() {
    let i = generate::ints(include_str!("../../input/day15.txt"));
    dbg!(day15p2(&i));
}
