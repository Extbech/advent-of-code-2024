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
        while a < my_vec.len() - 1 {
            if my_vec[a] == "." {
                let a_range = my_vec[a..my_vec.len()].iter().position(|a| a != ".").unwrap_or(0);
                if a_range == 0 {
                    for i in 0..my_vec.len() {
                        if my_vec[my_vec.len() - 1 - i] != "." {
                            my_vec.swap(a, i);
                        }
                }
                a += 1;
            } else {
                let mut b = my_vec.len() - 1;
                'inner: loop {
                    if b <= a {
                        a += a_range;
                        break 'inner;
                    }
                    if my_vec[b] != "." {
                        let b_range = (0..b).position(|i| my_vec[b] != my_vec[b - i]).unwrap_or(0);
                        if b_range <= a_range {
                            for i in 0..((b_range) + 1) {
                                my_vec.swap(a + i, b - i);
                            }
                            a += (b_range) + 1;
                            println!("{:?}", a);
                            break 'inner;
                        } else {
                            b -= b_range + 1;
                        }
                    } else {
                        b -= 1;
                    }
                }
                }
            } else {
                a += 1;
            }
        }
        println!("{:?}", my_vec);
        my_vec
            .iter()
            .enumerate()
            .map(|(i, s)| i as u64 * s.parse::<u64>().unwrap_or(0))
            .sum()
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

    #[test]
    fn test_part_two() {
        let day_nine = get_day_nine();
        let output = day_nine.part_two();
        assert_eq!(output, 2858);
    }
}
