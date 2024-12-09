use std::iter::zip;

use crate::Solution;

pub struct DaySevenSolution {
    result: Vec<u64>,
    input: Vec<Vec<u64>>,
}

impl Solution for DaySevenSolution {
    const DAY: u8 = 7;

    fn new() -> Self {
        let data = parse_input(Self::read_data_to_vec().unwrap());
        DaySevenSolution {
            result: data.0,
            input: data.1,
        }
    }

    fn part_one(&self) -> u64 {
        let mut res = 0;
        for (input, result) in zip(&self.input, &self.result) {
            if equation_is_valid(input, *result) {
                res += result;
            }
        }
        res
    }

    fn part_two(&self) -> u64 {
        let mut res = 0;
        for (input, result) in zip(&self.input, &self.result) {
            if equation_is_valid2(input, *result) {
                res += result;
            }
        }
        res
    }
}

fn parse_input(input: Vec<String>) -> (Vec<u64>, Vec<Vec<u64>>) {
    input
        .into_iter()
        .map(|s| {
            (
                s.split(':').collect::<Vec<&str>>()[0]
                    .parse::<u64>()
                    .unwrap(),
                s.split(':').collect::<Vec<&str>>()[1]
                    .split_whitespace()
                    .map(|x| x.parse::<u64>().unwrap())
                    .collect(),
            )
        })
        .unzip()
}

fn equation_is_valid(input: &[u64], result: u64) -> bool {
    if input.len() == 1 {
        return result == input[0];
    }
    let m = input.len() - 1;
    if result % input[m] == 0 && equation_is_valid(&input[..m], result / input[m]) {
        return true;
    }
    if input[m] <= result && equation_is_valid(&input[..m], result - input[m]) {
        return true;
    }
    false
}

fn equation_is_valid2(input: &[u64], result: u64) -> bool {
    let ans1 = equation_is_valid2_optimized(input, result);
    debug_assert_eq!(
        ans1,
        equation_is_valid2_unoptimized(&input[1..], result, input[0]),
        "{} : {:?}",
        result,
        input
    );
    ans1
}

fn equation_is_valid2_optimized(input: &[u64], result: u64) -> bool {
    if input.len() == 1 {
        return result == input[0];
    }
    let m = input.len() - 1;
    if result % input[m] == 0 && equation_is_valid2_optimized(&input[..m], result / input[m]) {
        return true;
    }
    if result % 10u64.pow(input[m].ilog10() + 1) == input[m]
        && equation_is_valid2_optimized(&input[..m], result / 10u64.pow(input[m].ilog10() + 1))
    {
        return true;
    }
    if input[m] <= result && equation_is_valid2_optimized(&input[..m], result - input[m]) {
        return true;
    }
    false
}

fn equation_is_valid2_unoptimized(input: &[u64], result: u64, accumulator: u64) -> bool {
    if input.is_empty() {
        return result == accumulator;
    }
    equation_is_valid2_unoptimized(&input[1..], result, accumulator * input[0])
        || equation_is_valid2_unoptimized(&input[1..], result, accumulator + input[0])
        || equation_is_valid2_unoptimized(
            &input[1..],
            result,
            concatenate_int(accumulator, input[0]),
        )
}

fn concatenate_int(a: u64, b: u64) -> u64 {
    a * 10u64.pow(b.ilog10() + 1) + b
}
