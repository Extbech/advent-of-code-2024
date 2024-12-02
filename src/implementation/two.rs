use crate::Solution;

pub struct DayTwoSolution {
    data: Vec<Vec<u32>>,
}

impl Solution for DayTwoSolution {
    const DAY: u8 = 2;

    fn new() -> Self {
        DayTwoSolution {
            data: parse_input(&Self::read_data_to_vec().unwrap()),
        }
    }

    fn part_one(&self) -> usize {
        self.data
            .iter()
            .filter(|report| report_is_safe(report.iter()))
            .count()
    }

    fn part_two(&self) -> usize {
        self.data
            .iter()
            .filter(|report| report_is_safe_with_one_removal(report))
            .count()
    }
}

fn parse_input(data: &[String]) -> Vec<Vec<u32>> {
    data.iter()
        .map(|s| s.split_whitespace().map(|c| c.parse().unwrap()).collect())
        .collect()
}

fn report_is_safe<'a>(report: impl Iterator<Item = &'a u32> + Clone) -> bool {
    let mut decreasing = false;
    let mut increasing = false;
    !report.clone().zip(report.skip(1)).any(|(a, b)| {
        if a.abs_diff(*b) >= 1 && a.abs_diff(*b) <= 3 {
            if a > b {
                decreasing = true;
                increasing
            } else {
                increasing = true;
                decreasing
            }
        } else {
            true
        }
    })
}

fn report_is_safe_with_one_removal(report: &[u32]) -> bool {
    let res = report_is_safe(report.iter());
    if res {
        return true;
    }
    (0..report.len())
        .map(|i| report.iter().enumerate().filter(move |(idx, _)| *idx != i))
        .any(|slice| report_is_safe(slice.map(|(_, b)| b)))
}
