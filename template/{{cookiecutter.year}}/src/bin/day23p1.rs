use common::generate;

use advent_of_code_{{cookiecutter.year}}::day23::day23p1;

fn main() {
    let i = generate::ints(include_str!("../../input/day23.txt"));
    dbg!(day23p1(&i));
}
