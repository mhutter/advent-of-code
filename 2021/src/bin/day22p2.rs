use advent_of_code_2021::{day22::day22p2, generate};

fn main() {
    let i = generate::ints(include_str!("../../input/day22.txt"));
    dbg!(day22p2(&i));
}