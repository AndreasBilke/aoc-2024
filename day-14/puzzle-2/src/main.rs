use std::env;
use std::fs;
use regex::Regex;

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
    let robots: Vec<Robot> = lines
        .iter().map(|l| Robot::from(l.as_str()))
        .collect();

    let mut map = Map { max_x: 101, max_y: 103, robots }; // final: 101, 103

    let mut seconds = 1;
    loop {
        if seconds > 100000 {
            break;
        }

        println!("SECONDS {:?}", seconds);
        map.simulate_steps(1);
        map.draw();

        seconds += 1;
    }

    map.simulate_steps(100);
    map.draw();

    0
}

#[derive(Debug)]
struct Map {
    max_x: i64,
    max_y: i64,
    robots: Vec<Robot>
}

impl Map {
    pub fn simulate_steps(&mut self, steps: i64) {
        self.robots.iter_mut().for_each(|r| {
           r.move_in_grid(steps, self.max_x, self.max_y);
        });
    }

    pub fn draw(&self) {
        for y in 0..self.max_y {
            for x in 0..self.max_x {
                let robot = self.robots.iter().find(|r| r.x == x && r.y == y);
                if let Some(_) = robot {
                    print!("#");
                } else {
                    print!(" ");
                }
            }
            println!();
        }
    }
}

#[derive(Debug)]
struct Robot {
    x: i64,
    y: i64,
    v_x: i64,
    v_y: i64
}

impl Robot {
    fn from(l: &str) -> Self {
        // e.g.: p=23,10 v=-27,-20
        let robo_regex = Regex::new(r"p=(-?\d+),(-?\d+) v=(-?\d+),(-?\d+)").unwrap();
        let capture = robo_regex.captures(l).unwrap();

        let x = capture.get(1).unwrap().as_str().parse::<i64>().unwrap();
        let y = capture.get(2).unwrap().as_str().parse::<i64>().unwrap();
        let v_x = capture.get(3).unwrap().as_str().parse::<i64>().unwrap();
        let v_y = capture.get(4).unwrap().as_str().parse::<i64>().unwrap();

        Robot { x, y, v_x, v_y }
    }

    pub fn move_in_grid(&mut self, steps: i64, max_x: i64, max_y: i64) {
        self.x = self.x + steps * self.v_x;
        self.y = self.y + steps * self.v_y;

        // check boundaries
        if self.x >= max_x {
            self.x = self.x % max_x;
        } else if self.x < 0 {
            let wrap = self.x % max_x;
            if wrap == 0 {
                self.x = 0;
            } else {
                self.x = max_x + wrap; // wrap is negative
            }
        }

        if self.y >= max_y {
            self.y = self.y % max_y;
        } else if self.y < 0 {
            let wrap = self.y % max_y;
            if wrap == 0 {
                self.y = 0;
            } else {
                self.y = max_y + wrap;
            }
        }
    }
}
