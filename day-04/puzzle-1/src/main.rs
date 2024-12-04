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

fn get_all_coords(coord: (i64, i64)) -> Vec<[(i64, i64); 4]> {
    let mut coords: Vec<[(i64, i64); 4]> = vec![];

    // horizontal, to right
    coords.push([
        (coord.0, coord.1),
        (coord.0 + 1, coord.1),
        (coord.0 + 2, coord.1),
        (coord.0 + 3, coord.1)
    ]);
    // horizontal, to left
    coords.push([
        (coord.0, coord.1),
        (coord.0 - 1, coord.1),
        (coord.0 - 2, coord.1),
        (coord.0 - 3, coord.1)
    ]);

    // vertical, to up
    coords.push([
        (coord.0, coord.1),
        (coord.0, coord.1 - 1),
        (coord.0, coord.1 - 2),
        (coord.0, coord.1 - 3)
    ]);
    // vertical, to down
    coords.push([
        (coord.0, coord.1),
        (coord.0, coord.1 + 1),
        (coord.0, coord.1 + 2),
        (coord.0, coord.1 + 3)
    ]);

    // diagonal, right up
    coords.push([
        (coord.0, coord.1),
        (coord.0 + 1, coord.1 - 1),
        (coord.0 + 2, coord.1 - 2),
        (coord.0 + 3, coord.1 - 3)
    ]);
    // diagonal, right down
    coords.push([
        (coord.0, coord.1),
        (coord.0 + 1, coord.1 + 1),
        (coord.0 + 2, coord.1 + 2),
        (coord.0 + 3, coord.1 + 3)
    ]);

    // diagonal, left up
    coords.push([
        (coord.0, coord.1),
        (coord.0 - 1, coord.1 - 1),
        (coord.0 - 2, coord.1 - 2),
        (coord.0 - 3, coord.1 - 3)
    ]);
    // diagonal, left down
    coords.push([
        (coord.0, coord.1),
        (coord.0 - 1, coord.1 + 1),
        (coord.0 - 2, coord.1 + 2),
        (coord.0 - 3, coord.1 + 3)
    ]);

    coords
}

fn get_word(data: &Vec<String>, coords: &[(i64, i64)]) -> String {
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
            let all_coords = get_all_coords(init_coord);
            let xmas_matches = all_coords
                .iter().filter(|coord| {
                    let word = get_word(lines, *coord);

                    word.eq("XMAS")
                }).count();

            r + xmas_matches
        });

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part() {
        let result = process(&read_file(&String::from("../test-input")));

        assert_eq!(result, 18);
    }
}
