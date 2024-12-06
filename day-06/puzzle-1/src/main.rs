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
    let mut map = Map::from(lines);
    let mut ranges: Vec<Walk> = vec![];

    loop {
        let walk = map.walk();
        ranges.push(walk.clone());
        match walk {
            HorizontalRange(_, x_to, _) => {
                if x_to == 0 || x_to == map.max_pos.0 {
                    break;
                }
            },
            VerticalRange(_, y_to, _) => {
                if y_to == 0 || y_to == map.max_pos.1 {
                    break;
                }
            },
        }
    }
    let distinct_positions = explode_positions(ranges);

    distinct_positions.len()
}

fn explode_positions(ranges: Vec<Walk>) -> HashSet<Position> {
    let mut positions: HashSet<Position> = HashSet::new();

    ranges.iter().for_each(|w| {
        let new_pos = match w {
            VerticalRange(x_from, x_to, y) => {
                let mut items: Vec<Position> = vec![];
                let (from, to) = if x_to < x_from {
                    (x_to, x_from)
                } else {
                    (x_from, x_to)
                };

                for x in *from..=*to {
                    items.push((x, *y));
                }

                items
            },
            HorizontalRange(y_from, y_to, x) => {
                let mut items: Vec<Position> = vec![];
                let (from, to) = if y_to < y_from {
                    (y_to, y_from)
                } else {
                    (y_from, y_to)
                };

                for y in *from..=*to {
                    items.push((*x, y));
                }

                items
            }
        };
        positions.extend(new_pos);
    });

    positions
}

type Position = (u64, u64);

#[derive(Debug, Clone)]
enum Walk {
    HorizontalRange(u64, u64, u64),
    VerticalRange(u64, u64, u64)
}

#[derive(Debug)]
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

        assert_eq!(result, 41);
    }
}
