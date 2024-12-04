use crate::Solution;

const PATTERN: &[u8] = b"XMAS";

pub struct DayFourSolution {
    data: Vec<Vec<u8>>,
}

impl Solution for DayFourSolution {
    const DAY: u8 = 4;

    fn new() -> Self {
        DayFourSolution {
            data: parse_input(Self::read_data_to_vec().unwrap()),
        }
    }

    fn part_one(&self) -> u16 {
        let mut count = 0;
        for y in 0..self.data.len() {
            for x in 0..self.data[y].len() {
                if self.data[y][x] == b'X' {
                    count += check_x_direction(&self.data[y], x);
                    count += check_y_direction(&self.data, y, x);
                    count += check_diag(&self.data, x, y);
                }
            }
        }
        count
    }

    fn part_two(&self) -> u16 {
        let mut count = 0;
        for y in 1..self.data.len() - 1 {
            for x in 1..self.data[y].len() - 1 {
                if self.data[y][x] == b'A' && check_diag_two(&self.data, x, y) {
                    count += 1;
                }
            }
        }
        count
    }
}

fn parse_input(input: Vec<String>) -> Vec<Vec<u8>> {
    input.into_iter().map(|line| line.into_bytes()).collect()
}

fn check_x_direction(row: &[u8], x: usize) -> u16 {
    let mut count = 0;
    // CHECK LEFT
    if x as i16 - PATTERN.len() as i16 + 1 >= 0
        && (1..PATTERN.len()).all(|i| row[x - i] == PATTERN[i])
    {
        count += 1;
    }
    // CHECK RIGHT
    if x + PATTERN.len() - 1 < row.len() && (1..PATTERN.len()).all(|i| row[x + i] == PATTERN[i]) {
        count += 1;
    }
    count
}

fn check_y_direction(grid: &[Vec<u8>], y: usize, x: usize) -> u16 {
    let mut count = 0;
    // CHECK UPWARDS
    if y as i16 - PATTERN.len() as i16 + 1 >= 0
        && (1..PATTERN.len()).all(|i| grid[y - i][x] == PATTERN[i])
    {
        count += 1;
    }
    // CHECK DOWNWARDS
    if y + PATTERN.len() - 1 < grid[y].len()
        && (1..PATTERN.len()).all(|i| grid[y + i][x] == PATTERN[i])
    {
        count += 1;
    }
    count
}

fn check_diag(grid: &[Vec<u8>], x: usize, y: usize) -> u16 {
    let mut count = 0;
    if y as i16 - PATTERN.len() as i16 + 1 >= 0 {
        // TOP LEFT
        if x as i16 - PATTERN.len() as i16 + 1 >= 0
            && (1..PATTERN.len()).all(|i| grid[y - i][x - i] == PATTERN[i])
        {
            count += 1;
        }
        // TOP RIGHT
        if x + PATTERN.len() - 1 < grid[y].len()
            && (1..PATTERN.len()).all(|i| grid[y - i][x + i] == PATTERN[i])
        {
            count += 1;
        }
    }
    if y + PATTERN.len() - 1 < grid.len() {
        // BOTTOM LEFT
        if x as i16 - PATTERN.len() as i16 + 1 >= 0
            && (1..PATTERN.len()).all(|i| grid[y + i][x - i] == PATTERN[i])
        {
            count += 1;
        }
        // BOTTOM RIGHT
        if x + PATTERN.len() - 1 < grid[y].len()
            && (1..PATTERN.len()).all(|i| grid[y + i][x + i] == PATTERN[i])
        {
            count += 1;
        }
    }
    count
}

fn check_diag_two(grid: &[Vec<u8>], x: usize, y: usize) -> bool {
    if x as i16 > 0 && x + 1 < grid[y].len() && y as i16 > 0 && y + 1 < grid.len() {
        return check_mas(grid[y - 1][x - 1], grid[y][x], grid[y + 1][x + 1])
            && check_mas(grid[y - 1][x + 1], grid[y][x], grid[y + 1][x - 1]);
    }
    false
}

fn check_mas(m: u8, a: u8, s: u8) -> bool {
    (m == b'M' && a == b'A' && s == b'S') || (m == b'S' && a == b'A' && s == b'M')
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_part_one() {
        let test_vec = parse_input(
            fs::read_to_string("data/test/test_4.txt")
                .map(|data| {
                    data.lines()
                        .map(|line| line.to_string())
                        .collect::<Vec<String>>()
                })
                .unwrap(),
        );

        let day_four = DayFourSolution { data: test_vec };
        let sol = day_four.part_one();

        assert_eq!(18, sol);
    }

    #[test]
    fn test_part_two() {
        let test_vec = parse_input(
            fs::read_to_string("data/test/test_4.txt")
                .map(|data| {
                    data.lines()
                        .map(|line| line.to_string())
                        .collect::<Vec<String>>()
                })
                .unwrap(),
        );

        let day_four = DayFourSolution { data: test_vec };
        let sol = day_four.part_two();

        assert_eq!(9, sol);
    }

    #[test]
    fn test_helper() {
        assert!(check_mas(b'M', b'A', b'S'));
        assert!(check_mas(b'S', b'A', b'M'));
    }
}
