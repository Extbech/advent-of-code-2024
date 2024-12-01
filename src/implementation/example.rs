use crate::Solution;

/// Solution for Day One
pub struct ExampleSolution {
    data: Vec<String>,
}

impl Solution for ExampleSolution {
    // This const does not matter as it is not used for this example implementation.
    const DAY: u8 = 0;

    // This const is used to determine if the solution is an example solution.
    const EXAMPLE: bool = true;

    // Init the solution struct with the data from the file.
    fn new() -> Self {
        ExampleSolution {
            data: Self::read_data_to_vec().unwrap(),
        }
    }
    // Implementation for part one of the problem.
    // This implementation will sum all the even numbers in the data.
    fn part_one(&self) -> i32 {
        self.data
            .iter()
            .map(|x| x.parse::<i32>().unwrap())
            .filter(|f| f % 2 == 0)
            .sum::<i32>()
    }

    // Implementation for part two of the problem.
    // This implementation will sum all the odd numbers in the data.
    fn part_two(&self) -> i32 {
        self.data
            .iter()
            .map(|x| x.parse::<i32>().unwrap())
            .filter(|f| f % 2 != 0)
            .sum::<i32>()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        assert_eq!(ExampleSolution::new().part_one(), 30);
    }

    #[test]
    fn test_part_two() {
        assert_eq!(ExampleSolution::new().part_two(), 25);
    }
}
