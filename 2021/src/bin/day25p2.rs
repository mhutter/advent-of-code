use advent_of_code_2021::{day25::day25p2, generate};

fn main() {
    let i = generate::ints(include_str!("../../input/day25.txt"));
    dbg!(day25p2(&i));
}
