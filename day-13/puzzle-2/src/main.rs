use std::env;
use std::fs;
use regex::Regex;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        panic!("Not enough command line arguments");
    }
    
    let input = &args[1];
    let lines = read_file(input);
    let result = process(&lines);
    
    println!("Result is {}", result);
}

pub fn read_file(file_name: &String) -> Vec<String> {
    let lines = fs::read_to_string(file_name)
        .expect("Could not read file");

    let lines: Vec<String> = lines
        .trim()
        .split('\n')
        .map(String::from)
        .collect();
    
    lines
}

pub fn process(lines: &Vec<String>) -> usize {
    let claw_machine_input = lines.split(|l| l.len() == 0);

    let result: i64 = claw_machine_input
        .map(|s| ClawMachine::from(s))
        .map(|c| c.min_costs())
        .filter(|r| r.0 == true)
        .map(|r| r.1 )
        .sum();

    result as usize
}

#[derive(Debug)]
struct ClawMachine {
    target_x: i64,
    target_y: i64,
    button_a_x_factor: i64,
    button_a_y_factor: i64,
    button_b_x_factor: i64,
    button_b_y_factor: i64
}

impl ClawMachine {
    pub fn from(input: &[String]) -> Self {
        if input.len() != 3 {
            panic!("Unknown input format: {:?}", input);
        }

        // e.g.: Button A: X+94, Y+34
        let button_regex = Regex::new(r"Button [AB]: X\+(\d+), Y\+(\d+)").unwrap();
        // e.g.: Prize: X=12748, Y=12176
        let target_regex = Regex::new(r"Prize: X=(\d+), Y=(\d+)").unwrap();

        let button_a_result = button_regex.captures(input[0].as_str()).unwrap();
        let button_a_x_factor = button_a_result.get(1).unwrap().as_str().parse::<i64>().unwrap();
        let button_a_y_factor = button_a_result.get(2).unwrap().as_str().parse::<i64>().unwrap();

        let button_b_result = button_regex.captures(input[1].as_str()).unwrap();
        let button_b_x_factor = button_b_result.get(1).unwrap().as_str().parse::<i64>().unwrap();
        let button_b_y_factor = button_b_result.get(2).unwrap().as_str().parse::<i64>().unwrap();

        let prize_result = target_regex.captures(input[2].as_str()).unwrap();
        let target_x = prize_result.get(1).unwrap().as_str().parse::<i64>().unwrap() + 10000000000000;
        let target_y = prize_result.get(2).unwrap().as_str().parse::<i64>().unwrap() + 10000000000000;

        ClawMachine { target_x, target_y, button_a_x_factor, button_a_y_factor, button_b_x_factor, button_b_y_factor }
    }

    pub fn min_costs(&self) -> (bool, i64) {
        let a_dividend = (self.target_x * self.button_b_y_factor - self.button_b_x_factor * self.target_y) as f64;
        let a_divisor = (self.button_a_x_factor * self.button_b_y_factor - self.button_b_x_factor * self.button_a_y_factor) as f64;
        let a = a_dividend / a_divisor;

        if a - a.floor() != 0f64 { // is this the proper way of checking a for being an integer?
            return (false, 0);
        }
        let b = (self.target_y as f64 - self.button_a_y_factor as f64 * a) / self.button_b_y_factor as f64;
        let r = (true, 3*a as i64 + b as i64);

        r
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part() {
        let result = process(&read_file(&String::from("../test-input")));

        assert_eq!(result, 875318608908);
    }
}
