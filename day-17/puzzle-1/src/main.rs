use std::env;
use std::fs;
use std::ops::BitXor;
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

pub fn process(lines: &Vec<String>) -> String {
    let mut input_split = lines.split(|l| l.len() == 0);
    let cpu_data = input_split.next().unwrap();

    let instruction_line = input_split.next().unwrap().get(0).unwrap();
    // Format --> Program: 0,1,5,4,3,0
    let mut instruction_split = instruction_line.split(" ");
    instruction_split.next();
    let instructions: Vec<i64> = instruction_split.next().unwrap().split(",").map(|i| i.parse::<i64>().unwrap()).collect();

    let mut cpu = Cpu::from(cpu_data, instructions);
    loop {
        let halt = cpu.tick();
        if halt {
            break;
        }
    }

    cpu.read_output()
}

#[derive(Debug)]
struct Cpu {
    register_a: i64,
    register_b: i64,
    register_c: i64,

    instruction_pointer: i64,
    instructions: Vec<i64>,

    output_buffer: Vec<i64>
}

impl Cpu {
    pub fn from(data: &[String], instructions: Vec<i64>) -> Self {
        let register_regex = Regex::new(r"Register [ABC]: (\d+)").unwrap();
        let reg_a_capture = register_regex.captures(data.get(0).unwrap()).unwrap();
        let value_reg_a = reg_a_capture.get(1).unwrap().as_str().parse::<i64>().unwrap();

        let reg_b_capture = register_regex.captures(data.get(1).unwrap()).unwrap();
        let value_reg_b = reg_b_capture.get(1).unwrap().as_str().parse::<i64>().unwrap();

        let reg_c_capture = register_regex.captures(data.get(2).unwrap()).unwrap();
        let value_reg_c = reg_c_capture.get(1).unwrap().as_str().parse::<i64>().unwrap();

        let output_buffer: Vec<i64> = vec![];

        Cpu { register_a: value_reg_a, register_b: value_reg_b, register_c: value_reg_c, instructions, instruction_pointer: 0, output_buffer }
    }

    pub fn tick(&mut self) -> bool {
        if self.instruction_pointer as usize >= self.instructions.len() {
            return true;
        }
        let op_code = self.instructions[self.instruction_pointer as usize];
        let operand = self.instructions[(self.instruction_pointer + 1)  as usize];
        let op = Operation::from(
            op_code,
            operand
        );
        Operation::execute(op, self);

        false // program continues
    }

    pub fn read_output(&self) -> String {
        let out_as_str: Vec<String> = self.output_buffer.iter().map(|o| o.to_string()).collect();
        out_as_str.join(",")
    }
}

enum Operation {
    Adv(i64),
    Bxl(i64),
    Bst(i64),
    Jnz(i64),
    Bxc,
    Out(i64),
    Bdv(i64),
    Cdv(i64)
}

impl Operation {
    pub fn from(op_code: i64, operand: i64) -> Self {
        match op_code {
            0 => Operation::Adv(operand),
            1 => Operation::Bxl(operand),
            2 => Operation::Bst(operand),
            3 => Operation::Jnz(operand),
            4 => Operation::Bxc,
            5 => Operation::Out(operand),
            6 => Operation::Bdv(operand),
            7 => Operation::Cdv(operand),
            _ => { panic!("Unknown opcode {} with operand {}", op_code, operand); }
        }
    }

    pub fn execute(operation: Operation, cpu: &mut Cpu) {
        match operation {
            Operation::Adv(combo_operand) => {
                let numerator = cpu.register_a;
                let denominator = 2i64.pow(Operation::translate_combo_operand(combo_operand, cpu) as u32);
                let result = numerator / denominator;

                cpu.register_a = result;
            }
            Operation::Bxl(literal_operand) => {
                let result = cpu.register_b.bitxor(literal_operand);
                cpu.register_b = result;
            }
            Operation::Bst(combo_operand) => {
                cpu.register_b = Operation::translate_combo_operand(combo_operand, cpu) % 8;
            }
            Operation::Jnz(literal_operand) => {
                if cpu.register_a != 0 {
                    cpu.instruction_pointer = literal_operand;
                } else {
                    cpu.instruction_pointer += 2; // override maybe the later default increasing
                }
            }
            Operation::Bxc => {
                let result = cpu.register_b.bitxor(cpu.register_c);
                cpu.register_b = result;
            }
            Operation::Out(combo_operand) => {
                let result = Operation::translate_combo_operand(combo_operand, cpu) % 8;
                cpu.output_buffer.push(result);
            }
            Operation::Bdv(combo_operand) => {
                let numerator = cpu.register_a;
                let denominator = 2i64.pow(Operation::translate_combo_operand(combo_operand, cpu) as u32);
                let result = numerator / denominator;

                cpu.register_b = result;
            }
            Operation::Cdv(combo_operand) => {
                let numerator = cpu.register_a;
                let denominator = 2i64.pow(Operation::translate_combo_operand(combo_operand, cpu) as u32);
                let result = numerator / denominator;

                cpu.register_c = result;
            }
        }

        // forward instruction counter
        match operation {
            Operation::Jnz(_) => {}, // no instruction increase
            _ => { cpu.instruction_pointer += 2 } // all others: increase by two
        }
    }

    fn translate_combo_operand(operand: i64, cpu: &Cpu) -> i64 {
        match operand {
            0 => 0,
            1 => 1,
            2 => 2,
            3 => 3,
            4 => cpu.register_a,
            5 => cpu.register_b,
            6 => cpu.register_c,
            _ => panic!("Unsupported operand {}", operand)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part() {
        let result = process(&read_file(&String::from("../test-input")));

        assert_eq!(result, "4,6,3,5,6,3,5,2,1,0");
    }
}
