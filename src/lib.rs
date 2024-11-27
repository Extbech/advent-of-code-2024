use std::{fs, io::Error};

/// The Solution trait is used to define the interface for each day's solution.
pub trait Solution<T: std::fmt::Display, G: std::fmt::Display> {
    /// Implement this function to create a new instance of the solution.
    fn new() -> Self;

    /// This function is used to read the data from the file and return it as a string.
    fn read_data_to_string(day: u8) -> Result<String, Error> {
        let path = format!("data/day_{}.txt", day);
        fs::read_to_string(path)
    }
    
    /// This function is used to read the data from the file and return it as a vector of strings.
    fn read_data_to_vec(day: u8) -> Result<Vec<String>, Error> {
        let path = format!("data/day_{}.txt", day);
        fs::read_to_string(path)
            .map(|data| data.lines().map(|line| line.to_string()).collect())
    }

    /// Implement this function to solve part one of the problem.
    fn part_one(&self) -> T;
    
    /// Implement this function to solve part two of the problem.
    fn part_two(&self) -> G;

    /// Implement this function to solve both parts of the problem and print the results.
    fn solve(&self) { println!("Part one: {}\nPart two: {}\n", self.part_one(), self.part_two()); }
}

/// Solution for Day One
pub struct DayOne {
    data: Vec<String>,
}

impl Solution<i32, i32> for DayOne {
    fn new() -> Self {
        DayOne { data: Self::read_data_to_vec(1).unwrap() }
    }
    fn part_one(&self) -> i32 {
        self.data.iter().map(|x| x.parse::<i32>().unwrap()).filter(|f| f % 2 == 0).sum()
    }

    fn part_two(&self) -> i32 {
        self.data.iter().map(|x| x.parse::<i32>().unwrap()).filter(|f| f % 2 != 0).sum()
    }
}

/// Solution for Day Two
pub struct DayTwo {
    data: Vec<String>,
}

/// Solution for Day Three
pub struct DayThree {
    data: Vec<String>,
}

/// Solution for Day Four
pub struct DayFour {
    data: Vec<String>,
}
