use advent_of_code_2024::{
    implementation::{
        eight::DayEightSolution, eleven::DayElevenSolution, five::DayFiveSolution,
        four::DayFourSolution, nine::DayNineSolution, one::DayOneSolution, seven::DaySevenSolution,
        six::DaySixSolution, ten::DayTenSolution, three::DayThreeSolution,
        twelve::DayTwelveSolution, two::DayTwoSolution,
    },
    Solution,
};

use criterion::{criterion_group, criterion_main, Criterion};

fn benchmark_aoc_day_one(c: &mut Criterion) {
    let day_one = DayOneSolution::new();
    let mut group = c.benchmark_group("AOC day 1");

    group.bench_function("Solution one", |b| b.iter(|| day_one.part_one()));
    group.bench_function("Solution two", |b| b.iter(|| day_one.part_two()));
    group.finish();
}

fn benchmark_aoc_day_two(c: &mut Criterion) {
    let day_two = DayTwoSolution::new();
    let mut group = c.benchmark_group("AOC day 2");

    group.bench_function("Solution one", |b| b.iter(|| day_two.part_one()));
    group.bench_function("Solution two", |b| b.iter(|| day_two.part_two()));
    group.finish();
}

fn benchmark_aoc_day_three(c: &mut Criterion) {
    let day_three = DayThreeSolution::new();
    let mut group = c.benchmark_group("AOC day 3");

    group.bench_function("Solution one", |b| b.iter(|| day_three.part_one()));
    group.bench_function("Solution two", |b| b.iter(|| day_three.part_two()));
    group.finish();
}

fn benchmark_aoc_day_four(c: &mut Criterion) {
    let day_four = DayFourSolution::new();
    let mut group = c.benchmark_group("AOC day 4");

    group.bench_function("Solution one", |b| b.iter(|| day_four.part_one()));
    group.bench_function("Solution two", |b| b.iter(|| day_four.part_two()));
    group.finish();
}

fn benchmark_aoc_day_five(c: &mut Criterion) {
    let day_five = DayFiveSolution::new();
    let mut group = c.benchmark_group("AOC day 5");

    group.bench_function("Solution one", |b| b.iter(|| day_five.part_one()));
    group.bench_function("Solution two", |b| b.iter(|| day_five.part_two()));
    group.finish();
}

fn benchmark_aoc_day_six(c: &mut Criterion) {
    let day_six = DaySixSolution::new();
    let mut group = c.benchmark_group("AOC day 6");

    group.bench_function("Solution one", |b| b.iter(|| day_six.part_one()));
    group.bench_function("Solution two", |b| b.iter(|| day_six.part_two()));
    group.finish();
}

fn benchmark_aoc_day_seven(c: &mut Criterion) {
    let day_seven = DaySevenSolution::new();
    let mut group = c.benchmark_group("AOC day 7");

    group.bench_function("Solution one", |b| b.iter(|| day_seven.part_one()));
    group.bench_function("Solution two", |b| b.iter(|| day_seven.part_two()));
    group.finish();
}

fn benchmark_aoc_day_eight(c: &mut Criterion) {
    let day_eight = DayEightSolution::new();
    let mut group = c.benchmark_group("AOC day 8");

    group.bench_function("Solution one", |b| b.iter(|| day_eight.part_one()));
    group.bench_function("Solution two", |b| b.iter(|| day_eight.part_two()));
    group.finish();
}

fn benchmark_aoc_day_nine(c: &mut Criterion) {
    let day_nine = DayNineSolution::new();
    let mut group = c.benchmark_group("AOC day 9");

    group.bench_function("Solution one", |b| b.iter(|| day_nine.part_one()));
    group.bench_function("Solution two", |b| b.iter(|| day_nine.part_two()));
    group.finish();
}

fn benchmark_aoc_day_ten(c: &mut Criterion) {
    let day_ten = DayTenSolution::new();
    let mut group = c.benchmark_group("AOC day 10");

    group.bench_function("Solution one", |b| b.iter(|| day_ten.part_one()));
    group.bench_function("Solution two", |b| b.iter(|| day_ten.part_two()));
    group.finish();
}

fn benchmark_aoc_day_eleven(c: &mut Criterion) {
    let day_eleven = DayElevenSolution::new();
    let mut group = c.benchmark_group("AOC day 11");

    group.bench_function("Solution one", |b| b.iter(|| day_eleven.part_one()));
    group.bench_function("Solution two", |b| b.iter(|| day_eleven.part_two()));
    group.finish();
}

fn benchmark_aoc_day_twelve(c: &mut Criterion) {
    let day_twelve = DayTwelveSolution::new();
    let mut group = c.benchmark_group("AOC day 12");

    group.bench_function("Solution one", |b| b.iter(|| day_twelve.part_one()));
    group.bench_function("Solution two", |b| b.iter(|| day_twelve.part_two()));
    group.finish();
}

criterion_group!(
    benches,
    benchmark_aoc_day_one,
    benchmark_aoc_day_two,
    benchmark_aoc_day_three,
    benchmark_aoc_day_four,
    benchmark_aoc_day_five,
    benchmark_aoc_day_six,
    benchmark_aoc_day_seven,
    benchmark_aoc_day_eight,
    benchmark_aoc_day_nine,
    benchmark_aoc_day_ten,
    benchmark_aoc_day_eleven,
    benchmark_aoc_day_twelve
);
criterion_main!(benches);
