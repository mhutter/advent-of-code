use advent_of_code_2021::{day07::day07p2, generate};

fn main() {
    let i = generate::ints(include_str!("../../input/day07.txt"));
    dbg!(day07p2(&i));
}
