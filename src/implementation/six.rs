use std::collections::HashSet;

use crate::Solution;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
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
        loop {
            visited_squares.insert((x, y));
            if is_at_bounds(x, y, self.data[0].len() - 1, self.data.len() - 1) {
                break;
            }
            match dir {
                Direction::Up => {
                    if self.data[y - 1][x] != OBSTACLE {
                        y -= 1;
                    } else {
                        dir = Direction::Right;
                    }
                }
                Direction::Down => {
                    if self.data[y + 1][x] != OBSTACLE {
                        y += 1;
                    } else {
                        dir = Direction::Left;
                    }
                }
                Direction::Left => {
                    if self.data[y][x - 1] != OBSTACLE {
                        x -= 1;
                    } else {
                        dir = Direction::Up;
                    }
                }
                Direction::Right => {
                    if self.data[y][x + 1] != OBSTACLE {
                        x += 1;
                    } else {
                        dir = Direction::Down;
                    }
                }
            }
        }
        visited_squares.len()
    }

    /// TLDR; Find squares...
    fn part_two(&self) -> usize {
        let mut valid_squares: HashSet<(usize, usize)> = HashSet::new();
        let mut visisted_squares: HashSet<(usize, usize)> = HashSet::new();
        let (mut x, mut y, mut dir) = find_guard_pos(&self.data);
        loop {
            if is_at_bounds(x, y, self.data[0].len() - 1, self.data.len() - 1) {
                break;
            }
            visisted_squares.insert((x, y));
            match dir {
                Direction::Up => {
                    if self.data[y - 1][x] != OBSTACLE {
                        if obstacle_placement_leads_to_loop(&self.data, x, y, Direction::Up)
                            && !visisted_squares.contains(&(x, y - 1))
                        {
                            valid_squares.insert((x, y - 1));
                        };
                        y -= 1;
                    } else {
                        dir = Direction::Right;
                    }
                }
                Direction::Down => {
                    if self.data[y + 1][x] != OBSTACLE {
                        if obstacle_placement_leads_to_loop(&self.data, x, y, Direction::Down)
                            && !visisted_squares.contains(&(x, y + 1))
                        {
                            valid_squares.insert((x, y + 1));
                        };
                        y += 1;
                    } else {
                        dir = Direction::Left;
                    }
                }
                Direction::Left => {
                    if self.data[y][x - 1] != OBSTACLE {
                        if obstacle_placement_leads_to_loop(&self.data, x, y, Direction::Left)
                            && !visisted_squares.contains(&(x - 1, y))
                        {
                            valid_squares.insert((x - 1, y));
                        };

                        x -= 1;
                    } else {
                        dir = Direction::Up;
                    }
                }
                Direction::Right => {
                    if self.data[y][x + 1] != OBSTACLE {
                        if obstacle_placement_leads_to_loop(&self.data, x, y, Direction::Right)
                            && !visisted_squares.contains(&(x + 1, y))
                        {
                            valid_squares.insert((x + 1, y));
                        };

                        x += 1;
                    } else {
                        dir = Direction::Down;
                    }
                }
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

/// This will be a bottleneck for the performance of part 2 as it will be called for each square we do not encounter an already existing obstacle
/// so optimizing this will lead to performance gains :)
fn obstacle_placement_leads_to_loop(
    grid: &[Vec<u8>],
    mut x: usize,
    mut y: usize,
    initial_dir: Direction,
) -> bool {
    let temp_block_pos = match initial_dir {
        Direction::Up => (x, y - 1),
        Direction::Down => (x, y + 1),
        Direction::Left => (x - 1, y),
        Direction::Right => (x + 1, y),
    };
    let mut dir = match initial_dir {
        Direction::Up => Direction::Right,
        Direction::Down => Direction::Left,
        Direction::Left => Direction::Up,
        Direction::Right => Direction::Down,
    };
    let mut visitied_squares: HashSet<(usize, usize, Direction)> = HashSet::new();
    visitied_squares.insert((x, y, initial_dir));

    loop {
        if is_at_bounds(x, y, grid[0].len() - 1, grid.len() - 1) {
            return false;
        }
        match dir {
            Direction::Up => {
                if grid[y - 1][x] != OBSTACLE && ((x, y - 1) != temp_block_pos) {
                    y -= 1;
                } else {
                    dir = Direction::Right;
                }
            }
            Direction::Down => {
                if grid[y + 1][x] != OBSTACLE && ((x, y + 1) != temp_block_pos) {
                    y += 1;
                } else {
                    dir = Direction::Left;
                }
            }
            Direction::Left => {
                if grid[y][x - 1] != OBSTACLE && ((x - 1, y) != temp_block_pos) {
                    x -= 1;
                } else {
                    dir = Direction::Up;
                }
            }
            Direction::Right => {
                if grid[y][x + 1] != OBSTACLE && ((x + 1, y) != temp_block_pos) {
                    x += 1;
                } else {
                    dir = Direction::Down;
                }
            }
        }
        if visitied_squares.contains(&(x, y, dir)) {
            return true;
        }
        visitied_squares.insert((x, y, dir));
    }
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
        assert_eq!(7, output);
    }
}
