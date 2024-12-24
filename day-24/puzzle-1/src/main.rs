use std::cmp::Reverse;
use std::collections::HashMap;
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
    let mut system = System::from(lines);
    // find out all z-Gates by searching the output gates list
    let mut z_gates = system.rules.iter().filter(|(gate_name, _)| {
        gate_name.starts_with("z")
    }).map(|(gate_name, _)| {
        gate_name.clone()
    }).collect::<Vec<String>>();

    system.compute_outputs(&z_gates);

    z_gates.sort_by_key(|b| Reverse(b.clone()));

    let out_string = z_gates.iter().map(|gate| {
        let gate_value = system.values.get(gate).unwrap();
        format!("{}", gate_value)
    }).collect::<Vec<String>>().join("");
    let result = usize::from_str_radix(out_string.as_str(), 2).unwrap();

    result
}

#[derive(Debug)]
struct System {
    rules: HashMap<String, Gate>,
    values: HashMap<String, u8>
}

impl System {
    pub fn from(lines: &Vec<String>) -> Self {
        let mut components = lines.split(|l| l.len() == 0);

        let mut values = HashMap::new();
        components.next().unwrap().iter().for_each(|iv| {
            let mut line_split = iv.split(": ");
            let gate_name = String::from(line_split.next().unwrap());
            let gate_value = line_split.next().unwrap().parse::<u8>().unwrap();

            values.insert(gate_name, gate_value);
        });

        let re = Regex::new(r"([a-z0-9]{3}) (AND|OR|XOR) ([a-z0-9]{3}) -> ([a-z0-9]{3})").unwrap();
        let mut rules = HashMap::new();
        components.next().unwrap().iter().for_each(|gr| {
            let matches = re.captures(gr).unwrap();
            let g1 = String::from(matches.get(1).unwrap().as_str());
            let operation = String::from(matches.get(2).unwrap().as_str());
            let g2 = String::from(matches.get(3).unwrap().as_str());
            let result_gate = String::from(matches.get(4).unwrap().as_str());

            let operation = Gate::from(g1, g2, operation);
            rules.insert(result_gate, operation);
        });

        System { rules, values }
    }

    pub fn compute_outputs(&mut self, gates: &Vec<String>) {
        // trigger solve for each z-Gate if no value is present
        for z_gate in gates {
            let result = self.compute_value(&z_gate);
            self.values.insert(z_gate.clone(), result);
        }
    }

    fn compute_value(&self, for_gate: &String) -> u8 {
        if self.values.contains_key(for_gate) {
            return *self.values.get(for_gate).unwrap();
        }

        let gate = self.rules.get(for_gate).unwrap();
        let result = match gate {
            Gate::And(g1, g2) => {
                let g1_value = self.compute_value(g1);
                let g2_value = self.compute_value(g2);

                g1_value & g2_value
            },
            Gate::Or(g1, g2) => {
                let g1_value = self.compute_value(g1);
                let g2_value = self.compute_value(g2);

                g1_value | g2_value
            },
            Gate::Xor(g1, g2) => {
                let g1_value = self.compute_value(g1);
                let g2_value = self.compute_value(g2);

                g1_value ^ g2_value
            }
        };

        result
    }
}

#[derive(Debug)]
enum Gate {
    And(String, String),
    Or(String, String),
    Xor(String, String)
}

impl Gate {
    pub fn from(g1: String, g2: String, operation: String) -> Self {
        match operation.as_str() {
            "AND" => Gate::And(g1, g2),
            "OR" => Gate::Or(g1, g2),
            "XOR" => Gate::Xor(g1, g2),
            _ => panic!("Unsupported gate type: {}", operation)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part() {
        let result = process(&read_file(&String::from("../test-input")));

        assert_eq!(result, 2024);
    }
}
