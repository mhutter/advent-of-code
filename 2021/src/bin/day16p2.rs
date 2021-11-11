use advent_of_code_2021::{day16::day16p2, generate};

fn main() {
    let i = generate::ints(include_str!("../../input/day16.txt"));
    dbg!(day16p2(&i));
}
