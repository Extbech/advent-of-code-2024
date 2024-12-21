use std::cmp::min;

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
        let mut i = 0;
        let mut j = self.data.len() - 1;
        let mut left_id = 0;
        let mut right_id = self.data.len() / 2;
        let mut left_len = self.data[i];
        let mut right_len = self.data[j];
        let mut empty = false;
        let mut sum = 0u64;
        let mut k = 0;
        loop {
            while left_len == 0 && i < j {
                i += 1;
                empty ^= true;
                if i < j {
                    left_len = self.data[i];
                }
                if !empty {
                    left_id += 1;
                }
            }
            while right_len == 0 && i < j {
                j -= 2;
                if i < j {
                    right_len = self.data[j];
                }
                right_id -= 1;
            }
            if j <= i {
                let len = (right_len + if !empty { left_len } else { 0 }) as usize;
                sum += (left_id * ((k+len)*(k+len-1) - (k-1)*k) / 2) as u64;
                break;
            }
            if empty {
                let m = min(right_len, left_len) as usize;
                right_len -= m as u32;
                left_len -= m as u32;
                sum += ((((k+m).wrapping_sub(1))*(k+m) - (k.wrapping_sub(1))*k) / 2 * right_id) as u64;
                k += m as usize;
            } else {
                let m = left_len as usize;
                left_len = 0;
                sum += ((((k+m).wrapping_sub(1))*(k+m) - (k.wrapping_sub(1))*k) / 2 * left_id) as u64;
                k += m;
            }
        }
        sum
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

/* 0099811188827773336446555566
0  0*0
0  0*1
18 9*2
45 18 + 9*3
77 45 + 8*4
82 77 + 1*5
88 82 + 1*6
95 88 + 1*7
159 95 + 8*8
231 159 + 8*9
311 231 + 8*10
333 311 + 2*11
417 333 + 7*12
508 417 + 7*13
606 508 + 7*14
651 606 + 3*15
699 651 + 3*16
750 699 + 3*17
858 750 + 6*18
934 858 + 4*19
1014 934 + 4*20
1140 1014 + 6*21
1250 1140 + 5*22
1365 1250 + 5*23
1485 1365 + 5*24
1610 1485 + 5*25
1766 1610 + 6*26
*/

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
