#![allow(refining_impl_trait)]
use std::{fs, io::Error};

/// The Solution trait is used to define the interface for each day's solution.
pub trait Solution {
    /// Implement this function to return the day number.
    const DAY: u8;

    /// Implement this function to create a new instance of the solution.
    fn new() -> Self;

    /// This function is used to read the data from the file and return it as a string.
    fn read_data_to_string() -> Result<String, Error> {
        let path = format!("data/day_{}.txt", Self::DAY);
        fs::read_to_string(path)
    }

    /// This function is used to read the data from the file and return it as a vector of strings.
    fn read_data_to_vec() -> Result<Vec<String>, Error> {
        let path = format!("data/day_{}.txt", Self::DAY);
        fs::read_to_string(path).map(|data| data.lines().map(|line| line.to_string()).collect())
    }

    /// Implement this function to solve part one of the problem.
    fn part_one(&self) -> impl std::fmt::Display;

    /// Implement this function to solve part two of the problem.
    fn part_two(&self) -> impl std::fmt::Display;

    /// This function runs part one and two and prints the result for the day.
    fn solve(&self) {
        println!(
            "Day {}\nPart one: {}\nPart two: {}\n",
            Self::DAY,
            self.part_one(),
            self.part_two()
        );
    }
}

/// Solution for Day One
pub struct DayOneSolution {
    data: Vec<String>,
}

impl Solution for DayOneSolution {
    const DAY: u8 = 1;

    fn new() -> Self {
        DayOneSolution {
            data: Self::read_data_to_vec().unwrap(),
        }
    }

    fn part_one(&self) -> i32 {
        self.data
            .iter()
            .map(|x| x.parse::<i32>().unwrap())
            .filter(|f| f % 2 == 0)
            .sum::<i32>()
    }

    fn part_two(&self) -> i32 {
        self.data
            .iter()
            .map(|x| x.parse::<i32>().unwrap())
            .filter(|f| f % 2 != 0)
            .sum::<i32>()
    }
}

pub struct DayTwoSolution {
    data: Vec<String>,
}

impl Solution for DayTwoSolution {
    const DAY: u8 = 2;

    fn new() -> Self {
        DayTwoSolution {
            data: Self::read_data_to_vec().unwrap(),
        }
    }

    fn part_one(&self) -> i32 {
        self.data
            .iter()
            .map(|x| x.parse::<i32>().unwrap())
            .filter(|f| f % 2 == 0)
            .sum::<i32>()
    }

    fn part_two(&self) -> i32 {
        self.data
            .iter()
            .map(|x| x.parse::<i32>().unwrap())
            .filter(|f| f % 2 != 0)
            .sum::<i32>()
    }
}

pub struct AdventOfCodeSolver<'a> {
    solutions: &'a [fn() -> ()],
}

impl<'a> AdventOfCodeSolver<'a> {
    pub fn new() -> Self {
        AdventOfCodeSolver {
            solutions: &[
                || DayOneSolution::new().solve(),
                || DayTwoSolution::new().solve(),
            ],
        }
    }

    pub fn solve(&self, args: Vec<String>) {
        match args.len() {
            1 => self.solutions.iter().for_each(|s| s()),
            2 => match args[1].as_str() {
                "1" | "one" => self.solutions[0](),
                "2" | "two" => self.solutions[1](),
                "3" | "three" => unimplemented!(),
                _x => println!("Day <{_x}> not found or implemented"),
            },
            _ => println!("Invalid Input. Too many number of arguments"),
        }
    }
}
