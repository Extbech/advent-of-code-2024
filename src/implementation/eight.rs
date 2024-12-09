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
            println!("{:?}", marked_positions);
        }
        visualize_grid(self.grid.clone(), &marked_positions);
        marked_positions.len()
    }
    
    fn part_two(&self) -> u32 {
        32
    }
}

fn visualize_grid(mut grid: Vec<Vec<u8>>, valid: &HashSet<(usize, usize)>) {
    for (index, row) in grid.iter_mut().enumerate() {
        let iter = valid.iter().filter(|(_x, y)| y == &index);
        for v in iter {
            row[v.0] = b'#';
        }
    }
    let mut vis_grid: [[char; 12]; 12] = [[char::default(); 12]; 12];
    for i in 0..grid.len() {
        for j in 0..grid[i].len() {
            vis_grid[i][j] = grid[i][j] as char;
        }
    }
    for row in &vis_grid {
        println!("{:?}", row);
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
    for (i, pos_1) in positions.iter().enumerate() {
        for (j, pos_2) in positions.iter().enumerate() {
            if j != i {
                if pos_1.1 < pos_2.1 {
                    for anti_pos in get_position(grid, pos_1, pos_2) {
                        if let Some(anti_pos) = anti_pos {
                            marked.insert(anti_pos);
                        }
                    }
                }
                else if pos_1.1 > pos_2.1 {
                    for anti_pos in get_position(grid, pos_2, pos_1) {
                        if let Some(anti_pos) = anti_pos {
                            marked.insert(anti_pos);
                        }
                    }
                } else {
                    if pos_1.0 < pos_2.0 {
                        for anti_pos in get_position(grid, pos_1, pos_2) {
                            if let Some(anti_pos) = anti_pos {
                                marked.insert(anti_pos);
                            }
                        }
                    }
                    else {
                        for anti_pos in get_position(grid, pos_2, pos_1) {
                            if let Some(anti_pos) = anti_pos {
                                marked.insert(anti_pos);
                            }
                        }
                    }
                }
            }
        }
    }
}

fn get_position<'a>(grid: &'a [Vec<u8>], pos_top: &'a (usize, usize), pos_bot: &'a (usize, usize)) -> Vec<Option<(usize, usize)>> {
    let diff = (pos_top.0.abs_diff(pos_bot.0), pos_top.1.abs_diff(pos_bot.1));
    if !is_outside_grid(grid, pos_top, &diff) && !is_outside_grid(grid, pos_bot, &diff){
        return vec![Some((pos_top.0 - diff.0, pos_top.1 - diff.1)), Some((pos_bot.0 + diff.0, pos_bot.1 + diff.1))];
    }
    if !is_outside_grid(grid, pos_bot, &diff) {
        return vec![Some((pos_bot.0 + diff.0, pos_bot.1 + diff.1)), None];
    }
    if !is_outside_grid(grid, pos_top, &diff) {
        return vec![Some((pos_top.0 - diff.0, pos_top.1 - diff.1)), None];
    }
    vec![None, None]
}

fn is_outside_grid(grid: &[Vec<u8>], pos: &(usize, usize), diff: &(usize, usize)) -> bool {
    if (pos.0 as i32 - diff.0 as i32) < 0 || (pos.1 as i32 - diff.1 as i32) < 0 {
        return true;
    }
    if pos.0 + diff.0 > grid[0].len() || pos.1 + diff.1 > grid.len() {
        return true;
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
