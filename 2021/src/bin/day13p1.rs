use advent_of_code_2021::{day13::day13p1, generate};

fn main() {
    let i = generate::ints(include_str!("../../input/day13.txt"));
    dbg!(day13p1(&i));
}
