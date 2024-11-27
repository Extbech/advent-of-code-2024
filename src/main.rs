use advent_of_code_2024::*;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    parse_input(args);
}

fn parse_input(args: Vec<String>) {
    match args.len() {
        1 => println!("Running all days ..."),
        2 => match args[1].as_str() {
            "1" | "one" => {
                let day_one = DayOne::new();
                day_one.solve();
            },
            _x => println!("Day {_x} not found or implemented"),
        },
        _ => println!("Invalid Input. Too many number of arguments"),
    }
}