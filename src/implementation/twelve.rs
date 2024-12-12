use std::collections::HashSet;

use crate::Solution;

pub struct DayTwelveSolution {
    grid: Vec<Vec<u8>>,
}

impl Solution for DayTwelveSolution {
    const DAY: u8 = 12;

    fn new() -> Self {
        DayTwelveSolution { grid: parse_input(Self::read_data_to_vec().unwrap()) }
    }

    fn part_one(&self) -> u64 {
        let mut set: HashSet<(usize, usize)> = HashSet::new();
        let mut sum = 0;
        for y in 0..self.grid.len() {
            for x in 0..self.grid[y].len() {
                if !set.contains(&(x, y)) {
                    let (area, perimeter) = iter_grid_recurse(&self.grid, (x, y), 0, 0, &mut set);
                    sum += area * perimeter;
                }
            } 
        }
        sum
    }

    fn part_two(&self) -> u64 {
        let mut set: HashSet<(usize, usize)> = HashSet::new();
        let mut sum = 0;
        for y in 0..self.grid.len() {
            for x in 0..self.grid[y].len() {
                if !set.contains(&(x, y)) {
                    let (area, perimeter) = iter_grid_recurse_2(&self.grid, (x, y), 0, 0, &mut set, None,Direction::Right);
                    println!("Char: {} Area: {}, Perimeter: {}",self.grid[y][x] as char, area, perimeter);
                    sum += area * perimeter;
                }
            }
        }
        sum
    }
}

fn parse_input(input: Vec<String>) -> Vec<Vec<u8>> {
    input.iter().map(|x| x.as_bytes().to_vec()).collect()
}

fn iter_grid_recurse(grid: &Vec<Vec<u8>>, cords: (usize, usize), mut area: u64, mut perimeter: u64, set: &mut HashSet<(usize, usize)>) -> (u64, u64) {
    // Break condition, no
    set.insert(cords);
    area += 1;
    perimeter += get_perimeter(grid, cords);
    // check if we can recurse go left
    if cords.0 > 0 && grid[cords.1][cords.0 - 1] == grid[cords.1][cords.0] && !set.contains(&(cords.0 - 1, cords.1)) {
        (area, perimeter) = iter_grid_recurse(grid, (cords.0 - 1, cords.1), area, perimeter, set);
    }
    // check if we can recurse go right
    if cords.0 < grid[cords.1].len() - 1 && grid[cords.1][cords.0 + 1] == grid[cords.1][cords.0] && !set.contains(&(cords.0 + 1, cords.1)) {
        (area, perimeter) = iter_grid_recurse(grid, (cords.0 + 1, cords.1), area, perimeter, set);
    }
    // check if we can recurse go up
    if cords.1 > 0 && grid[cords.1 - 1][cords.0] == grid[cords.1][cords.0] && !set.contains(&(cords.0, cords.1 - 1)) {
        (area, perimeter) = iter_grid_recurse(grid, (cords.0, cords.1 - 1), area, perimeter, set);
    }
    // check if we can recurse go down
    if cords.1 < grid.len() - 1 && grid[cords.1 + 1][cords.0] == grid[cords.1][cords.0] && !set.contains(&(cords.0, cords.1 + 1)) {
        (area, perimeter) = iter_grid_recurse(grid, (cords.0, cords.1 + 1), area, perimeter, set);
    }

    return (area, perimeter);
}

fn get_perimeter(grid: &Vec<Vec<u8>>, cords: (usize, usize)) -> u64 {
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

#[derive(PartialEq, Clone, Copy)]
enum Direction {
    Left,
    Right,
    Up,
    Down,
}

fn iter_grid_recurse_2(grid: &Vec<Vec<u8>>, cords: (usize, usize), mut area: u64, mut perimeter: u64, set: &mut HashSet<(usize, usize)>, prev_dir: Option<Direction>, dir: Direction) -> (u64, u64) {
    // Break condition, no
    set.insert(cords);
    area += 1;
    if prev_dir.is_some(){
        if prev_dir.unwrap() != dir {
            perimeter += get_perimeter(grid, cords);
        }
    } else {
        perimeter += get_perimeter(grid, cords);
    }
    // check if we can recurse go left
    if cords.0 > 0 && grid[cords.1][cords.0 - 1] == grid[cords.1][cords.0] && !set.contains(&(cords.0 - 1, cords.1)) {
        (area, perimeter) = iter_grid_recurse_2(grid, (cords.0 - 1, cords.1), area, perimeter, set, Some(dir),Direction::Left);
    }
    // check if we can recurse go right
    if cords.0 < grid[cords.1].len() - 1 && grid[cords.1][cords.0 + 1] == grid[cords.1][cords.0] && !set.contains(&(cords.0 + 1, cords.1)) {
        (area, perimeter) = iter_grid_recurse_2(grid, (cords.0 + 1, cords.1), area, perimeter, set, Some(dir),Direction::Right);
    }
    // check if we can recurse go up
    if cords.1 > 0 && grid[cords.1 - 1][cords.0] == grid[cords.1][cords.0] && !set.contains(&(cords.0, cords.1 - 1)) {
        (area, perimeter) = iter_grid_recurse_2(grid, (cords.0, cords.1 - 1), area, perimeter, set, Some(dir),Direction::Up);
    }
    // check if we can recurse go down
    if cords.1 < grid.len() - 1 && grid[cords.1 + 1][cords.0] == grid[cords.1][cords.0] && !set.contains(&(cords.0, cords.1 + 1)) {
        (area, perimeter) = iter_grid_recurse_2(grid, (cords.0, cords.1 + 1), area, perimeter, set, Some(dir),Direction::Down);
    }

    return (area, perimeter);
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
                "MMMISSJEEE".to_string()
                ])
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