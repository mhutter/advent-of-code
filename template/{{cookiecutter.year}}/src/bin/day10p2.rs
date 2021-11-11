use common::generate;

use advent_of_code_{{cookiecutter.year}}::day10::day10p2;

fn main() {
    let i = generate::ints(include_str!("../../input/day10.txt"));
    dbg!(day10p2(&i));
}
