use std::collections::HashSet;
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
    let map = Map::from(lines);

    map.explore()
}

struct Map {
    points: HashSet<Point>
}

impl Map {
    pub fn from(lines: &Vec<String>) -> Self {
        let mut points: HashSet<Point> = HashSet::new();

        for (y, line) in lines.iter().enumerate() {
            for (x, c) in line.chars().enumerate() {
                let value = c.to_digit(10).unwrap();

                points.insert(Point { x: x as i64, y: y as i64, height: value as i64});
            }
        }

        Map { points }
    }

    pub fn explore(&self) -> usize {
        let starting_points = self.points.iter().filter(|p| p.height == 0);

        starting_points.fold(0, |r, p| {
            let distinct_paths = self.explore_from_point(p);

            r + distinct_paths
        })
    }

    fn explore_from_point(&self, from: &Point) -> usize {
        if from.height == 9 {
            return 1;
        }

        let new_neighbours_coords = vec![
            (from.x - 1, from.y),
            (from.x + 1, from.y),
            (from.x, from.y - 1),
            (from.x, from.y + 1)
        ];
        // this search could be more efficient
        let new_neighbours: Vec<&Point> = self.points.iter().filter(|p| {
           let p_coord = (p.x, p.y);

            new_neighbours_coords.contains(&p_coord)
        }).filter(|p| {
            p.height == from.height + 1
        }).collect();

        let mut distinct_paths = 0;
        new_neighbours.iter().for_each(|p| {
            distinct_paths += self.explore_from_point(p)
        });

        distinct_paths
    }
}

#[derive(Debug, Hash, PartialEq, Eq)]
struct Point {
    x: i64,
    y: i64,
    height: i64
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part() {
        let result = process(&read_file(&String::from("../test-input")));

        assert_eq!(result, 81);
    }
}
