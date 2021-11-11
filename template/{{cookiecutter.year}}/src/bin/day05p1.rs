use common::generate;

use advent_of_code_{{cookiecutter.year}}::day05::day05p1;

fn main() {
    let i = generate::ints(include_str!("../../input/day05.txt"));
    dbg!(day05p1(&i));
}
