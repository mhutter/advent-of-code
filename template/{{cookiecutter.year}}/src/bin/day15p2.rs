use common::generate;

use advent_of_code_{{cookiecutter.year}}::day15::day15p2;

fn main() {
    let i = generate::ints(include_str!("../../input/day15.txt"));
    dbg!(day15p2(&i));
}
