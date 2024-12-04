use crate::Solution;

const PATTERN: &str = "XMAS";
const PATTERN_TWO: &str = "MAS";

pub struct DayFourSolution {
    data: Vec<Vec<char>>,
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
                if self.data[y][x] == 'X' {
                    let row = &self.data[y];
                    let col = &self.data.iter().map(|a| a[x]).collect::<Vec<char>>();
                    count += check_x_direction(row, x);
                    count += check_y_direction(col, y);
                    count += check_diag(&self.data, x, y)
                }
            }
        }
        count
    }

    fn part_two(&self) -> u16 {
        let mut count = 0;
        for y in 0..self.data.len() {
            for x in 0..self.data[y].len() {
                if self.data[y][x] == 'A' && check_diag_two(&self.data, x, y) {
                    count += 1;
                }
            }
        }
        count
    }
}

fn parse_input(input: Vec<String>) -> Vec<Vec<char>> {
    input.iter().map(|line| line.chars().collect()).collect()
}

fn check_x_direction(row: &[char], x: usize) -> u16 {
    let mut count = 0;
    if x as i16 - PATTERN.len() as i16 + 1 >= 0
        && (1..PATTERN.len()).all(|i| row[x - i] == PATTERN.chars().nth(i).unwrap())
    {
        count += 1;
    }
    if x + PATTERN.len() - 1 < row.len()
        && (1..PATTERN.len()).all(|i| row[x + i] == PATTERN.chars().nth(i).unwrap())
    {
        count += 1;
    }
    count
}

fn check_y_direction(col: &[char], y: usize) -> u16 {
    let mut count = 0;
    if y as i16 - PATTERN.len() as i16 + 1 >= 0
        && (1..PATTERN.len()).all(|i| col[y - i] == PATTERN.chars().nth(i).unwrap())
    {
        count += 1;
    }
    if y + PATTERN.chars().count() - 1 < col.len()
        && (1..PATTERN.len()).all(|i| col[y + i] == PATTERN.chars().nth(i).unwrap())
    {
        count += 1;
    }
    count
}

fn check_diag(grid: &[Vec<char>], x: usize, y: usize) -> u16 {
    let mut count = 0;
    if y as i16 - PATTERN.len() as i16 + 1 >= 0 {
        // TOP LEFT
        if x as i16 - PATTERN.len() as i16 + 1 >= 0
            && (1..PATTERN.len()).all(|i| grid[y - i][x - i] == PATTERN.chars().nth(i).unwrap())
        {
            count += 1;
        }
        // TOP RIGHT
        if x + PATTERN.len() - 1 < grid[y].len()
            && (1..PATTERN.len()).all(|i| grid[y - i][x + i] == PATTERN.chars().nth(i).unwrap())
        {
            count += 1;
        }
    }
    if y + PATTERN.len() - 1 < grid.len() {
        // BOTTOM LEFT
        if x as i16 - PATTERN.len() as i16 + 1 >= 0
            && (1..PATTERN.len()).all(|i| grid[y + i][x - i] == PATTERN.chars().nth(i).unwrap())
        {
            count += 1;
        }
        // BOTTOM RIGHT
        if x + PATTERN.len() - 1 < grid[y].len()
            && (1..PATTERN.len()).all(|i| grid[y + i][x + i] == PATTERN.chars().nth(i).unwrap())
        {
            count += 1;
        }
    }
    count
}

fn check_diag_two(grid: &[Vec<char>], x: usize, y: usize) -> bool {
    if x as i16 > 0 && x + 1 < grid[y].len() && y as i16 > 0 && y + 1 < grid.len() {
        return check_mas(vec![grid[y - 1][x - 1], grid[y][x], grid[y + 1][x + 1]])
            && check_mas(vec![grid[y - 1][x + 1], grid[y][x], grid[y + 1][x - 1]]);
    }
    false
}
fn check_mas(line: Vec<char>) -> bool {
    (0..PATTERN_TWO.len()).all(|i| line[i] == PATTERN_TWO.chars().nth(i).unwrap())
        || (0..PATTERN_TWO.len())
            .all(|i| line[line.len() - 1 - i] == PATTERN_TWO.chars().nth(i).unwrap())
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
        assert!(check_mas(vec!['M', 'A', 'S']));
        assert!(check_mas(vec!['S', 'A', 'M']));
    }
}
