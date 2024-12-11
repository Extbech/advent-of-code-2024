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
        let mut stones = self.data.clone();
        (0..25).for_each(|_| apply_rule(&mut stones));
        stones.len()
    }

    fn part_two(&self) -> u64 {
        42
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

#[cfg(test)]
mod tests {
    use super::*;

    fn get_day_eleven() -> DayElevenSolution {
        DayElevenSolution { data: vec![125, 17] }
    }

    #[test]
    fn test_part_one() {
        let day_eleven = get_day_eleven();
        assert_eq!(day_eleven.part_one(), 55312);
    }
}