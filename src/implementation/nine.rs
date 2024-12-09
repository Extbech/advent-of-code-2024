use crate::Solution;

pub struct DayNineSolution {
    data: Vec<u32>,
}

impl Solution for DayNineSolution {
    const DAY: u8 = 9;

    fn new() -> Self {
        DayNineSolution {
            data: Self::read_data_to_string()
                .unwrap()
                .chars()
                .map(|c| c.to_digit(10).unwrap())
                .collect::<Vec<u32>>(),
        }
    }

    fn part_one(&self) -> u64 {
        let mut my_vec: Vec<String> = Vec::new();
        let mut id = 0;
        for (idx, v) in self.data.iter().enumerate() {
            if idx % 2 == 0 || idx == 0 {
                my_vec.extend(vec![id.to_string(); *v as usize]);
                id += 1;
            } else {
                my_vec.extend(vec![".".to_string(); *v as usize])
            }
        }

        let mut a = 0;
        let mut b = my_vec.len() - 1;
        loop {
            if my_vec[a] == "." {
                if my_vec[b] != "." {
                    my_vec.swap(a, b);
                    a += 1;
                    b -= 1;
                } else {
                    b -= 1;
                }
            } else {
                a += 1;
            }
            if my_vec[a..my_vec.len()].iter().all(|s| s == ".") {
                break;
            }
        }
        my_vec[0..a]
            .iter()
            .enumerate()
            .map(|(i, s)| i as u64 * s.parse::<u64>().unwrap())
            .sum()
    }

    fn part_two(&self) -> u64 {
        42
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    fn get_day_nine() -> DayNineSolution {
        DayNineSolution {
            data: "2333133121414131402"
                .to_string()
                .chars()
                .map(|c| c.to_digit(10).unwrap())
                .collect::<Vec<u32>>(),
        }
    }
    #[test]
    fn test_part_one() {
        let day_nine = get_day_nine();
        let output = day_nine.part_one();
        assert_eq!(output, 1928);
    }
}
