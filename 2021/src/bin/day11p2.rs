use advent_of_code_2021::{day11::day11p2, generate};

fn main() {
    let i = generate::ints(include_str!("../../input/day11.txt"));
    dbg!(day11p2(&i));
}
