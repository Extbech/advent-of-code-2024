use std::collections::HashMap;

use crate::Solution;

pub struct DayElevenSolution {
    data: Vec<u64>,
}

impl Solution for DayElevenSolution {
    const DAY: u8 = 11;

    fn new() -> Self {
        DayElevenSolution { data: parse_input(Self::read_data_to_string().unwrap()) }
    }

    fn part_one(&self) -> usize {
        let mut map: HashMap<(u64, u64), u64> = HashMap::new();
        self.data.iter().map(|stone| apply_rule_recursion(*stone, 25, &mut map)).sum()
    }

    fn part_two(&self) -> usize {
        let mut map: HashMap<(u64, u64), u64> = HashMap::new();
        self.data.iter().map(|stone| apply_rule_recursion(*stone, 75, &mut map)).sum()
    }
}

fn parse_input(input: String) -> Vec<u64> {
    input.split_whitespace().map(|l| l.parse().unwrap()).collect()
}

fn apply_rule(stones: &mut Vec<u64>) {
    let mut i = 0;
    while i < stones.len() {
        match stones[i] {
            0 => {
                stones[i] = 1;
                i += 1;
            },
            x if x.to_string().len() % 2 == 0 => {
                let num_str = x.to_string();
                let (p1, p2) = num_str.split_at(x.to_string().len() / 2);
                stones[i] = p1.parse().unwrap();
                stones.insert(i + 1, p2.parse().unwrap());
                i += 2;
            },
            _ => {
                stones[i] = stones[i] * 2024 as u64;
                i += 1;
            },
        }
    }
}

fn apply_rule_recursion(stone: u64, iter: u64, map: &mut HashMap<(u64, u64), u64>) -> usize {
    if iter == 0 {
        return 1;
    }
    
    if let Some(&res) = map.get(&(stone, iter)) {
        return res as usize;
    }
    
    let res = match stone {
        0 => apply_rule_recursion(1, iter - 1, map),
        x if x.to_string().len() % 2 == 0 => {
            let num_str = x.to_string();
            let (p1, p2) = num_str.split_at(x.to_string().len() / 2);
            apply_rule_recursion(p1.parse().unwrap(), iter - 1, map) + apply_rule_recursion(p2.parse().unwrap(), iter - 1, map)
        },
        _ => apply_rule_recursion(stone * 2024 as u64, iter - 1, map),
    };
    map.insert((stone, iter), res as u64);
    
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_day_eleven() -> DayElevenSolution {
        DayElevenSolution { data: vec![125, 17] }
    }

    #[test]
    fn test_part_one() {
        let day_eleven: DayElevenSolution = get_day_eleven();
        assert_eq!(day_eleven.part_one(), 55312);
    }
}