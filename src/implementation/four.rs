use crate::Solution;

pub struct DayFourSolution {
    data: Vec<String>,
}

impl Solution for DayFourSolution {
    const DAY: u8 = 4;

    fn new() -> Self {
        DayFourSolution { Self::read_data_to_vec() }
    }

    fn part_one(&self) -> ! {
        todo!("Implement part_one function for DayFourSolution")
    }

    fn part_two(&self) -> ! {
        todo!("Implement part_two function for DayFourSolution")
    }
}
