use std::collections::{HashMap, HashSet, VecDeque};
use std::env;
use std::fs;
use itertools::Itertools;

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

    let default_distance = m.shortest_path();

    // for every wall, for every neighbour pair
    // remove them from the map and run the shortest path algorithm
    let mut cheat_counter = 0;

    m.objects.iter().filter(|(_, t)| {
        **t == Type::Wall
    }).for_each(|(c, _)| {
        let mut cm = m.clone();
        cm.remove_wall(c);
        let shortest_path = cm.shortest_path();
        if default_distance - shortest_path >= 100 {
            cheat_counter += 1;
        }
    });

    cheat_counter
}

type Coord = (i64, i64);

#[derive(Debug, Clone, PartialEq)]
enum Type {
    Wall, Field
}

#[derive(Debug, Clone)]
struct Map {
    objects: HashMap<Coord, Type>,
    start: Coord,
    end: Coord
}

impl Map {
    pub fn from(lines: &Vec<String>) -> Self {
        let mut start: Coord = (0, 0);
        let mut end: Coord = (0, 0);

        let mut objects: HashMap<Coord, Type> = HashMap::new();
        for (row, line) in lines.iter().dropping(1).dropping_back(1).enumerate() { // get rid of border
            for (column, c) in line.chars().dropping(1).dropping_back(1).enumerate() { // get rid of border
                let coord = (column as i64, row as i64);
                if c == '#' {
                    objects.insert(coord, Type::Wall);
                } else {
                    objects.insert(coord, Type::Field);
                }

                if c == 'S' {
                    start = coord.clone();
                } else if c == 'E' {
                    end = coord.clone();
                }
            }
        }

        Map { objects, start, end }
    }

    pub fn remove_wall(&mut self, w: &Coord) {
        self.objects.insert(w.clone(), Type::Field);
    }

    pub fn shortest_path(&self) -> usize {
        let mut queue: VecDeque<(Coord, usize)> = VecDeque::new();
        queue.push_front((self.start, 0));
        let mut seen: HashSet<Coord> = HashSet::new();
        seen.insert(self.start);

        let shortest_path = loop {
            if queue.len() == 0 {
                break None;
            }
            let (current, distance) = queue.pop_back().unwrap();
            seen.insert(current.clone());
            if current == self.end {
                break Some(distance);
            }

            for n in self.get_neighbours(&current) {
                if !seen.contains(&n) {
                    queue.push_front((n.clone(), distance + 1));
                }
            }
        };

        if let Some(distance) = shortest_path {
            distance
        } else {
            panic!("No path from {:?} to {:?}", self.start, self.end);
        }
    }

    fn get_neighbours(&self, f: &Coord) -> Vec<Coord> {
        let p_neighbours = vec![
            (f.0 - 1, f.1),
            (f.0 + 1, f.1),
            (f.0, f.1 - 1),
            (f.0, f.1 + 1)
        ];

        let mut real_neighbours = vec![];
        for c in p_neighbours {
            if let Some(field_type) = self.objects.get(&c) {
                if *field_type == Type::Field {
                    real_neighbours.push(c);
                }
            }
        }

        real_neighbours

    }
}
