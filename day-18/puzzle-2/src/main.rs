use std::cmp::Reverse;
use std::collections::HashMap;
use std::env;
use std::fs;
use priority_queue::PriorityQueue;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        panic!("Not enough command line arguments");
    }
    
    let input = &args[1];
    let lines = read_file(input);
    let result = process(&lines, (70, 70));

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

pub fn process(lines: &Vec<String>, end_point: (i64, i64)) -> String {
    let corrupted_bytes: Vec<(i64, i64)> = lines.iter().map(|s| {
        let split = s.split_once(",").unwrap();
        let x = split.0.parse::<i64>().unwrap();
        let y = split.1.parse::<i64>().unwrap();
        (x, y)
    }).collect();

    for bytes_fallen in 0..corrupted_bytes.len() {
        let corrupted_bytes_at_time: &[(i64, i64)] = corrupted_bytes.get(0..bytes_fallen).unwrap();
        let result = dijkstra_thingy(end_point, corrupted_bytes_at_time);
        if let Some(coordinates) = result {
            return format!("{},{}", coordinates.0, coordinates.1);
        }
    }

    "".to_string() // default value
}

fn dijkstra_thingy(end_point: (i64, i64), corrupted_bytes: &[(i64, i64)]) -> Option<(i64, i64)> {
    let mut pq: PriorityQueue<(i64, i64), Reverse<i64>> = PriorityQueue::new();
    let mut distances: HashMap<(i64, i64), i64> = HashMap::new();
    let mut predecessors: HashMap<(i64, i64), (i64, i64)> = HashMap::new();

    pq.push((0, 0), Reverse(0));
    distances.insert((0, 0), 0);

    let last_byte = loop {
        if pq.is_empty() {
            return Some(corrupted_bytes.last().unwrap().clone()); // last failing byte
        }

        let (p, distance) = pq.pop().unwrap();
        if p == end_point {
            break None; // we reached the end
        }

        for neighbour in get_neighbours(p, end_point, corrupted_bytes) {

            let old_neighbour_distance = *distances.get(&neighbour).unwrap_or(&i64::MAX);
            if old_neighbour_distance > distance.0 + 1 {
                pq.push_decrease(neighbour, Reverse(distance.0 + 1));
                distances.insert(neighbour, distance.0 + 1);
                predecessors.insert(neighbour, p);
            }
        }
    };

    last_byte
}

fn get_neighbours(p: (i64, i64), grid: (i64, i64), broken_bytes: &[(i64, i64)]) -> Vec<(i64, i64)> {
    let possible_neighbours = vec![
        (p.0 - 1, p.1),
        (p.0 + 1, p.1),
        (p.0, p.1 - 1),
        (p.0, p.1 + 1)
    ];

    let real_neighbours: Vec<(i64, i64)> = possible_neighbours.iter().filter(|&n| {
            if n.0 < 0 || n.0 > grid.0 {
                return false;
            }

            if n.1 < 0 || n.1 > grid.1 {
                return false;
            }

            !broken_bytes.contains(n)
        })
        .map(|n| n.clone())
        .collect();

    real_neighbours
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part() {
        let result = process(&read_file(&String::from("../test-input")), (6, 6));

        assert_eq!(result, "6,1");
    }
}
