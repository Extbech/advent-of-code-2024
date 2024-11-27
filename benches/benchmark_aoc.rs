use advent_of_code_2024::{DayOneSolution, Solution};
use criterion::{criterion_group, criterion_main, Criterion};

fn benchmark_aoc_day_one(c: &mut Criterion) {
    let day_one = DayOneSolution::new();
    let mut group = c.benchmark_group("AOC day 1");

    group.bench_function("Solution one", |b| b.iter(|| day_one.part_one()));
    group.bench_function("Solution two", |b| b.iter(|| day_one.part_two()));
    group.finish();
}

criterion_group!(benches, benchmark_aoc_day_one);
criterion_main!(benches);
