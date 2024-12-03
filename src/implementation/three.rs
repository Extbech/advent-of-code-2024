use crate::Solution;
use regex::Regex;
pub struct DayThreeSolution {
    data: String,
}

impl Solution for DayThreeSolution {
    const DAY: u8 = 3;

    fn new() -> Self {
        DayThreeSolution {
            data: Self::read_data_to_string().unwrap(),
        }
    }

    fn part_one(&self) -> u32 {
        let re = regex::Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
        re.captures_iter(&self.data)
            .map(|c| c.extract())
            .map(|(_, [a, b])| a.parse::<u32>().unwrap() * b.parse::<u32>().unwrap())
            .sum()
    }

    fn part_two(&self) -> u32 {
        let re = Regex::new(r"do\(\)|don't\(\)|mul\((\d+),(\d+)\)").unwrap();
        let mut enable_mul = true;
        let mut sum = 0;

        for cap in re.captures_iter(&self.data) {
            match cap.get(0).map_or("", |m| m.as_str()) {
                "do()" => enable_mul = true,
                "don't()" => enable_mul = false,
                _ => {
                    if enable_mul {
                        if let (Some(a), Some(b)) = (cap.get(1), cap.get(2)) {
                            sum += a.as_str().parse::<u32>().unwrap()
                                * b.as_str().parse::<u32>().unwrap();
                        }
                    }
                }
            }
        }
        sum
    }
}
