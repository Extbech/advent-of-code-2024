use std::collections::HashSet;

use crate::Solution;

pub struct DayTwelveSolution {
    grid: Vec<Vec<u8>>,
}

impl Solution for DayTwelveSolution {
    const DAY: u8 = 12;

    fn new() -> Self {
        DayTwelveSolution {
            grid: parse_input(Self::read_data_to_vec().unwrap()),
        }
    }

    fn part_one(&self) -> u64 {
        let mut vec: Vec<Vec<bool>> = vec![vec![false; self.grid[0].len()]; self.grid.len()];
        let mut sum = 0;
        for y in 0..self.grid.len() {
            for x in 0..self.grid[y].len() {
                if !vec[y][x] {
                    let (area, perimeter) = iter_grid_recurse(&self.grid, (x, y), 0, 0, &mut vec);
                    sum += area * perimeter;
                }
            }
        }
        sum
    }

    fn part_two(&self) -> u64 {
        let mut vec: Vec<Vec<bool>> = vec![vec![false; self.grid[0].len()]; self.grid.len()];
        let mut perimeter_set: HashSet<(usize, usize, Direction)> = HashSet::new();
        let mut sum = 0;
        for y in 0..self.grid.len() {
            for x in 0..self.grid[y].len() {
                if !vec[y][x] {
                    let (area, perimeter) =
                        iter_grid_recurse_2(&self.grid, (x, y), 0, 0, &mut vec, &mut perimeter_set);
                    sum += area * perimeter;
                    perimeter_set.clear()
                }
            }
        }
        sum
    }
}

fn parse_input(input: Vec<String>) -> Vec<Vec<u8>> {
    input.iter().map(|x| x.as_bytes().to_vec()).collect()
}

fn iter_grid_recurse(
    grid: &Vec<Vec<u8>>,
    cords: (usize, usize),
    mut area: u64,
    mut perimeter: u64,
    vec: &mut Vec<Vec<bool>>,
) -> (u64, u64) {
    vec[cords.1][cords.0] = true;
    area += 1;
    perimeter += get_perimeter(grid, cords);
    // check if we can recurse go left
    if cords.0 > 0
        && grid[cords.1][cords.0 - 1] == grid[cords.1][cords.0]
        && !vec[cords.1][cords.0 - 1]
    {
        (area, perimeter) = iter_grid_recurse(grid, (cords.0 - 1, cords.1), area, perimeter, vec);
    }
    // check if we can recurse go right
    if cords.0 < grid[cords.1].len() - 1
        && grid[cords.1][cords.0 + 1] == grid[cords.1][cords.0]
        && !vec[cords.1][cords.0 + 1]
    {
        (area, perimeter) = iter_grid_recurse(grid, (cords.0 + 1, cords.1), area, perimeter, vec);
    }
    // check if we can recurse go up
    if cords.1 > 0
        && grid[cords.1 - 1][cords.0] == grid[cords.1][cords.0]
        && !vec[cords.1 - 1][cords.0]
    {
        (area, perimeter) = iter_grid_recurse(grid, (cords.0, cords.1 - 1), area, perimeter, vec);
    }
    // check if we can recurse go down
    if cords.1 < grid.len() - 1
        && grid[cords.1 + 1][cords.0] == grid[cords.1][cords.0]
        && !vec[cords.1 + 1][cords.0]
    {
        (area, perimeter) = iter_grid_recurse(grid, (cords.0, cords.1 + 1), area, perimeter, vec);
    }

    (area, perimeter)
}

fn get_perimeter(grid: &[Vec<u8>], cords: (usize, usize)) -> u64 {
    let mut perimeter = 0;
    let x = cords.0;
    let y = cords.1;
    if x == 0 || grid[y][x - 1] != grid[y][x] {
        perimeter += 1;
    }
    if x == grid[y].len() - 1 || grid[y][x + 1] != grid[y][x] {
        perimeter += 1;
    }
    if y == 0 || grid[y - 1][x] != grid[y][x] {
        perimeter += 1;
    }
    if y == grid.len() - 1 || grid[y + 1][x] != grid[y][x] {
        perimeter += 1;
    }
    perimeter
}

