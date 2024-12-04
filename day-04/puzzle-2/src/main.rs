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

fn get_word(data: &Vec<String>, coords: &[(i64, i64); 3]) -> String {
    let mut s = String::new();

    coords.iter().for_each(|c| s.push(get(data, c)));

    s
}

fn get(data: &Vec<String>, coord: &(i64, i64)) -> char {
    // each string is equally long, it's enough to get number of columns from first String
    if coord.0 < 0 || coord.0 >= data.get(0).unwrap().len() as i64 {
        return '.';
    }

    if coord.1 < 0 || coord.1 >= data.len() as i64 {
        return '.';
    }

    data.get(coord.1 as usize).unwrap().chars().nth(coord.0 as usize).unwrap()
}

pub fn process(lines: &Vec<String>) -> usize {
    let max_x = lines.get(0).unwrap().len() as i64;
    let max_y = lines.len() as i64;

    let result = (0..max_x).flat_map(|x| (0..max_y).map(move |y| (x, y)))
        .fold(0, |r, init_coord| {
            let w_1 = get_word(lines, &[
                (init_coord.0 - 1, init_coord.1 - 1),
                (init_coord.0, init_coord.1),
                (init_coord.0 + 1, init_coord.1 + 1)
            ]);
            let w_2 = get_word(lines, &[
                (init_coord.0 - 1, init_coord.1 + 1),
                (init_coord.0, init_coord.1),
                (init_coord.0 + 1, init_coord.1 - 1)
            ]);

            let mut hit = 0;
            if (w_1.eq("MAS") || w_1.eq("SAM")) &&  (w_2.eq("MAS") || w_2.eq("SAM")) {
                hit = 1;
            }

            r + hit
        });

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part() {
        let result = process(&read_file(&String::from("../test-input")));

        assert_eq!(result, 9);
    }
}
