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
    let buyers = lines.iter().map(|l| Buyer::from(l)).collect::<Vec<Buyer>>();

    buyers.iter().fold(0, |r, b| {
        r + b.compute(2000)
    }) as usize
}

struct Buyer {
    seed: u64
}

impl Buyer {
    pub fn from(s: &String) -> Self {
        let seed = s.parse::<u64>().unwrap();

        Buyer { seed }
    }

    pub fn compute(&self, rounds: usize) -> u64 {
        let mut new_secret = self.seed;
        (0..rounds).for_each(|_| {
            new_secret = ((new_secret * 64) ^ new_secret) % 16777216;
            new_secret = ((new_secret / 32) ^ new_secret) % 16777216;
            new_secret = ((new_secret * 2048) ^ new_secret) % 16777216;
        });

        new_secret
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part() {
        let result = process(&read_file(&String::from("../test-input")));

        assert_eq!(result, 37327623);
    }
}
