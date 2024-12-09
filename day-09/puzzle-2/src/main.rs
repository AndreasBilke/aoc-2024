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

    let mut empty_indicies: Vec<(usize, u64)> = vec![];
    let mut blocks: Vec<Block> = vec![];

    let mut disk_index = 0;

    for (input_index, item) in line.chars().enumerate() {
        let amount = item.to_digit(10).unwrap() as u64;

        if input_index % 2 == 0 { // block data
            let block_count = input_index / 2;
            blocks.push(Block { id: block_count as u64, disk_index: (disk_index, amount)})
        } else { // empty data
            empty_indicies.push((disk_index, amount));
        }

        disk_index += amount as usize;
    }

    defragnent(&mut blocks, &mut empty_indicies);

    compute_checksum(&blocks) as usize
}

fn defragnent(data_blocks: &mut Vec<Block>, empty_blocks: &mut Vec<(usize, u64)>) {
    // idea: start from last block, find first big enough block in empty_blocks
    // update data_block start index
    // remove/split empty_block from list

    for block in data_blocks.iter_mut().rev() {
        let empty_pos = empty_blocks.iter().position(|&e| {
            e.0 < block.disk_index.0 && e.1 >= block.disk_index.1
        });
        if let Some(empty_pos) = empty_pos {
            let empty_block = empty_blocks[empty_pos];
            block.disk_index.0 = empty_block.0;
            // split block
            if empty_block.1 == block.disk_index.1 { // case 1: block fits completely
                empty_blocks.remove(empty_pos);
            } else { // empty block is larger than data block.
                let new_empty_block = (empty_block.0 + block.disk_index.1 as usize, empty_block.1 - block.disk_index.1);
                empty_blocks[empty_pos] = new_empty_block;
            }
        }
    }
}

fn compute_checksum(blocks: &Vec<Block>) -> u64 {
    blocks.iter().fold(0, |a, b| {
        let mut block_check_sum = 0;
        for offset in 0..b.disk_index.1 {
            block_check_sum += b.id * (b.disk_index.0 as u64 + offset);
        }

        a + block_check_sum
    })
}

#[derive(Debug)]
struct Block {
    id: u64,
    disk_index: (usize, u64) // start index, number of elements
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part() {
        let result = process(&read_file(&String::from("../test-input")));

        assert_eq!(result, 2858);
    }
}
