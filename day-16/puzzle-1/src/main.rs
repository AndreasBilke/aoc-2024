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
    let m = Map::from(lines);

    m.solve()
}

#[derive(Debug)]
struct Map {
    edges: HashMap<(i64, i64), Vec<(i64, i64)>>,
    start: (i64, i64),
    end: (i64, i64)
}

impl Map {
    pub fn from(lines: &Vec<String>) -> Self {
        let mut points: HashSet<(i64, i64)> = HashSet::new();
        let mut start = (0, 0);
        let mut end = (0, 0);

        // first: store all points, later convert it to graph structure
        for (row, line) in lines.iter().enumerate() {
            for (column, c) in line.chars().enumerate() {
                if c != '#' {
                    points.insert((column as i64, row as i64));
                }

                if c == 'S' {
                    start = (column as i64, row as i64);
                } else if c == 'E' {
                    end = (column as i64, row as i64);
                }
            }
        }

        let mut edges: HashMap<(i64, i64), Vec<(i64, i64)>> = HashMap::new();
        for p in &points {
            let possible_neighbours = vec![
                points.get(&(p.0 - 1, p.1)),
                points.get(&(p.0 + 1, p.1)),
                points.get(&(p.0, p.1 - 1)),
                points.get(&(p.0, p.1 + 1)),
            ];
            let mut real_neighbours: Vec<(i64, i64)> = vec![];
            possible_neighbours.iter().for_each(|pn| {
                if let Some(&n) = pn {
                    real_neighbours.push(n);
                }
            });
            edges.insert(p.clone(), real_neighbours);
        }

        Map { edges, start, end }
    }

    pub fn solve(&self) -> usize {
        let mut pq: PriorityQueue<((i64, i64), Direction), Reverse<i64>> = PriorityQueue::new();
        let mut seen: HashSet<((i64, i64), Direction)> = HashSet::new();
        pq.push((self.start, Direction::East), Reverse(0));

        let shortest_path = loop {
            if pq.is_empty() {
                break None;
            }

            let ((next_point, direction), value) = pq.pop().unwrap();
            if next_point == self.end {
                break Some(value.0);
            }
            if seen.contains(&(next_point, direction.clone())) {
                continue;
            } else {
                seen.insert((next_point, direction.clone()));
            }

            for neighbour in self.edges.get(&next_point).unwrap() {
                let (from_to_distance, from_to_direction) = Map::distance(&next_point, neighbour, direction.clone());
                pq.push((*neighbour, from_to_direction), Reverse(value.0 + from_to_distance));
            }
        };

        shortest_path.unwrap() as usize
    }

    fn new_direction(from: &(i64, i64), to: &(i64, i64)) -> Direction {
        let x_change = to.0 - from.0;
        let y_change = to.1 - from.1;

        if x_change < 0 {
            Direction::West
        } else if x_change > 0 {
            Direction::East
        } else if y_change < 0 {
            Direction::North
        } else {
            Direction::South
        }
    }

    fn distance(from: &(i64, i64), to: &(i64, i64), from_direction: Direction) -> (i64, Direction) {
        let from_to_direction = Map::new_direction(from, to);

        let costs = match from_direction {
            Direction::North => {
                match from_to_direction {
                    Direction::North => 1,
                    Direction::West => 1001,
                    Direction::East => 1001,
                    Direction::South => 2001
                }
            },
            Direction::South => {
                match from_to_direction {
                    Direction::North => 2001,
                    Direction::West => 1001,
                    Direction::East => 1001,
                    Direction::South => 1
                }
            },
            Direction::East => {
                match from_to_direction {
                    Direction::North => 1001,
                    Direction::West => 2001,
                    Direction::East => 1,
                    Direction::South => 1001
                }
            },
            Direction::West => {
                match from_to_direction {
                    Direction::North => 1001,
                    Direction::West => 1,
                    Direction::East => 2001,
                    Direction::South => 1001
                }
            },
        };

        (costs, from_to_direction)
    }
}

#[derive(Debug, Eq, PartialEq, Clone, Hash)]
pub enum Direction {
    North, South, East, West
}

impl Direction {
    pub fn as_str(&self) -> &str {
        match self {
            Direction::North => "^",
            Direction::South => "v",
            Direction::East => ">",
            Direction::West => "<"
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let result = process(&read_file(&String::from("../test-input-1")));

        assert_eq!(result, 7036);
    }

    #[test]
    fn test_part2() {
        let result = process(&read_file(&String::from("../test-input-2")));

        assert_eq!(result, 11048);
    }
}
