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
    let mut input = lines.split(|l| l.len() == 0);
    let all_towels: Vec<Towel> = input.next().unwrap().get(0).unwrap()
        .split(", ").map(|t| Towel::from(t)).collect();

    let all_patterns: Vec<ColourPattern> = input.next().unwrap()
        .iter().map(|l| ColourPattern::from(l))
        .collect();

    all_patterns.iter().filter(|p| {
            p.is_valid(&all_towels)
        })
        .count()
}

struct Towel {
    colours: Vec<Colour>
}

impl Towel {
    pub fn from(colours: &str) -> Self {
        let colours: Vec<Colour> = colours.chars().map(|c| Colour::from(c)).collect();

        Towel { colours }
    }
}

struct ColourPattern {
    pattern: Vec<Colour>
}

impl ColourPattern {
    pub fn from(string_pattern: &String) -> Self {
        let pattern: Vec<Colour> = string_pattern.chars().map(|c| Colour::from(c)).collect();

        ColourPattern { pattern }
    }

    pub fn is_valid(&self, towels: &Vec<Towel>) -> bool {
        self.can_solve(towels, 0)
    }

    fn can_solve(&self, towels: &Vec<Towel>, start: usize) -> bool {
        if start == self.pattern.len() {
            return true;
        }

        // try to find a towel which starts with pattern, if so, forward the slice
        let pattern = self.pattern.get(start..).unwrap();
        let matching_towels: Vec<&Towel> = towels.iter().filter(|t| {
            let towel_colours = t.colours.as_slice();
            pattern.starts_with(towel_colours)
        }).collect();

        for t in matching_towels {
            let r = self.can_solve(towels, start + t.colours.len());
            if r {
                return true;
            }
        }

        false
    }
}

#[derive(Debug, Eq, PartialEq)]
enum Colour {
    White, Blue, Black, Red, Green
}

impl Colour {
    pub fn from(c: char) -> Self {
        match c {
            'w' => Colour::White,
            'u' => Colour::Blue,
            'b' => Colour::Black,
            'r' => Colour::Red,
            'g' => Colour::Green,
            _ => panic!("Unknown colour: '{}'", c)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part() {
        let result = process(&read_file(&String::from("../test-input")));

        assert_eq!(result, 6);
    }
}
