use common::generate;

use advent_of_code_{{cookiecutter.year}}::day07::day07p1;

fn main() {
    let i = generate::ints(include_str!("../../input/day07.txt"));
    dbg!(day07p1(&i));
}
