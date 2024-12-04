use std::cmp::min;

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

fn report_is_safe<'a>(report: impl DoubleEndedIterator<Item = &'a u32> + Clone) -> bool {
    report_is_safe_one_sided(report.clone(), true) || report_is_safe_one_sided(report, false)
}

// If we know the order of the report we can reduce the number of checks
fn report_is_safe_one_sided<'a>(
    report: impl Iterator<Item = &'a u32> + Clone,
    increasing: bool,
) -> bool {
    report
        .clone()
        .zip(report.clone().skip(1))
        .all(|(&a, &b)| is_safe_pair(a, b, increasing))
}

fn is_safe_pair(first: u32, second: u32, increasing: bool) -> bool {
    if increasing {
        is_safe_pair_increasing(first, second)
    } else {
        is_safe_pair_increasing(second, first)
    }
}

fn is_safe_pair_increasing(first: u32, second: u32) -> bool {
    first < second && second <= 3 + first
}

fn drop_second(s: &[u32]) -> impl DoubleEndedIterator<Item = &u32> + Clone {
    std::iter::once(&s[0]).chain(&s[min(2, s.len())..])
}

fn report_is_safe_with_one_removal(report: &[u32]) -> bool {
    let ans1 = report_is_safe_with_one_removal_optimized(report);
    #[cfg(debug_assertions)]
    {
        let ans2 = report_is_safe_with_one_removal_spec(report);
        debug_assert_eq!(ans1, ans2, "{:?}", report);
    }
    ans1
}

fn report_is_safe_with_one_removal_optimized(report: &[u32]) -> bool {
    // If neither of the two first elements can be removed, they must be kept
    let res = report_is_safe(report.iter().skip(1)) || report_is_safe(drop_second(report));
    if res {
        return true;
    }
    // since the two first elements are kept, their order determines the total order
    let increasing = report[0] < report[1];
    if !is_safe_pair(report[0], report[1], increasing) {
        return false;
    }
    for i in 1..(report.len() - 2) {
        // First time we find a cause of unsafety, we see what we have to remove to solve it
        if !is_safe_pair(report[i], report[i + 1], increasing) {
            if is_safe_pair(report[i], report[i + 2], increasing) {
                // skip i+1
                return report_is_safe_one_sided(drop_second(&report[(i + 2)..]), increasing);
            } else if is_safe_pair(report[i - 1], report[i + 1], increasing) {
                // skip i
                return report_is_safe_one_sided(drop_second(&report[(i + 1)..]), increasing);
            } else {
                return false;
            }
        }
    }
    true
}

#[test]
fn test_optimized() {
    assert!(report_is_safe_with_one_removal_optimized(&[
        64, 67, 69, 70, 68, 71, 72
    ]))
}

fn drop_nth(s: &[u32], n: usize) -> impl DoubleEndedIterator<Item = &u32> + Clone {
    s[..n].iter().chain(&s[(n + 1)..])
}

fn report_is_safe_with_one_removal_spec(report: &[u32]) -> bool {
    (0..report.len()).any(|i| report_is_safe(drop_nth(report, i)))
}
