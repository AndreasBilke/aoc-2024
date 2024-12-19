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
    let mut input = lines.split(|l| l.len() == 0);
    let all_towels: Vec<Towel> = input.next().unwrap().get(0).unwrap()
        .split(", ").map(|t| Towel::from(t)).collect();

    let all_patterns: Vec<ColourPattern> = input.next().unwrap()
        .iter().map(|l| ColourPattern::from(l))
        .collect();

    let mut onsen = Onsen::from(all_patterns, all_towels);
    onsen.compute_all()
}

struct Onsen {
    patterns: Vec<ColourPattern>,
    towels: Vec<Towel>,
    cache: HashMap<String, usize>
}

impl Onsen {
    pub fn from(patterns: Vec<ColourPattern>, towels: Vec<Towel>) -> Self {
        let cache: HashMap<String, usize> = HashMap::new();

        Onsen { patterns, towels, cache }
    }

    pub fn compute_all(&mut self) -> usize {
        self.patterns.iter().fold(0, |r, p| {
            r + p.total_number_of_patterns(&self.towels, &mut self.cache)
        })
    }
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

    pub fn total_number_of_patterns(&self, towels: &Vec<Towel>, cache: &mut HashMap<String, usize>) -> usize {
        self.patterns(towels, 0, cache)
    }

    fn patterns(&self, towels: &Vec<Towel>, start: usize, cache: &mut HashMap<String, usize>) -> usize {
        if start == self.pattern.len() {
            return 1;
        }

        // try to find a towel which starts with pattern, if so, forward the slice
        let pattern = self.pattern.get(start..).unwrap();
        // try search a result in the cache
        let pattern_string = ColourPattern::create_string(pattern);
        if let Some(count) = cache.get(&pattern_string) {
            return *count;
        }

        // no cache hit, we need to calculate it
        let matching_towels: Vec<&Towel> = towels.iter().filter(|t| {
            let towel_colours = t.colours.as_slice();
            pattern.starts_with(towel_colours)
        }).collect();

        let number_of_patterns = matching_towels.iter().fold(0, |r, t| {
            r + self.patterns(towels, start + t.colours.len(), cache)
        });
        // cache it for later usage
        cache.insert(pattern_string, number_of_patterns);

        number_of_patterns
    }

    fn create_string(colours: &[Colour]) -> String {
        String::from_iter(colours.iter().map(|c| c.as_char()))
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

    pub fn as_char(&self) -> char {
        match self {
            Colour::White => 'w',
            Colour::Blue => 'u',
            Colour::Black => 'b',
            Colour::Red => 'r',
            Colour::Green => 'g'
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part() {
        let result = process(&read_file(&String::from("../test-input")));

        assert_eq!(result, 16);
    }
}
