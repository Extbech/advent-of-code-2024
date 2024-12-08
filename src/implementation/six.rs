use std::{
    cmp::{max, min},
    collections::{HashMap, HashSet},
    hash::Hash,
};

use crate::Solution;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

const OBSTACLE: u8 = b'#';

pub struct DaySixSolution {
    data: Vec<Vec<u8>>,
}

impl Solution for DaySixSolution {
    const DAY: u8 = 6;

    fn new() -> Self {
        DaySixSolution {
            data: parse_input(Self::read_data_to_vec().unwrap()),
        }
    }

    fn part_one(&self) -> usize {
        let mut visited_squares: HashSet<(usize, usize)> = HashSet::new();
        let (mut x, mut y, mut dir) = find_guard_pos(&self.data);
        visited_squares.insert((x, y));
        while let Some((x1, y1, dir1)) = mov_one(&self.data, x, y, dir) {
            (x, y, dir) = (x1, y1, dir1);
            visited_squares.insert((x, y));
        }
        visited_squares.len()
    }

    /// TLDR; Find squares...
    fn part_two(&self) -> usize {
        let mut valid_squares: HashSet<(usize, usize)> = HashSet::new();
        let mut visited_squares: HashSet<(usize, usize)> = HashSet::new();
        let mut memo = HashMap::new();
        let (mut x, mut y, mut dir) = find_guard_pos(&self.data);
        visited_squares.insert((x, y));
        let mut visited_buf = HashSet::new();
        while let Some((x_next, y_next)) =
            free_step(x, y, self.data[0].len() - 1, self.data.len() - 1, dir)
        {
            if self.data[y_next][x_next] != OBSTACLE {
                if !visited_squares.contains(&(x_next, y_next))
                    && obstacle_placement_leads_to_loop(
                        &self.data,
                        x,
                        y,
                        (x_next, y_next),
                        next_dir(dir),
                        &mut memo,
                        &mut visited_buf,
                    )
                {
                    valid_squares.insert((x_next, y_next));
                };
                (x, y) = (x_next, y_next);
                visited_squares.insert((x, y));
            } else {
                dir = next_dir(dir);
            }
        }
        valid_squares.len()
    }
}

fn parse_input(input: Vec<String>) -> Vec<Vec<u8>> {
    input.into_iter().map(|x| x.into_bytes()).collect()
}

fn find_guard_pos(grid: &[Vec<u8>]) -> (usize, usize, Direction) {
    grid.iter()
        .enumerate()
        .find_map(|(y, row)| {
            row.iter().enumerate().find_map(|(x, &cell)| match cell {
                b'^' => Some((x, y, Direction::Up)),
                b'v' => Some((x, y, Direction::Down)),
                b'<' => Some((x, y, Direction::Left)),
                b'>' => Some((x, y, Direction::Right)),
                _ => None,
            })
        })
        .unwrap()
}

fn is_at_bounds(x: usize, y: usize, x_max: usize, y_max: usize) -> bool {
    x == 0 || y == 0 || x == x_max || y == y_max
}

fn next_dir(dir: Direction) -> Direction {
    match dir {
        Direction::Up => Direction::Right,
        Direction::Right => Direction::Down,
        Direction::Down => Direction::Left,
        Direction::Left => Direction::Up,
    }
}

fn free_step(
    x: usize,
    y: usize,
    x_max: usize,
    y_max: usize,
    dir: Direction,
) -> Option<(usize, usize)> {
    if is_at_bounds(x, y, x_max, y_max) {
        return None;
    }
    Some(match dir {
        Direction::Up => (x, y - 1),
        Direction::Right => (x + 1, y),
        Direction::Down => (x, y + 1),
        Direction::Left => (x - 1, y),
    })
}

