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
    let re = Regex::new(r"mul\(([1-9][0-9]{0,2}),([1-9][0-9]{0,2})\)").unwrap();

    let result: i64 = lines.iter().fold(0, |r, l| {
            let line_result  = re.captures_iter(l).fold(0, |r, c| {
                let n_1 = c.get(1).unwrap().as_str().parse::<i64>().unwrap();
                let n_2 = c.get(2).unwrap().as_str().parse::<i64>().unwrap();

                r + n_1 * n_2
            });

            r + line_result
        });

    result as usize
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part() {
        let result = process(&read_file(&String::from("../test-input")));

        assert_eq!(result, 161);
    }
}
