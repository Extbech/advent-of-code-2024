use std::collections::HashSet;

use crate::Solution;

pub struct DayEightSolution {
    grid: Vec<Vec<u8>>,
    antennas: HashSet<u8>,
}

impl Solution for DayEightSolution {
    const DAY: u8 = 8;

    fn new() -> Self {
        let data = parse_input(Self::read_data_to_vec().unwrap());
        DayEightSolution {
            grid: data.0,
            antennas: data.1,
        }
    }

    fn part_one(&self) -> usize {
        let mut marked_positions: HashSet<(usize, usize)> = HashSet::new();
        for antenna in &self.antennas {
            find_num_antinode(&self.grid, antenna, &mut marked_positions);
        }
        marked_positions.len()
    }

    fn part_two(&self) -> u32 {
        32
    }
}

fn parse_input(input: Vec<String>) -> (Vec<Vec<u8>>, HashSet<u8>) {
    (
        input.iter().map(|s| s.as_bytes().to_owned()).collect(),
        input
            .into_iter()
            .flat_map(|s| s.into_bytes())
            .filter(|c| *c != b'.')
            .collect(),
    )
}

fn find_num_antinode(grid: &[Vec<u8>], antenna: &u8, marked: &mut HashSet<(usize, usize)>) {
    let mut positions: HashSet<(usize, usize)> = HashSet::new();
    for y in 0..grid.len() {
        for x in 0..grid[y].len() {
            if grid[y][x] == *antenna {
                positions.insert((x, y));
            }
        }
    }
    println!("{}, {:?}", antenna, positions);
    for (i, pos_1) in positions.iter().enumerate() {
        for (j, pos_2) in positions.iter().enumerate() {
            if j != i {
                let diff = (pos_1.0.abs_diff(pos_2.0), pos_1.1.abs_diff(pos_2.1));
                if !outside_grid(pos_1, diff, grid) {
                    marked.insert((pos_1.0 - diff.0, pos_1.1 - diff.1));
                }
                if !outside_grid(pos_2, diff, grid) {
                    marked.insert((pos_2.0 + diff.0, pos_2.1 + diff.1));
                }
            }
        }
    }
}

fn outside_grid(pos_1: &(usize, usize), diff_2: &(usize, usize), grid: &[Vec<u8>]) -> bool {
    if pos.0.checked_sub(diff.0).is_none() || pos.1.checked_sub(diff.1).is_none() {
        return true;
    } else {
        if grid[pos.1 - diff.1][pos.0 - diff.0] != b'.' {
            return true;
        }
    }
    if pos.0 + diff.0 > grid[0].len() - 1 || pos.1 + diff.1 > grid.len() - 1 {
        return true;
    } else {
        if grid[pos.1 + diff.1][pos.0 + diff.0] != b'.' {
            return true;
        }
    }
    false
}

#[cfg(test)]
mod tests {
    use std::fs;

    use super::*;
    fn get_day_eight() -> DayEightSolution {
        let data = parse_input(
            fs::read_to_string("data/test/test_8.txt")
                .unwrap()
                .lines()
                .map(|x| x.to_string())
                .collect(),
        );
        DayEightSolution {
            grid: data.0,
            antennas: data.1,
        }
    }
    #[test]
    fn test_part_one() {
        let day_eight = get_day_eight();
        let output = day_eight.part_one();
        assert_eq!(output, 14);
    }
}
