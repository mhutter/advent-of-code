use advent_of_code_2021::{day09::day09p1, generate};

fn main() {
    let i = generate::ints(include_str!("../../input/day09.txt"));
    dbg!(day09p1(&i));
}
