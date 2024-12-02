use std::env;
use std::fs;
use adjacent_pair_iterator::AdjacentPairIterator;
use itertools::Itertools;

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

    let valid_lines = numbers.iter().map(|l| {
            (l, is_valid(l))
        })
        .map(|vl| {
            if vl.1 {
                return true
            }

            try_combinations(vl.0)
        })
        .filter(|b| b == &true)
        .count();

    valid_lines
}

fn try_combinations(line: &Vec<i64>) -> bool {
    // the whole line is not valid. Try to systematically remove
    // one number each time and check again
    // Trying all combinations is a horrible idea when it comes to run time
    // Tell no one that I teach algorithms and data structures

    let combination_valid = line.iter()
        .combinations(line.len() - 1)
        .any(|c| {
            let combination:Vec<i64> = c.iter().map(|&&i| i).collect();

            is_valid(&combination)
        });

    combination_valid
}

fn is_valid(nums: &Vec<i64>) -> bool {
    let diffs: Vec<i64> = nums.adjacent_pairs().map(|t| {
        t.1 - t.0
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

        assert_eq!(result, 4);
    }
}
