use std::iter::zip;

use crate::Solution;

pub struct DaySevenSolution {
    result: Vec<u64>,
    input: Vec<Vec<u64>>
}

impl Solution for DaySevenSolution {
    const DAY: u8 = 7;

    fn new() -> Self {
        let data = parse_input(Self::read_data_to_vec().unwrap());
        DaySevenSolution { 
            result : data.0,
            input: data.1
        }
    }

    fn part_one(&self) -> u64 {
        let mut res = 0;
        for (input, result) in zip(&self.input, &self.result) {
            if equation_is_valid(result, input) {
                res += result;
            }
        }
        res
    }

    fn part_two(&self) -> u64 {
        43
    }
}

fn parse_input(input: Vec<String>) -> (Vec<u64>, Vec<Vec<u64>>) {
    input
        .into_iter()
        .map(|s| {
            (
                s.split(':').collect::<Vec<&str>>()[0].parse::<u64>().unwrap(),
                s.split(':').collect::<Vec<&str>>()[1].split_whitespace().map(|x| x.parse::<u64>().unwrap()).collect()
            )
        })
        .unzip()
}

fn equation_is_valid(result: &u64, input: &Vec<u64>) -> bool {
    let mut num_mult = 0;
    loop {
        let mut temp_res = 0;
        for i in 0..input.len() - 1 {
            temp_res += input[i] + input[i+1];
        }
        if &temp_res == result {
            return true;
        }
        if num_mult == input.len() - 1 {
            break
        }
        num_mult += 1;
    }
    false
}