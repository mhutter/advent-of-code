use advent_of_code_2021::{day18::day18p1, generate};

fn main() {
    let i = generate::ints(include_str!("../../input/day18.txt"));
    dbg!(day18p1(&i));
}
