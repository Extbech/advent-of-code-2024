use crate::Solution;

pub struct DayTenSolution {
    map: Vec<Vec<u8>>,
}

impl Solution for DayTenSolution {
    const DAY: u8 = 10;

    fn new() -> Self {
        DayTenSolution { map: parse_input(&Self::read_data_to_vec().unwrap()) }
    }

    fn part_one(&self) -> u32 {
        let mut sum = 0;
        for (y, row) in self.map.iter().enumerate() {
            for (x, num) in row.iter().enumerate() {
                if *num == 0 {
                    println!("Found {} at ({}, {})", find_num_trails(&self.map, (x, y)), x, y);
                    sum += find_num_trails(&self.map, (x, y));
                    return sum;
                }
            }
        }
        sum
    }

    fn part_two(&self) -> ! {
        todo!("Implement part_two function for DayTenSolution")
    }
}
fn parse_input(input: &[String]) -> Vec<Vec<u8>> {
    input
    .iter()
    .map(|line| line.chars().map(|c| c.to_digit(10).unwrap() as u8)
    .collect())
    .collect()
}

fn find_num_trails(map: &[Vec<u8>], cords: (usize, usize)) -> u32 {
    let mut sum = 0;
    let mut cumvec = vec![];
    find_next_step(map, cords, 0, &mut sum, &mut cumvec);
    for row in cumvec.chunks(9) {
        println!("{:?}", row);
    }
    return sum;
}

// Recursive function to find the next step
fn find_next_step(map: &[Vec<u8>], cords: (usize, usize), num: u8, accumulator: &mut u32, cumvec: &mut Vec<(usize, usize)>) {
    if num == 9 {
        *accumulator += 1;
        return
    }
    // Move left
    if cords.0 > 0 && map[cords.1][cords.0 - 1] == num + 1 {
        cumvec.push((cords.0 - 1, cords.1));
        find_next_step(map, (cords.0 - 1, cords.1), num + 1, accumulator, cumvec);
    }
    // Move right
    if cords.0 < map[0].len() - 1 && map[cords.1][cords.0 + 1] == num + 1 {
        cumvec.push((cords.0 + 1, cords.1));
        find_next_step(map, (cords.0 + 1, cords.1), num + 1, accumulator, cumvec);
    }
    // Move up
    if cords.1 > 0 && map[cords.1 - 1][cords.0] == num + 1 {
        cumvec.push((cords.0, cords.1 - 1));
        find_next_step(map, (cords.0, cords.1 - 1), num + 1, accumulator, cumvec);
    }
    // Move down
    if cords.1 < map.len() - 1 && map[cords.1 + 1][cords.0] == num + 1 {
        cumvec.push((cords.0, cords.1 + 1));
        find_next_step(map, (cords.0, cords.1 + 1), num + 1, accumulator, cumvec);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_day_ten() -> DayTenSolution {
        let input = vec![
            vec![8, 9, 0, 1, 0, 1, 2, 3],
            vec![7, 8, 1, 2, 1, 8, 7, 4],
            vec![8, 7, 4, 3, 0, 9, 6, 5],
            vec![9, 6, 5, 4, 9, 8, 7, 4],
            vec![4, 5, 6, 7, 8, 9, 0, 3],
            vec![3, 2, 0, 1, 9, 0, 1, 2],
            vec![0, 1, 3, 2, 9, 8, 0, 1],
            vec![1, 0, 4, 5, 6, 7, 3, 2]
        ];
        DayTenSolution { map: input }
    }
    
    #[test]
    fn test_part_one() {
        let day_ten = get_day_ten();
        assert_eq!(day_ten.part_one(), 36);
    }
}