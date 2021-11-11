use advent_of_code_2021::{day23::day23p2, generate};

fn main() {
    let i = generate::ints(include_str!("../../input/day23.txt"));
    dbg!(day23p2(&i));
}
