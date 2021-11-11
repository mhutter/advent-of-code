use advent_of_code_2021::{day21::day21p1, generate};

fn main() {
    let i = generate::ints(include_str!("../../input/day21.txt"));
    dbg!(day21p1(&i));
}
