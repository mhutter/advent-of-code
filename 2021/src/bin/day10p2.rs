use advent_of_code_2021::{day10::day10p2, generate};

fn main() {
    let i = generate::ints(include_str!("../../input/day10.txt"));
    dbg!(day10p2(&i));
}
