use common::generate;

use advent_of_code_{{cookiecutter.year}}::day20::day20p2;

fn main() {
    let i = generate::ints(include_str!("../../input/day20.txt"));
    dbg!(day20p2(&i));
}