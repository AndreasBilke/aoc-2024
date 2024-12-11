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
    let numbers: Vec<u64> = lines.get(0).unwrap()
        .split(" ")
        .map(|n| n.parse::<u64>().unwrap())
        .collect();

    // do not simulate the stones. Just count how often each stone appeared
    let mut number_counter: HashMap<u64, u64> = HashMap::new();
    numbers.iter().for_each(|n| { number_counter.insert(*n, 1); });

    let mut loop_counter = 0;
    loop {
        if loop_counter == 25 {
            break;
        }

        let mut new_number_counter: HashMap<u64, u64> = HashMap::new();
        for (k, v) in number_counter.iter() {
            let blink = blink(*k);
            match blink {
                StoneResult::Replace(n_k) => {
                    let old_value = new_number_counter.get(&n_k).unwrap_or(&0);
                    new_number_counter.insert(n_k, old_value + v);
                },
                StoneResult::Split(n_k1, n_k2) => {
                    let old_value1 = new_number_counter.get(&n_k1).unwrap_or(&0);
                    new_number_counter.insert(n_k1, old_value1 + v);
                    let old_value2 = new_number_counter.get(&n_k2).unwrap_or(&0);
                    new_number_counter.insert(n_k2, old_value2 + v);
                }
            }
        }

        number_counter = new_number_counter;
        loop_counter += 1;
    }

    // result is the sum of all values
    number_counter.iter().fold(0, |r, (_, value)| {
        r + *value
    }) as usize
}

fn blink(number: u64) -> StoneResult {
    if number == 0 {
        StoneResult::Replace(1)
    } else if let Some((number1, number2)) = split_if_even_number_of_digits(number) {
        StoneResult::Split(number1, number2)
    } else {
        StoneResult::Replace(number * 2024)
    }
}

fn split_if_even_number_of_digits(number: u64) -> Option<(u64, u64)> {
    let number_of_digits = number.ilog10() + 1;
    if number_of_digits % 2 == 1 {
        return None;
    }
    let divider = 10u64.pow(number_of_digits / 2);

    let number1 = number / divider;
    let number2 = number - number1 * divider;

    Some((number1, number2))
}

enum StoneResult {
    Split(u64, u64),
    Replace(u64)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part() {
        let result = process(&read_file(&String::from("../test-input")));

        assert_eq!(result, 55312);
    }
}
