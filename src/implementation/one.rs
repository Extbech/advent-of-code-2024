use std::collections::HashMap;

use crate::Solution;

pub struct DayOneSolution {
    data: Vec<String>,
}

impl Solution for DayOneSolution {
    const DAY: u8 = 1;

    fn new() -> Self {
        DayOneSolution {
            data: Self::read_data_to_vec().unwrap(),
        }
    }

    fn part_one(&self) -> u32 {
        let (mut right_list, mut left_list): (Vec<u32>, Vec<u32>) = parse_input(&self.data);
        right_list.sort();
        left_list.sort();
        // Quantum looping
        right_list
            .into_iter()
            .zip(left_list.into_iter())
            .map(|(r, l)| r.abs_diff(l))
            .sum()
    }

    fn part_two(&self) -> u32 {
        let (mut right_list, mut left_list): (Vec<u32>, Vec<u32>) = parse_input(&self.data);
        let mut similarity_score: u32 = 0;
        right_list.sort();
        left_list.sort();
        let mut idx_r = 0;
        let mut idx_l = 0;
        // Thing goes BRRRT
        'outer: loop {
            while left_list.get(idx_l) < right_list.get(idx_r) {
                if left_list.get(idx_l) == None {
                    break 'outer;
                }
                idx_l += 1;
            }
            while right_list.get(idx_r) < left_list.get(idx_l) {
                if right_list.get(idx_r) == None {
                    break 'outer;
                }
                idx_r += 1;
            }
            if left_list.get(idx_l) == right_list.get(idx_r) {
                let Some(&val) = left_list.get(idx_l) else {
                    break;
                };
                let index_l_before = idx_l;
                let index_r_before = idx_r;
                while left_list[idx_l] == val {
                    idx_l += 1;
                }
                while right_list[idx_r] == val {
                    idx_r += 1;
                }
                similarity_score += (idx_l as u32 - index_l_before as u32)
                    * val
                    * (idx_r as u32 - index_r_before as u32)
            }
        }
        similarity_score
    }
}

fn parse_input(input: &Vec<String>) -> (Vec<u32>, Vec<u32>) {
    input
        .iter()
        .map(|l| {
            (
                l.split_whitespace().collect::<Vec<&str>>()[0]
                    .parse::<u32>()
                    .unwrap(),
                l.split_whitespace().collect::<Vec<&str>>()[1]
                    .parse::<u32>()
                    .unwrap(),
            )
        })
        .unzip()
}