fn get_perimeter_2(
    grid: &[Vec<u8>],
    cords: (usize, usize),
    perimeter_set: &mut HashSet<(usize, usize, Direction)>,
) -> u64 {
    let mut perimeter = 0;
    let x = cords.0;
    let y = cords.1;
    let id = grid[y][x];
    // LEFT
    if (x == 0 || grid[y][x - 1] != id) && !perimeter_set.contains(&(x, y, Direction::Left)) {
        perimeter += 1;
        for i in 1.. {
            if y + i <= grid.len() - 1
                && grid[y + i][x] == id
                && (x == 0 || grid[y + i][x - 1] != id)
            {
                perimeter_set.insert((x, y + i, Direction::Left));
            } else {
                break;
            }
        }
        for i in 1.. {
            if y >= i && grid[y - i][x] == id && (x == 0 || grid[y - i][x - 1] != id) {
                perimeter_set.insert((x, y - i, Direction::Left));
            } else {
                break;
            }
        }
    }
    // RIGHT
    if (x == grid[y].len() - 1 || grid[y][x + 1] != id)
        && !perimeter_set.contains(&(x, y, Direction::Right))
    {
        perimeter += 1;
        for i in 1.. {
            if y + i <= grid.len() - 1
                && grid[y + i][x] == id
                && (x == grid[y].len() - 1 || grid[y + i][x + 1] != id)
            {
                perimeter_set.insert((x, y + i, Direction::Right));
            } else {
                break;
            }
        }
        for i in 1.. {
            if y >= i
                && grid[y - i][x] == id
                && (x == grid[y].len() - 1 || grid[y - i][x + 1] != id)
            {
                perimeter_set.insert((x, y - i, Direction::Right));
            } else {
                break;
            }
        }
    }
    // UP
    if (y == 0 || grid[y - 1][x] != id) && !perimeter_set.contains(&(x, y, Direction::Up)) {
        perimeter += 1;
        for i in 1.. {
            if x >= i && grid[y][x - i] == id && (y == 0 || grid[y - 1][x - i] != id) {
                perimeter_set.insert((x - i, y, Direction::Up));
            } else {
                break;
            }
        }
        for i in 1.. {
            if x + i <= grid[y].len() - 1
                && grid[y][x + i] == id
                && (y == 0 || grid[y - 1][x + i] != id)
            {
                perimeter_set.insert((x + i, y, Direction::Up));
            } else {
                break;
            }
        }
    }
    // DOWN
    if (y == grid.len() - 1 || grid[y + 1][x] != id)
        && !perimeter_set.contains(&(x, y, Direction::Down))
    {
        perimeter += 1;
        for i in 1.. {
            if x >= i && grid[y][x - i] == id && (y == grid.len() - 1 || grid[y + 1][x - i] != id) {
                perimeter_set.insert((x - i, y, Direction::Down));
            } else {
                break;
            }
        }
        for i in 1.. {
            if x + i <= grid[y].len() - 1
                && grid[y][x + i] == id
                && (y == grid.len() - 1 || grid[y + 1][x + i] != id)
            {
                perimeter_set.insert((x + i, y, Direction::Down));
            } else {
                break;
            }
        }
    }
    perimeter
}

#[derive(PartialEq, Clone, Copy, Hash, Eq)]
enum Direction {
    Left,
    Right,
    Up,
    Down,
}

fn iter_grid_recurse_2(
    grid: &Vec<Vec<u8>>,
    cords: (usize, usize),
    mut area: u64,
    mut perimeter: u64,
    vec: &mut Vec<Vec<bool>>,
    perimeter_set: &mut HashSet<(usize, usize, Direction)>,
) -> (u64, u64) {
    vec[cords.1][cords.0] = true;
    area += 1;
    perimeter += get_perimeter_2(grid, cords, perimeter_set);
    // check if we can recurse go left
    if cords.0 > 0
        && grid[cords.1][cords.0 - 1] == grid[cords.1][cords.0]
        && !vec[cords.1][cords.0 - 1]
    {
        (area, perimeter) = iter_grid_recurse_2(
            grid,
            (cords.0 - 1, cords.1),
            area,
            perimeter,
            vec,
            perimeter_set,
        );
    }
    // check if we can recurse go right
    if cords.0 < grid[cords.1].len() - 1
        && grid[cords.1][cords.0 + 1] == grid[cords.1][cords.0]
        && !vec[cords.1][cords.0 + 1]
    {
        (area, perimeter) = iter_grid_recurse_2(
            grid,
            (cords.0 + 1, cords.1),
            area,
            perimeter,
            vec,
            perimeter_set,
        );
    }
    // check if we can recurse go up
    if cords.1 > 0
        && grid[cords.1 - 1][cords.0] == grid[cords.1][cords.0]
        && !vec[cords.1 - 1][cords.0]
    {
        (area, perimeter) = iter_grid_recurse_2(
            grid,
            (cords.0, cords.1 - 1),
            area,
            perimeter,
            vec,
            perimeter_set,
        );
    }
    // check if we can recurse go down
    if cords.1 < grid.len() - 1
        && grid[cords.1 + 1][cords.0] == grid[cords.1][cords.0]
        && !vec[cords.1 + 1][cords.0]
    {
        (area, perimeter) = iter_grid_recurse_2(
            grid,
            (cords.0, cords.1 + 1),
            area,
            perimeter,
            vec,
            perimeter_set,
        );
    }

    (area, perimeter)
}
#[cfg(test)]

mod tests {
    use super::*;

    fn get_day_twelve() -> DayTwelveSolution {
        DayTwelveSolution {
            grid: parse_input(vec![
                "RRRRIICCFF".to_string(),
                "RRRRIICCCF".to_string(),
                "VVRRRCCFFF".to_string(),
                "VVRCCCJFFF".to_string(),
                "VVVVCJJCFE".to_string(),
                "VVIVCCJJEE".to_string(),
                "VVIIICJJEE".to_string(),
                "MIIIIIJJEE".to_string(),
                "MIIISIJEEE".to_string(),
                "MMMISSJEEE".to_string(),
            ]),
        }
    }
    #[test]
    fn test_part_one() {
        let day_twelve = get_day_twelve();
        assert_eq!(day_twelve.part_one(), 1930);
    }

    #[test]
    fn test_part_two() {
        let day_twelve = get_day_twelve();
        assert_eq!(day_twelve.part_two(), 1206);
    }
}
