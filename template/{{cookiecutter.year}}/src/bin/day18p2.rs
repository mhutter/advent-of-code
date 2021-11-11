use common::generate;

use advent_of_code_{{cookiecutter.year}}::day18::day18p2;

fn main() {
    let i = generate::ints(include_str!("../../input/day18.txt"));
    dbg!(day18p2(&i));
}
