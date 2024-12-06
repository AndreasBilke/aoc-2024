use std::collections::HashSet;
use std::env;
use std::fs;
use crate::Walk::{HorizontalRange, VerticalRange};

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
    let map = Map::from(lines);

    let mut loop_counter = 0;
    (0..=map.max_pos.0).for_each(|x| {
        (0..=map.max_pos.1).for_each(|y| {
            // also check of x,y is not next to guard
            if !map.obstacles.contains(&(x, y)) {
                let mut new_map = Map::from_map(&map, (x, y));
                if new_map.has_loop() {
                    loop_counter += 1;
                }
            }
        });
    });

    loop_counter
}

type Position = (u64, u64);

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
enum Walk {
    HorizontalRange(u64, u64, u64),
    VerticalRange(u64, u64, u64)
}

#[derive(Debug, Copy, Clone)]
enum Direction {
    Up,
    Down,
    Left,
    Right
}

impl Direction {
    pub fn from(marker: &char) -> Self {
        match marker {
            '^' => Direction::Up,
            'V' => Direction::Down,
            '<' => Direction::Left,
            '>' => Direction::Right,
            other => panic!("'{}' is not a valid direction marker", other)
        }
    }

    pub fn next(&self) -> Self {
        match self {
            Direction::Up => Direction::Right,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
            Direction::Right => Direction::Down
        }
    }
}

#[derive(Debug)]
struct Map {
    obstacles: HashSet<Position>,
    guard: (Position, Direction),
    max_pos: Position
}

impl Map {
    pub fn from_map(other: &Map, new_obs: Position) -> Self {
        let mut obstacles: HashSet<Position> = HashSet::new();
        other.obstacles.iter().for_each(|o| {
            obstacles.insert(o.clone());
        });
        obstacles.insert(new_obs);

        let guard = other.guard.clone();
        let max_pos = other.max_pos;

        Map { obstacles, guard, max_pos }
    }

    pub fn from(map: &Vec<String>) -> Self {
        let mut obstacles: HashSet<Position> = HashSet::new();
        let mut guard: (Position, Direction) = ((0, 0), Direction::Right);

        for (row, line) in map.iter().enumerate() {
            for (column, c) in line.chars().enumerate() {
                match c {
                    '#' => {
                        obstacles.insert((column as u64, row as u64));
                    },
                    '.' => {}, // ignore points
                    other => {
                        guard = ((column as u64, row as u64), Direction::from(&other));
                    }
                }
            }
        }

        let max_x = obstacles.iter()
            .max_by_key(|e| e.0)
            .unwrap();
        let max_y = obstacles.iter()
            .max_by_key(|e| e.1)
            .unwrap();
        let max_pos = (max_x.0, max_y.1);

        Map { obstacles, guard, max_pos }
    }

    pub fn has_loop(&mut self) -> bool {
        let mut ranges: HashSet<Walk> = HashSet::new();

        let is_loop = loop {
            let walk = self.walk();
            if ranges.contains(&walk) {
                break true;
            }
            match walk {
                HorizontalRange(_, x_to, _) => {
                    if x_to == 0 || x_to == self.max_pos.0 {
                        break false;
                    }
                },
                VerticalRange(_, y_to, _) => {
                    if y_to == 0 || y_to == self.max_pos.1 {
                        break false;
                    }
                },
            }
            ranges.insert(walk);
        };

        is_loop
    }

    pub fn walk(&mut self) -> Walk {
        let old_pos = self.guard.0;

        let new_pos: Position = match self.guard.1 {
            Direction::Up => {
                // get all obstacles with same x pos and a smaller y pos
                let obs = self.obstacles.iter().filter(|o| {
                        o.0 == old_pos.0 && o.1 < old_pos.1
                    })
                    .map(|p| p.1)
                    .max();
                let new_y = match obs {
                    Some(pos) => pos + 1,
                    None => 0
                };
                (old_pos.0, new_y)
            },
            Direction::Down => {
                // get all obstacles with same x pos and a larger y pos
                let obs = self.obstacles.iter().filter(|o| {
                    o.0 == old_pos.0 && o.1 > old_pos.1
                })
                    .map(|p| p.1)
                    .min();
                let new_y = match obs {
                    Some(pos) => pos - 1,
                    None => self.max_pos.1
                };
                (old_pos.0, new_y)
            },
            Direction::Left => {
                // get all obstacles with same y pos and a smaller x pos
                let obs = self.obstacles.iter().filter(|o| {
                    o.1 == old_pos.1 && o.0 < old_pos.0
                })
                    .map(|p| p.0)
                    .max();
                let new_x = match obs {
                    Some(pos) => pos + 1,
                    None => 0
                };
                (new_x, old_pos.1)
            },
            Direction::Right => {
                // get all obstacles with same y pos and a larger x pos
                let obs = self.obstacles.iter().filter(|o| {
                    o.1 == old_pos.1 && o.0 > old_pos.0
                })
                    .map(|p| p.0)
                    .min();
                let new_x = match obs {
                    Some(pos) => pos - 1,
                    None => self.max_pos.0
                };
                (new_x, old_pos.1)
            }
        };
        self.guard.0 = new_pos;
        self.guard.1 = Direction::next(&self.guard.1);

        if old_pos.0 == new_pos.0 {
            VerticalRange(old_pos.1, new_pos.1, new_pos.0)
        } else {
            HorizontalRange(old_pos.0, new_pos.0, new_pos.1)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part() {
        let result = process(&read_file(&String::from("../test-input")));

        assert_eq!(result, 6);
    }
}
