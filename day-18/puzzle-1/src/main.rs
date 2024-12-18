use std::cmp::Reverse;
use std::collections::{HashMap, HashSet};
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
    let result = process(&lines, (70, 70), 1024);
    // let result = process(&lines, (6, 6), 12);

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

pub fn process(lines: &Vec<String>, end_point: (i64, i64), bytes_fallen: usize) -> usize {
    let corrupted_bytes: Vec<(i64, i64)> = lines.iter().map(|s| {
        let split = s.split_once(",").unwrap();
        let x = split.0.parse::<i64>().unwrap();
        let y = split.1.parse::<i64>().unwrap();
        (x, y)
    }).collect();
    let corrupted_bytes_at_time: &[(i64, i64)] = corrupted_bytes.get(0..bytes_fallen).unwrap();

    dijkstra_thingy(end_point, corrupted_bytes_at_time)
}

fn dijkstra_thingy(end_point: (i64, i64), corrupted_bytes: &[(i64, i64)]) -> usize {
    let mut pq: PriorityQueue<(i64, i64), Reverse<i64>> = PriorityQueue::new();
    let mut distances: HashMap<(i64, i64), i64> = HashMap::new();
    let mut predecessors: HashMap<(i64, i64), (i64, i64)> = HashMap::new();

    pq.push((0, 0), Reverse(0));
    distances.insert((0, 0), 0);

    let shortest_path = loop {
        if pq.is_empty() {
            panic!("Queue is empty, but goal not reached");
        }

        let (p, distance) = pq.pop().unwrap();
        if p == end_point {
            break distance.0;
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

    let path = construct_path(predecessors, &end_point);
    draw(&end_point, corrupted_bytes, path);

    shortest_path as usize
}

fn construct_path(predecessors: HashMap<(i64, i64), (i64, i64)>, end: &(i64, i64)) -> HashSet<(i64, i64)> {
    let mut c = end;
    let mut path: HashSet<(i64, i64)> = HashSet::new();

    loop {
        if *c == (0, 0) {
            break;
        }
        let p = predecessors.get(c).unwrap();
        path.insert(p.clone());
        c = p;
    }

    path
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

fn draw(grid: &(i64, i64), corrupted_bytes: &[(i64, i64)], path: HashSet<(i64, i64)>) {
    for y in 0..=grid.1 {
        for x in 0..=grid.0 {
            if corrupted_bytes.contains(&(x, y)) {
                print!("#");
            } else if path.contains(&(x, y)){
                print!("O");
            } else {
                print!(".");
            }
        }
        println!();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part() {
        let result = process(&read_file(&String::from("../test-input")), (6, 6), 12);

        assert_eq!(result, 22);
    }
}
