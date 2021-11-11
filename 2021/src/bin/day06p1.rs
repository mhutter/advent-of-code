use advent_of_code_2021::{day06::day06p1, generate};

fn main() {
    let i = generate::ints(include_str!("../../input/day06.txt"));
    dbg!(day06p1(&i));
}
