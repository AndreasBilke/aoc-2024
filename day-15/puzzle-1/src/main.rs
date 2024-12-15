use std::collections::HashMap;
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
    let mut inputs= lines.split(|l| l.len() == 0);
    let mut map = Map::from(inputs.next().unwrap());
    let movements: Vec<Movement> = inputs.next().unwrap()
        .iter().flat_map(|s| {
            s.chars().map(|c| Movement::from(c))
        })
        .collect();

    map.draw();
    movements.iter().for_each(|m| {
        map.move_robot(m.clone());
        // println!("Map after {:?}", m);
        // map.draw();
    });

    sum_of_coordinates(&map)
}

fn sum_of_coordinates(map: &Map) -> usize {
    map.objects.iter()
        .filter(|(_, o)| {
            **o == Object::Box
        })
        .map(|(c, _)| {
            (100 * c.1 + c.0) as usize
        })
        .sum()
}

#[derive(Debug)]
struct Map {
    robot_position: (i64, i64),
    objects: HashMap<(i64, i64), Object>
}

impl Map {
    pub fn from(lines: &[String]) -> Self {
        let mut objects: HashMap<(i64, i64), Object> = HashMap::new();
        let mut robot_position: (i64, i64) = (0, 0);

        for (row, line) in lines.iter().enumerate() {
            for (column, c) in line.chars().enumerate() {
                let coordinate = (column as i64, row as i64);
                let object = Object::from(c);
                if let Some(object) = object {
                    if object == Object::Robot {
                        robot_position = coordinate;
                    }
                    objects.insert(coordinate, object);
                }
            }
        }

        Map { robot_position, objects }
    }

    pub fn move_robot(&mut self, m: Movement) {
        let next_coord = m.next(self.robot_position);
        let robot_neighbour = self.objects.get(&next_coord);
        if let Some(robot_neighbour) = robot_neighbour {
            match robot_neighbour {
                Object::Wall => {} // if next neighbour is wall, don't move at all
                Object::Box => {
                    if self.move_boxes(next_coord, &m) { // if there was a movement, move robot as well
                        self.objects.insert(self.robot_position, Object::Nothing);
                        self.objects.insert(next_coord, Object::Robot);
                        self.robot_position = next_coord;
                    }
                }
                Object::Nothing => { // if next neighbour is empty space, just move
                    self.objects.insert(self.robot_position, Object::Nothing);
                    self.objects.insert(next_coord, Object::Robot);
                    self.robot_position = next_coord;
                }
                _ => {}
            }
        } else {
            panic!("Could not find neighbour for robot at {:?} with neighbour pos {:?}", self.robot_position, next_coord);
        }
    }

    pub fn move_boxes(&mut self, start: (i64, i64), movement: &Movement) -> bool {
        // find next object of type nothing, move all boxes (but only if there was no wall
        // in between
        if let Some(p) = self.find_free_position(start, movement) {
            // move boxes. between start and p there are only boxes.
            // no need to update boxes in between, since the content of each
            // coordinate will not change
            self.objects.insert(p, Object::Box);
            self.objects.insert(start, Object::Nothing);

            true
        } else {
            false
        }
    }

    fn find_free_position(&self, start: (i64, i64), movement: &Movement) -> Option<(i64, i64)> {
        match movement {
            Movement::Right => {
                for x in start.0 + 1.. {
                    let object = self.objects.get(&(x, start.1)).unwrap();
                    if *object == Object::Wall {
                        return None;
                    } else if *object == Object::Nothing {
                        return Some((x, start.1));
                    }
                }
            },
            Movement::Left => {
                for x in (0..=start.0 - 1).rev() {
                    let object = self.objects.get(&(x, start.1)).unwrap();
                    if *object == Object::Wall {
                        return None;
                    } else if *object == Object::Nothing {
                        return Some((x, start.1));
                    }
                }
            },
            Movement::Up => {
                for y in (0..=start.1 - 1).rev() {
                    let object = self.objects.get(&(start.0, y)).unwrap();
                    if *object == Object::Wall {
                        return None;
                    } else if *object == Object::Nothing {
                        return Some((start.0, y));
                    }
                }
            },
            Movement::Down => {
                for y in start.1 + 1.. {
                    let object = self.objects.get(&(start.0, y)).unwrap();
                    if *object == Object::Wall {
                        return None;
                    } else if *object == Object::Nothing {
                        return Some((start.0, y));
                    }
                }
            }
        }

        None
    }

    pub fn draw(&self) {
        let max_x = self.objects.iter().map(|(c, _)| c.0).max().unwrap();
        let max_y = self.objects.iter().map(|(c, _)| c.1).max().unwrap();

        for row in 0..=max_x {
            for column in 0..=max_y {
                let o = self.objects.get(&(column, row)).unwrap();
                let c = match o {
                    Object::Nothing => '.',
                    Object::Wall => '#',
                    Object::Box => 'O',
                    Object::Robot => '@'
                };
                print!("{}", c);
            }
            println!();
        }

    }
}

#[derive(Debug, Clone)]
enum Movement {
    Up, Down, Left, Right
}

impl Movement {
    pub fn from(s: char) -> Self {
        match s {
            '^' => Movement::Up,
            'v' => Movement::Down,
            '<' => Movement::Left,
            '>' => Movement::Right,
            _ => panic!("Unknown direction symbol: {:?}", s)
        }
    }
    pub fn next(&self, c: (i64, i64)) -> (i64, i64) {
        match self {
            Movement::Up => (c.0, c.1 - 1),
            Movement::Down => (c.0, c.1 + 1),
            Movement::Left => (c.0 - 1, c.1),
            Movement::Right => (c.0 + 1, c.1)
        }
    }

}

#[derive(Debug, Eq, PartialEq)]
enum Object {
    Robot, Wall, Box, Nothing
}

impl Object {
    pub fn from(s: char) -> Option<Self> {
        match s {
            '#' => Some(Object::Wall),
            '@' => Some(Object::Robot),
            'O' => Some(Object::Box),
            '.' => Some(Object::Nothing),
            _ => None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let result = process(&read_file(&String::from("../test-input-1")));

        assert_eq!(result, 10092);
    }

    #[test]
    fn test_part2() {
        let result = process(&read_file(&String::from("../test-input-2")));

        assert_eq!(result, 2028);
    }
}
