use std::collections::HashMap;
use std::env;
use std::fs;
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
    let components = lines.split(|l| l.len() == 0);

    let mut locks: Vec<Lock> = vec![];
    let mut keys: Vec<Key> = vec![];

    for component in components {
        // it's a lock
        if component.get(0).unwrap().starts_with("#") {
            let l = Lock::from(component);
            locks.push(l);
        } else { // it's a key
            let k = Key::from(component);
            keys.push(k);
        }
    }

    let occurrences: HashMap<(u8, u8, u8, u8, u8), u64> = count_occurrences(&locks);

    let mut matching_keys = 0;
    for key in &keys {
        let inverse_pins = (key.pins[0], key.pins[1], key.pins[2], key.pins[3], key.pins[4]);
        let matching_locks = occurrences.get(&inverse_pins).unwrap_or(&0);
        matching_keys += matching_locks;
    }

    matching_keys as usize
}

fn count_occurrences(locks: &Vec<Lock>) -> HashMap<(u8, u8, u8, u8, u8), u64> {
    let mut occurrences: HashMap<(u8, u8, u8, u8, u8), u64> = HashMap::new();

    for lock in locks {
        for p0 in 0..=5 - lock.pins[0] {
            for p1 in 0..=5 - lock.pins[1] {
                for p2 in 0..=5 - lock.pins[2] {
                    for p3 in 0..=5 - lock.pins[3] {
                        for p4 in 0..=5 - lock.pins[4] {
                            let combo = (p0, p1, p2, p3, p4);
                            let old_value = occurrences.get(&combo).unwrap_or(&0);
                            occurrences.insert(combo, *old_value + 1u64);
                        }
                    }
                }
            }
        }
    }

    occurrences
}

pub fn extract_components(r: (u8, u8, u8, u8, u8), l: &String) -> (u8, u8, u8, u8, u8) {
    let chars = l.chars().collect::<Vec<char>>();
    let mut row = (0, 0, 0, 0, 0);
    if chars.get(0).unwrap() == &'#' {
        row.0 = 1;
    }
    if chars.get(1).unwrap() == &'#' {
        row.1 = 1;
    }
    if chars.get(2).unwrap() == &'#' {
        row.2 = 1;
    }
    if chars.get(3).unwrap() == &'#' {
        row.3 = 1;
    }
    if chars.get(4).unwrap() == &'#' {
        row.4 = 1;
    }

    (r.0 + row.0, r.1 + row.1, r.2 + row.2, r.3 + row.3, r.4 + row.4)
}

#[derive(Debug)]
struct Lock {
    pins: [u8; 5]
}

impl Lock {
    pub fn from(lines: &[String]) -> Self {
        if lines.len() != 7 {
            panic!("Unknown lock format");
        }
        if lines.get(0).unwrap() != "#####" {
            panic!("Input does not start with lock indicator");
        }

        let pins = lines.iter().dropping(1).dropping_back(1).fold((0, 0, 0, 0, 0), |r, l| {
            extract_components(r, l)
        });

        Lock { pins: [pins.0, pins.1, pins.2, pins.3, pins.4] }
    }
}

#[derive(Debug)]
struct Key {
    pins: [u8; 5]
}

impl Key {
    pub fn from(lines: &[String]) -> Self {
        if lines.len() != 7 {
            panic!("Unknown key format");
        }
        if lines.get(0).unwrap() != "....." {
            panic!("Input does not start with key indicator");
        }

        let pins = lines.iter().dropping(1).dropping_back(1).fold((0, 0, 0, 0, 0), |r, l| {
            extract_components(r, l)
        });

        Key { pins: [pins.0, pins.1, pins.2, pins.3, pins.4] }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part() {
        let result = process(&read_file(&String::from("../test-input")));

        assert_eq!(result, 3);
    }
}
