use crate::Solution;

pub struct DayEightSolution {
    grid: Vec<Vec<u8>>,
}

impl Solution for DayEightSolution {
    const DAY: u8 = 8;

    fn new() -> Self {
        let data = parse_input(Self::read_data_to_vec().unwrap());
        DayEightSolution { grid: data }
    }

    fn part_one(&self) -> usize {
        let mut grid = self.grid.clone();
        let il = grid.len();
        let jl = grid[0].len();
        let mut an_grid = vec![vec![b'.'; jl]; il];
        for i in 0..il {
            for j in 0..jl {
                if [b'.', b'#'].contains(&grid[i][j]) {
                    continue;
                }
                for i2 in 0..il {
                    for j2 in 0..jl {
                        if i2 == i && j2 == j {
                            continue;
                        }
                        mark_antinode(&mut an_grid, &mut grid, i, j, i2, j2);
                        mark_antinode(&mut an_grid, &mut grid, i2, j2, i, j);
                    }
                }
            }
        }
        for row in an_grid
            .iter()
            .map(|row| row.iter().map(|&e| e as char).collect::<String>())
        {
            println!("{row}");
        }
        an_grid
            .into_iter()
            .map(|row| row.into_iter().filter(|&e| e == b'#').count())
            .sum()
    }

    fn part_two(&self) -> u32 {
        32
    }
}

// Marks the antinode where (i2, j2) is between (i1, j1) and the antinode
fn mark_antinode(
    an_grid: &mut [Vec<u8>],
    grid: &mut [Vec<u8>],
    i1: usize,
    j1: usize,
    i2: usize,
    j2: usize,
) {
    let il = grid.len();
    let jl = grid[0].len();
    if grid[i1][j1] == grid[i2][j2] {
        let Some(i_an) = (2 * i2).checked_sub(i1) else {
            return;
        };
        let Some(j_an) = (2 * j2).checked_sub(j1) else {
            return;
        };
        if i_an < il && j_an < jl {
            an_grid[i_an][j_an] = b'#'
        }
    }
}

fn parse_input(input: Vec<String>) -> Vec<Vec<u8>> {
    input.iter().map(|s| s.as_bytes().to_owned()).collect()
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
        DayEightSolution { grid: data }
    }
    #[test]
    fn test_part_one() {
        let day_eight = get_day_eight();
        let output = day_eight.part_one();
        assert_eq!(output, 14);
    }
}
