use advent_of_code_2021::{day05::day05p2, generate};

fn main() {
    let i = generate::ints(include_str!("../../input/day05.txt"));
    dbg!(day05p2(&i));
}
