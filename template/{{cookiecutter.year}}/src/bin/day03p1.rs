use common::generate;

use advent_of_code_{{cookiecutter.year}}::day03::day03p1;

fn main() {
    let i = generate::ints(include_str!("../../input/day03.txt"));
    dbg!(day03p1(&i));
}
