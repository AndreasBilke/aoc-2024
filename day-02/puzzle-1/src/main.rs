use std::env;
use std::fs;

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
    let numbers: Vec<Vec<i64>> = lines
        .iter().map(|l| {
            let numbers: Vec<i64> = l.split(' ')
                .map(|n| n.parse::<i64>().unwrap())
                .collect();

            numbers
        }).collect();

    let valid_lines = numbers.iter().map(is_valid)
        .filter(|r| *r == true)
        .count();

    valid_lines
}

fn is_valid(nums: &Vec<i64>) -> bool {
    let diffs: Vec<i64> = nums
        .windows(2)
        .map(|t| {
            t[1] - t[0]
        }).collect();

    // check if differences are between -3..-1 or 1..3
    let diffs_ok = diffs.iter().all(|d| {
        (-3..=-1).contains(d) || (1..=3).contains(d)
    });

    if !diffs_ok {
        return false;
    }

    // check if sequence is either fully decreasing or increasing
    diffs.iter().all(|d| d < & 0) || diffs.iter().all(|d| d > & 0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part() {
        let result = process(&read_file(&String::from("../test-input")));

        assert_eq!(result, 2);
    }
}
