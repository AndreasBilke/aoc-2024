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
    let mut left_list: Vec<i64> = Vec::new();
    let mut right_list: Vec<i64> = Vec::new();

    lines.iter().for_each(|line| {
        let mut split = line.split("   ");
        let left_str = split.next().unwrap();
        let left_num = left_str.parse::<i64>().unwrap();

        let right_str = split.next().unwrap();
        let right_num = right_str.parse::<i64>().unwrap();

        left_list.push(left_num);
        right_list.push(right_num);
    });

    let result: usize = left_list.iter().map(|left_num| {
        let occurrence = right_list.iter()
            .filter(|&right_num| left_num == right_num)
            .count();

        (left_num * occurrence as i64) as usize
    }).sum();

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part() {
        let result = process(&read_file(&String::from("../test-input")));

        assert_eq!(result, 31);
    }
}
