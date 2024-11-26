use std::{fs, io::Error};

/// The Solution trait is used to define the interface for each day's solution.
pub trait Solution<T, G> {
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
    fn part_one(&self) -> T {
        unimplemented!()
    }
    
    /// Implement this function to solve part two of the problem.
    fn part_two(&self) -> G {
        unimplemented!()
    }

    /// Implement this function to solve both parts of the problem and print the results.
    fn solve(&self) {
        unimplemented!()
    }
}

pub struct DayOne {
    day: u8,
    data: Vec<String>,
}

impl Solution<i32, i32> for DayOne {
    fn new() -> Self {
        DayOne {
            day: 1,
            data: Self::read_data_to_vec(1).unwrap(),
        }
    }
    fn part_one(&self) -> i32 {
        self.data.iter().map(|x| x.parse::<i32>().unwrap()).filter(|f| f % 2 == 0).sum()
    }

    fn part_two(&self) -> i32 {
        self.data.iter().map(|x| x.parse::<i32>().unwrap()).filter(|f| f % 2 != 0).sum()
    }

    fn solve(&self) {
        let part_one = self.part_one();
        let part_two = self.part_two();
        println!("Day {}\nSolution part one: {}\nSolution part two: {}\n", self.day, part_one, part_two);
    }
}