fn mov_one(
    grid: &[Vec<u8>],
    x: usize,
    y: usize,
    dir: Direction,
) -> Option<(usize, usize, Direction)> {
    let (x_new, y_new) = free_step(x, y, grid[0].len() - 1, grid.len() - 1, dir)?;
    if grid[y_new][x_new] == OBSTACLE {
        Some((x, y, next_dir(dir)))
    } else {
        Some((x_new, y_new, dir))
    }
}

fn in_square(c1: (usize, usize), c2: (usize, usize), p: (usize, usize)) -> bool {
    let c_max = (max(c1.0, c2.0), max(c1.1, c2.1));
    let c_min = (min(c1.0, c2.0), min(c1.1, c2.1));
    c_min.0 <= p.0 && p.0 <= c_max.0 && c_min.1 <= p.1 && p.1 <= c_max.1
}

fn mov_till_turn(
    grid: &[Vec<u8>],
    x: usize,
    y: usize,
    dir: Direction,
    temp_block_pos: (usize, usize),
    memo: &mut HashMap<(usize, usize, Direction), (usize, usize)>,
) -> Option<(usize, usize, Direction)> {
    if is_at_bounds(x, y, grid[0].len() - 1, grid.len() - 1) {
        return None;
    }
    if let Some(&(x_new, y_new)) = memo.get(&(x, y, dir)) {
        if !in_square((x, y), (x_new, y_new), temp_block_pos) {
            return Some((x_new, y_new, next_dir(dir)));
        }
    }
    let (mut x_old, mut y_old) = (x, y);
    loop {
        let Some((x_new, y_new)) = free_step(x_old, y_old, grid[0].len() - 1, grid.len() - 1, dir)
        else {
            memo.insert((x, y, dir), (x_old, y_old));
            return None;
        };
        if grid[y_new][x_new] == OBSTACLE {
            memo.insert((x, y, dir), (x_old, y_old));
            return Some((x_old, y_old, next_dir(dir)));
        } else if (x_new, y_new) == temp_block_pos {
            return Some((x_old, y_old, next_dir(dir)));
        } else {
            (x_old, y_old) = (x_new, y_new)
        }
    }
}

/// This will be a bottleneck for the performance of part 2 as it will be called for each square we do not encounter an already existing obstacle
/// so optimizing this will lead to performance gains :)
fn obstacle_placement_leads_to_loop(
    grid: &[Vec<u8>],
    mut x: usize,
    mut y: usize,
    temp_block_pos: (usize, usize),
    mut dir: Direction,
    memo: &mut HashMap<(usize, usize, Direction), (usize, usize)>,
    visited_buf: &mut HashSet<(usize, usize, Direction)>,
) -> bool {
    visited_buf.clear();
    visited_buf.insert((x, y, dir));

    while let Some((x1, y1, dir1)) = mov_till_turn(grid, x, y, dir, temp_block_pos, memo) {
        (x, y, dir) = (x1, y1, dir1);
        if visited_buf.contains(&(x, y, dir)) {
            return true;
        }
        visited_buf.insert((x, y, dir));
    }
    false
}

fn visualize_grid(mut grid: Vec<Vec<u8>>, valid: &HashSet<(usize, usize)>) {
    for (index, row) in grid.iter_mut().enumerate() {
        let iter = valid.iter().filter(|(_x, y)| y == &index);
        for v in iter {
            row[v.0] = b'O';
        }
    }
    for row in &grid {
        println!("{:?}", row);
    }
}
#[cfg(test)]
mod tests {
    use std::fs;

    use super::*;
    fn get_day_six() -> DaySixSolution {
        DaySixSolution {
            data: parse_input(
                fs::read_to_string("data/test/test_6.txt")
                    .unwrap()
                    .lines()
                    .map(|x| x.to_string())
                    .collect(),
            ),
        }
    }
    #[test]
    fn test_part_one() {
        let day_six = get_day_six();
        let output = day_six.part_one();
        assert_eq!(output, 41);
    }

    #[test]
    fn test_part_two() {
        let day_six = get_day_six();
        let output = day_six.part_two();
        assert_eq!(6, output);
    }
}
