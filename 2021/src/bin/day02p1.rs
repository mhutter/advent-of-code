use advent_of_code_2021::{day02::day02p1, generate};

fn main() {
    let i = generate::ints(include_str!("../../input/day02.txt"));
    dbg!(day02p1(&i));
}
