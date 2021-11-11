use advent_of_code_2021::{day03::day03p1, generate};

fn main() {
    let i = generate::ints(include_str!("../../input/day03.txt"));
    dbg!(day03p1(&i));
}
