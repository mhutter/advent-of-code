use common::generate;

use advent_of_code_{{cookiecutter.year}}::day01::day01p2;

fn main() {
    let i = generate::ints(include_str!("../../input/day01.txt"));
    dbg!(day01p2(&i));
}