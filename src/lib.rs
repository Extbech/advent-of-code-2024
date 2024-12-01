#![allow(refining_impl_trait, dead_code)]
pub mod implementation;

use std::{fs, io::Error};
use crate::implementation::*;

/// The `Solution` trait defines the interface required for implementing the solution to a day's challenge in Advent of Code.
///
/// # Examples
///
/// ```
/// #[allow(refining_impl_trait)]
/// use advent_of_code_2024::Solution;
/// 
/// struct MyDaySolution {
///    data: Vec<String>,
/// }
///
/// impl Solution for MyDaySolution {
///     const DAY: u8 = 0;
///     const EXAMPLE: bool = true;
///
///     fn new() -> Self {
///         MyDaySolution {
///             data: Self::read_data_to_vec().unwrap(),        
///         }
///     }
///
///     fn part_one(&self) -> i32 {
///         self.data.iter().map(|x| x.parse::<i32>().unwrap()).sum()
///     }
///
///     fn part_two(&self) -> f32 {
///         self.data.iter().map(|x| x.parse::<f32>().unwrap()).sum()
///     }
/// }
///
/// let solution = MyDaySolution::new().solve();
/// ```
pub trait Solution {

    // Constant determining if the solution is to be used as example.
    const EXAMPLE: bool = false;

    /// Implement this function to return the day number.
    const DAY: u8;

    /// Implement this function to create a new instance of the solution.
    fn new() -> Self;

    // Function for getting file path based on AOC day and whether it is an example solution or not.
    fn get_file_path() -> String { if Self::EXAMPLE { "data/example.txt".to_string() } else { format!("data/day_{}.txt", Self::DAY) } }

    /// This function is used to read the data from the file and return it as a string.
    fn read_data_to_string() -> Result<String, Error> { fs::read_to_string(Self::get_file_path()) }

    /// This function is used to read the data from the file and return it as a vector of strings.
    fn read_data_to_vec() -> Result<Vec<String>, Error> { fs::read_to_string(Self::get_file_path()).map(|data| data.lines().map(|line| line.to_string()).collect()) }

    /// Implement this function to solve part one of the problem.
    fn part_one(&self) -> impl std::fmt::Display;

    /// Implement this function to solve part two of the problem.
    fn part_two(&self) -> impl std::fmt::Display;

    /// This function runs part one and two and prints the result for the day.
    fn solve(&self) {
        if Self::EXAMPLE {
            println!("\nExample Solution\nPart one solution: {}\nPart two solution: {}", self.part_one(), self.part_two());
        } else {
            println!(
                "\nDay {}\nPart one solution: {}\nPart two solution: {}",
                Self::DAY,
                self.part_one(),
                self.part_two()
            );
        }
    }
}

/// `AdventOfCodeSolver` is responsible for managing and executing solutions for each day.
///
/// # Examples
///
/// ```
/// use advent_of_code_2024::AdventOfCodeSolver;
/// 
/// let solver = AdventOfCodeSolver::new();
/// solver.solve(vec!["path_to_executable".to_string(), "ex".to_string()]);
/// // This will execute the example solution
/// ```
pub struct AdventOfCodeSolver<'a> {
    /// A slice of function pointers to the solutions for each day.
    solutions: &'a [fn() -> ()],
}

impl<'a> AdventOfCodeSolver<'a> {
    pub fn new() -> Self {
        AdventOfCodeSolver {
            solutions: &[
                || one::DayOneSolution::new().solve(),
                || two::DayTwoSolution::new().solve(),
                || three::DayThreeSolution::new().solve(),
                || four::DayFourSolution::new().solve(),
                || five::DayFiveSolution::new().solve(),
                || six::DaySixSolution::new().solve(),
                || seven::DaySevenSolution::new().solve(),
                || eight::DayEightSolution::new().solve(),
                || nine::DayNineSolution::new().solve(),
                || ten::DayTenSolution::new().solve(),
                || eleven::DayElevenSolution::new().solve(),
                || twelve::DayTwelveSolution::new().solve(),
                || thirteen::DayThirteenSolution::new().solve(),
                || fourteen::DayFourteenSolution::new().solve(),
                || fifteen::DayFifteenSolution::new().solve(),
                || sixteen::DaySixteenSolution::new().solve(),
                || seventeen::DaySeventeenSolution::new().solve(),
                || eighteen::DayEighteenSolution::new().solve(),
                || nineteen::DayNineteenSolution::new().solve(),
                || twenty::DayTwentySolution::new().solve(),
                || twenty_one::DayTwentyOneSolution::new().solve(),
                || twenty_two::DayTwentyTwoSolution::new().solve(),
                || twenty_three::DayTwentyThreeSolution::new().solve(),
                || twenty_four::DayTwentyFourSolution::new().solve(),
            ],
        }
    }

    /// This function is used to execute the solution for a specific day or all days depending on args provided.
    pub fn solve(&self, args: Vec<String>) {
        match args.len() {
            1 => {
                println!("\nExecuting all solutions...");
                self.solutions.iter().for_each(|s| s())
            },
            2 => match args[1].as_str() {
                "1" | "one" => self.solutions[0](),
                "2" | "two" => self.solutions[1](),
                "3" | "three" => self.solutions[2](),
                "4" | "four" => self.solutions[3](),
                "5" | "five" => self.solutions[4](),
                "6" | "six" => self.solutions[5](),
                "7" | "seven" => self.solutions[6](),
                "8" | "eight" => self.solutions[7](),
                "9" | "nine" => self.solutions[8](),
                "10" | "ten" => self.solutions[9](),
                "11" | "eleven" => self.solutions[10](),
                "12" | "twelve" => self.solutions[11](),
                "13" | "thirteen" => self.solutions[12](),
                "14" | "fourteen" => self.solutions[13](),
                "15" | "fifteen" => self.solutions[14](),
                "16" | "sixteen" => self.solutions[15](),
                "17" | "seventeen" => self.solutions[16](),
                "18" | "eighteen" => self.solutions[17](),
                "19" | "nineteen" => self.solutions[18](),
                "20" | "twenty" => self.solutions[19](),
                "21" | "twenty_one" => self.solutions[20](),
                "22" | "twenty_two" => self.solutions[21](),
                "23" | "twenty_three" => self.solutions[22](),
                "24" | "twenty_four" => self.solutions[23](),
                "ex" | "example" => example::ExampleSolution::new().solve(),
                _x => println!("Day <{_x}> not found or implemented"),
            },
            _ => println!("Invalid Input. Too many number of arguments"),
        }
    }
}
