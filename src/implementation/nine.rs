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

    fn part_one(&self) -> usize {
        let mut my_vec: Vec<i32> = Vec::new();
        let mut id = 0;
        for (idx, v) in self.data.iter().enumerate() {
            if idx % 2 == 0 || idx == 0 {
                my_vec.extend(vec![id; *v as usize]);
                id += 1;
            } else {
                my_vec.extend(vec![-1; *v as usize])
            }
        }

        let mut a = 0;
        let mut b = my_vec.len() - 1;
        loop {
            if my_vec[a] == -1 {
                if my_vec[b] != -1 {
                    my_vec.swap(a, b);
                    a += 1;
                    b -= 1;
                } else {
                    b -= 1;
                }
            } else {
                a += 1;
            }
            if my_vec[a..my_vec.len()].iter().all(|s| *s == -1) {
                break;
            }
        }
        my_vec[0..a]
            .iter()
            .enumerate()
            .map(|(i, s)| i * *s as usize)
            .sum()
    }

    fn part_two(&self) -> i64 {
        let mut my_vec: Vec<i64> = Vec::new();
        let mut id = 0;
        for (idx, v) in self.data.iter().enumerate() {
            if idx % 2 == 0 || idx == 0 {
                my_vec.extend(vec![id; *v as usize]);
                id += 1;
            } else {
                my_vec.extend(vec![-1; *v as usize])
            }
        }
        let mut b = my_vec.len() - 1;
        while b > 0 {
            if my_vec[0..b].iter().all(|s| *s != -1) {
                break;
            }
            if my_vec[b] != -1 {
                let b_range = (0..b).position(|i| my_vec[b] != my_vec[b - i]).unwrap_or(0);
                let mut a = 0;
                'inner: loop {
                    if a >= b {
                        b -= b_range;
                        break 'inner;
                    }
                    if my_vec[a] == -1 {
                        let a_range = (a..b).position(|x| my_vec[x] != -1).unwrap_or(0);
                        if a_range >= b_range {
                            for i in 0..b_range {
                                my_vec.swap(a + i, b - i);
                            }
                            b -= b_range;
                            break 'inner;
                        } else {
                            a += a_range;
                        }
                    }
                    a += 1;
                }
            } else {
                b -= 1;
            }
        }
        my_vec
            .iter()
            .enumerate()
            .map(|(i, s)| if *s == -1 { 0 } else { *s * i as i64 })
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
