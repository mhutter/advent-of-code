use advent_of_code_2022::day06::{day06p1, day06p2};

use criterion::{black_box, criterion_group, criterion_main, Criterion};

const INPUT: &str = include_str!("../input/day06.txt");

pub fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("Part 1", |b| b.iter(|| day06p1(black_box(INPUT))));
    c.bench_function("Part 2", |b| b.iter(|| day06p2(black_box(INPUT))));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
