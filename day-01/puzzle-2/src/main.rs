use std::collections::HashMap;
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
    let mut left_list: Vec<usize> = Vec::new();
    let mut right_list: Vec<usize> = Vec::new();

    lines.iter().for_each(|line| {
        let mut split = line.split("   ");
        let left_num = split.next().unwrap().parse::<usize>().unwrap();
        let right_num = split.next().unwrap().parse::<usize>().unwrap();

        left_list.push(left_num);
        right_list.push(right_num);
    });

    let occurrence = right_list.iter()
        .fold(HashMap::<usize, usize>::new(), |mut o, item| {
            *o.entry(*item).or_default() += 1;
            o
        });

    let result = left_list.iter().fold(0usize, |r, left_num| {
        let count = match occurrence.get_key_value(&left_num) {
            Some(c) => *c.1,
            None => 0
        };

        r + left_num * count
    });

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
