use crate::Solution;

pub struct DayThirteenSolution {
    button_configs: Vec<(Button, Button)>,
    prizes: Vec<Prize>,
}

#[derive(Debug)]
struct Button {
    x: u32,
    y: u32,
}

struct Prize {
    x: u32,
    y: u32,
}

impl Solution for DayThirteenSolution {
    const DAY: u8 = 13;

    fn new() -> Self {
        let input = parse_input(Self::read_data_to_vec().unwrap());
        DayThirteenSolution {
            button_configs: input.0,
            prizes: input.1,
        }
    }

    fn part_one(&self) -> u32 {
        let mut count: u32 = 0;
        for (button_config, prize) in self.button_configs.iter().zip(self.prizes.iter()) {
            if let Some(v) = find_minimum_tokens(
                button_config.0.x,
                button_config.0.y,
                button_config.1.x,
                button_config.1.y,
                prize.x,
                prize.y,
            ) {
                count += v;
            }
        }
        count
    }

    fn part_two(&self) -> u64 {
        const CORRECTION: u64 = 10000000000000;
        42
    }
}

fn find_minimum_tokens(xa: u32, ya: u32, xb: u32, yb: u32, x: u32, y: u32) -> Option<u32> {
    let mut min_tokens = None;

    for a_press in 1..100 {
        for b_press in 1..100 {
            let current_x = xa * a_press + xb * b_press;
            let current_y = ya * a_press + yb * b_press;

            if current_x == x && current_y == y {
                let tokens = 3 * a_press + b_press;
                min_tokens = Some(match min_tokens {
                    Some(current_min) if tokens < current_min => tokens,
                    Some(current_min) => current_min,
                    None => tokens,
                });
            }
        }
    }

    min_tokens
}

fn parse_input(input: Vec<String>) -> (Vec<(Button, Button)>, Vec<Prize>) {
    let mut button_configs = Vec::new();
    let mut prizes = Vec::new();
    let filtered_input = input
        .iter()
        .filter(|x| x.len() > 0)
        .collect::<Vec<&String>>();
    for combo in filtered_input.chunks(3) {
        button_configs.push((
            Button {
                x: combo[0].split("+").collect::<Vec<&str>>()[1]
                    .split(",")
                    .collect::<Vec<&str>>()[0]
                    .parse::<u32>()
                    .unwrap(),
                y: combo[0].split("+").collect::<Vec<&str>>()[2]
                    .parse::<u32>()
                    .unwrap(),
            },
            Button {
                x: combo[1].split("+").collect::<Vec<&str>>()[1]
                    .split(",")
                    .collect::<Vec<&str>>()[0]
                    .parse::<u32>()
                    .unwrap(),
                y: combo[1].split("+").collect::<Vec<&str>>()[2]
                    .parse::<u32>()
                    .unwrap(),
            },
        ));
        prizes.push(Prize {
            x: combo[2].split("=").collect::<Vec<&str>>()[1]
                .split(",")
                .collect::<Vec<&str>>()[0]
                .parse::<u32>()
                .unwrap(),
            y: combo[2].split("=").collect::<Vec<&str>>()[2]
                .parse::<u32>()
                .unwrap(),
        });
    }
    (button_configs, prizes)
}

#[cfg(test)]
mod tests {
    use std::fs;

    use super::*;
    fn get_day_thirteen() -> DayThirteenSolution {
        let data = parse_input(
            fs::read_to_string("data/test/test_13.txt")
                .unwrap()
                .lines()
                .map(|x| x.to_string())
                .collect(),
        );
        DayThirteenSolution {
            button_configs: data.0,
            prizes: data.1,
        }
    }
    #[test]
    fn test_part_one() {
        let day_thirteen = get_day_thirteen();
        let output = day_thirteen.part_one();
        assert_eq!(output, 480);
    }
}
