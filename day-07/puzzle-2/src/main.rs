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
    let equations: Vec<Equation> = lines.iter().map(|l| Equation::from(l)).collect();
    let result: u64 = equations
        .iter().filter(|e| e.is_valid())
        .map(|e| e.lhs)
        .sum();

    result as usize
}

struct Equation {
    lhs: u64,
    rhs: Vec<u64>
}

impl Equation {
    pub fn from(line: &String) -> Self {
        let mut eq_split = line.split(": ");
        let lhs = eq_split.next().unwrap().parse::<u64>().unwrap();

        let rhs: Vec<u64> = eq_split.next().unwrap()
            .split(" ").map(|e| e.parse::<u64>().unwrap())
            .collect();

        Equation { lhs, rhs }
    }

    pub fn is_valid(&self) -> bool {
        let mut possible_results: Vec<u64> = vec![];
        possible_results.push(*self.rhs.get(0).unwrap());

        // combine each element (from left to right) with all possible
        // previously found results
        self.rhs.iter().skip(1).for_each(|e| {
            let mut new_results: Vec<u64> = vec![];
            possible_results.iter().for_each(|p_r| {
                new_results.push(p_r + e);
                new_results.push(p_r * e);
                // concatenate the ugly way :)
                let s = format!("{}{}", p_r, e).parse::<u64>().unwrap();
                new_results.push(s);
            });
            possible_results = new_results;
        });

        possible_results.contains(&self.lhs)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part() {
        let result = process(&read_file(&String::from("../test-input")));

        assert_eq!(result, 11387);
    }
}
