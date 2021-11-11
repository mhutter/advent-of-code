use common::generate;

use advent_of_code_{{cookiecutter.year}}::day21::day21p2;

fn main() {
    let i = generate::ints(include_str!("../../input/day21.txt"));
    dbg!(day21p2(&i));
}
