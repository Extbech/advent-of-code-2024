use crate::Solution;

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

    fn part_one(&self) -> ! {
        todo!()
    }

    fn part_two(&self) -> ! {
        todo!()
    }
}