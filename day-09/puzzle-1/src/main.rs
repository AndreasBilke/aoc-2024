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
    let line = lines.get(0).unwrap(); // we have only one line

    let mut empty_indicies: Vec<usize> = vec![];
    let mut blocks: Vec<Block> = vec![];

    let mut disk_index = 0;

    for (input_index, item) in line.chars().enumerate() {
        let amount = item.to_digit(10).unwrap();
        (0..amount).for_each(|sub_index| {
            let i = disk_index + sub_index;
            let block_count = input_index / 2;
            if input_index % 2 == 0 { // block data
                blocks.push(Block { id: block_count as u64, disk_index: i as usize})
            } else { // empty data
                empty_indicies.push(i as usize);
            }
        });
        disk_index += amount;
    }

    // zip empty blocks with reversed block list
    // for empty block, replace the blocks disk_index
    // stop process if empty block index exceeds the old index of the blocks
    for (empty_index, block) in empty_indicies.iter().zip(blocks.iter_mut().rev()) {
        if empty_index > &block.disk_index {
            break;
        }

        block.disk_index = *empty_index;
    }

    compute_checksum(&blocks) as usize
}

fn compute_checksum(blocks: &Vec<Block>) -> u64 {
    blocks.iter().fold(0, |a, b| {
        a + b.id * b.disk_index as u64
    })
}

struct Block {
    id: u64,
    disk_index: usize
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part() {
        let result = process(&read_file(&String::from("../test-input")));

        assert_eq!(result, 1928);
    }
}
