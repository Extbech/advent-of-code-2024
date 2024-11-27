use advent_of_code_2024::*;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    AdventOfCodeSolver::new().solve(args);
}
