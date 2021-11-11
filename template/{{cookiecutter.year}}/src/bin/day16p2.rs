use common::generate;

use advent_of_code_{{cookiecutter.year}}::day16::day16p2;

fn main() {
    let i = generate::ints(include_str!("../../input/day16.txt"));
    dbg!(day16p2(&i));
}
