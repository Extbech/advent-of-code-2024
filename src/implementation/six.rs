use std::collections::HashSet;

use crate::Solution;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

const OBSTACLE: char = '#';

pub struct DaySixSolution {
    data: Vec<Vec<char>>,
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
        let (mut x, mut y, mut dir) = find_guard_pos(&self.data);
        loop {
            if is_at_bounds(x, y, self.data[0].len() - 1, self.data.len() - 1) {
                break;
            }
            match dir {
                Direction::Up => {
                    if self.data[y - 1][x] != OBSTACLE {
                        if obstacle_placement_leads_to_loop(&self.data, x, y, Direction::Up)
                            && (find_guard_pos(&self.data).0, find_guard_pos(&self.data).1)
                                != (x, y - 1)
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
                            && (find_guard_pos(&self.data).0, find_guard_pos(&self.data).1)
                                != (x, y + 1)
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
                        if !valid_squares.contains(&(x - 1, y)) {
                            if obstacle_placement_leads_to_loop(&self.data, x, y, Direction::Left)
                            && (find_guard_pos(&self.data).0, find_guard_pos(&self.data).1)
                            != (x - 1, y)
                            {
                                valid_squares.insert((x - 1, y));
                            };
                        }
                        x -= 1;
                    } else {
                        dir = Direction::Up;
                    }
                }
                Direction::Right => {
                    if self.data[y][x + 1] != OBSTACLE {
                        if obstacle_placement_leads_to_loop(&self.data, x, y, Direction::Right)
                            && (find_guard_pos(&self.data).0, find_guard_pos(&self.data).1)
                                != (x + 1, y)
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
        // visualize_grid(self.data.clone(), &valid_squares);
        valid_squares.len()
    }
}

fn parse_input(input: Vec<String>) -> Vec<Vec<char>> {
    input.iter().map(|x| x.chars().collect()).collect()
}

fn find_guard_pos(grid: &[Vec<char>]) -> (usize, usize, Direction) {
    grid.iter()
        .enumerate()
        .find_map(|(y, row)| {
            row.iter().enumerate().find_map(|(x, &cell)| match cell {
                '^' => Some((x, y, Direction::Up)),
                'v' => Some((x, y, Direction::Down)),
                '<' => Some((x, y, Direction::Left)),
                '>' => Some((x, y, Direction::Right)),
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
    grid: &[Vec<char>],
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
    let mut hits = 0;
    let mut dir = match initial_dir {
        Direction::Up => Direction::Right,
        Direction::Down => Direction::Left,
        Direction::Left => Direction::Up,
        Direction::Right => Direction::Down,
    };
    let mut visitied_squares: HashSet<(usize, usize, Direction)> = HashSet::new();
    visitied_squares.insert((x.clone(), y.clone(), initial_dir));
    // It will be a loop if we return to the same coords after (?) iterations. i.e a square.
    loop {
        if is_at_bounds(x, y, grid[0].len() - 1, grid.len() - 1) {
            return false;
        }
        match dir {
            Direction::Up => {
                if grid[y - 1][x] != OBSTACLE && ((x, y - 1) != temp_block_pos){
                    y -= 1;
                } else {
                    dir = Direction::Right;
                    hits += 1;
                }
            }
            Direction::Down => {
                if grid[y + 1][x] != OBSTACLE && ((x, y + 1) != temp_block_pos) {
                    y += 1;
                } else {
                    dir = Direction::Left;
                    hits += 1;
                }
            }
            Direction::Left => {
                if grid[y][x - 1] != OBSTACLE && ((x - 1, y) != temp_block_pos) {
                    x -= 1;
                } else {
                    dir = Direction::Up;
                    hits += 1;
                }
            }
            Direction::Right => {
                if grid[y][x + 1] != OBSTACLE && ((x + 1, y) != temp_block_pos) {
                    x += 1;
                } else {
                    dir = Direction::Down;
                    hits += 1;
                }
            }
        }
        if visitied_squares.contains(&(x, y, dir)) && hits > 2 {
            return true;
        }
        visitied_squares.insert((x, y, dir));
    }
}

fn visualize_grid(mut grid: Vec<Vec<char>>, valid: &HashSet<(usize, usize)>) {
    for (index, row) in grid.iter_mut().enumerate() {
        let mut iter = valid.iter().filter(|(_x, y)| y == &index);
        while let Some(v) = iter.next() {
            row[v.0] = 'O';
        }
    }
    for row in &grid {
        println!("{}", row.iter().collect::<String>());
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
