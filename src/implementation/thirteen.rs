use crate::Solution;

pub struct DayThirteenSolution {
    button_configs: Vec<(ButtonA, ButtonB)>,
    prizes: Vec<Prize>
}

#[derive(Debug)]
struct ButtonA {
    x: u32,
    y: u32,
}

#[derive(Debug)]
struct ButtonB {
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
        DayThirteenSolution { button_configs: input.0, prizes: input.1 }
    }

    fn part_one(&self) -> u32 {
        let mut count: u32 = 0;
        for (button_config, prize) in self.button_configs.iter().zip(self.prizes.iter()) {
            let mut tokens = Vec::new();
            find_prize_recurse(&button_config.0, &button_config.1, prize.x, prize.y, 0, 0, &mut tokens);
            if tokens.len() > 0 {
                count += *tokens.iter().min().unwrap();
            }
            break;
        }
        count
    }

    fn part_two(&self) -> u32 {
        42
    }
}

fn find_prize_recurse(
    button_a: &ButtonA,
    button_b: &ButtonB,
    x: u32,
    y: u32,
    a_press: u32,
    b_press: u32,
    tokens: &mut Vec<u32>,
) {
    println!("button A: {:?}, button B: {:?}, x: {:?}, y: {:?}, a_press: {:?}, b_press: {:?}", button_a, button_b, x, y, a_press, b_press);
    if a_press > 100 && b_press > 100 {
        return;
    }
    if x == 0 && y == 0 {
        tokens.push((3*a_press) + b_press);
        println!("Tokens updated: {:?}", tokens);
        return;
    }
    // Check if we can push button A.
    if x >= button_a.x && y >= button_a.y && a_press < 100 {
        return find_prize_recurse(button_a, button_b, x - button_a.x, y - button_a.y, a_press + 1, b_press, tokens);
    }
    // Check if we can push button B.
    if x >= button_b.x && y >= button_b.y && b_press < 100 {
        return find_prize_recurse(button_a, button_b, x - button_b.x, y - button_b.y, a_press, b_press + 1, tokens);
    }

    return;
}
fn parse_input(input: Vec<String>) -> (Vec<(ButtonA, ButtonB)>, Vec<Prize>) {
    let mut button_configs = Vec::new();
    let mut prizes = Vec::new();
    let filtered_input = input.iter().filter(|x| x.len() > 0).collect::<Vec<&String>>();
    for combo in filtered_input.chunks(3) {
        button_configs.push((
            ButtonA {
                x: combo[0].split("+").collect::<Vec<&str>>()[1].split(",").collect::<Vec<&str>>()[0].parse::<u32>().unwrap(),
                y: combo[0].split("+").collect::<Vec<&str>>()[2].parse::<u32>().unwrap(),
            },
            ButtonB {
                x: combo[1].split("+").collect::<Vec<&str>>()[1].split(",").collect::<Vec<&str>>()[0].parse::<u32>().unwrap(),
                y: combo[1].split("+").collect::<Vec<&str>>()[2].parse::<u32>().unwrap(),
            },
        ));
        prizes.push(Prize {
            x: combo[2].split("=").collect::<Vec<&str>>()[1].split(",").collect::<Vec<&str>>()[0].parse::<u32>().unwrap(),
            y: combo[2].split("=").collect::<Vec<&str>>()[2].parse::<u32>().unwrap(),
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
        DayThirteenSolution { button_configs: data.0, prizes: data.1 }
    }
    #[test]
    fn test_part_one() {
        let day_thirteen = get_day_thirteen();
        let output = day_thirteen.part_one();
        assert_eq!(output, 480);
    }
}