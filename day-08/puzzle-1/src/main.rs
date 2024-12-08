use std::collections::HashSet;
use std::env;
use std::fs;
use itertools::Itertools;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

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

    let antinodes = map.compute_antinodes();
    print_map(&map, &antinodes);

    antinodes.len()
}

fn print_map(map: &Map, antinodes: &HashSet<(i64, i64)>) {
    for r in 0..=map.max_location.1 {
        for c in 0..=map.max_location.0 {
            let p = (c, r);
            if is_antenna(map, &p) && antinodes.contains(&p) {
                print!("D");
            } else if is_antenna(map, &p) {
                print!("X");
            } else if antinodes.contains(&p) {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!();
    }
}

fn is_antenna(map: &Map, p: &(i64, i64)) -> bool {
    let r = map.antennas.iter().find_map(|a| {
        if a.location.0 == p.0 && a.location.1 == p.1 {
            Some(a)
        } else {
            None
        }
    });

    match r {
        Some(_) => true,
        None => false
    }
}

struct Map {
    antennas: HashSet<Antenna>,
    max_location: (i64, i64)
}

impl Map {
    pub fn from(lines: &Vec<String>) -> Self {
        let mut antennas: HashSet<Antenna> = HashSet::new();

        for (row, line) in lines.iter().enumerate() {
            for (column, c) in line.chars().enumerate() {
                if c == '.' {
                    continue;
                }

                let antenna = Antenna::from(c, (column as i64, row as i64));
                antennas.insert(antenna);
            }
        }

        let max_x = lines.get(0).unwrap().len() - 1;
        let max_y = lines.len() - 1;

        println!("Max x {:?}, Max y {:?}", max_x, max_y);

        Map { antennas, max_location: (max_x as i64, max_y  as i64) }
    }
    
    pub fn compute_antinodes(&self) -> HashSet<(i64, i64)> {
        let mut antinodes: HashSet<(i64, i64)> = HashSet::new();

        // loop over all antenna types
        // produce cartesian product (i.e. all pairs of same time)
        // compute distance
        // produce antinodes from distance

        AntennaType::iter().for_each(|at| {
           let same_type_antennas: Vec<&Antenna> = self.antennas
               .iter()
               .filter(|a| a.antenna_type == at)
               .collect();

            let copy_antennas = same_type_antennas.clone();
            same_type_antennas.iter().cartesian_product(copy_antennas)
                .map(|(a1, a2)| {
                    (
                        a1.location, a2.location, a1.distance(a2)
                    )
                }).for_each(|(l1, l2, d)| {
                    // if distance is zero, l1 and l2 are the same spots
                    if !(d.0 == 0 && d.1 == 0) {
                        let mut ans: Vec<(i64, i64)> = vec![];
                        ans.push((l1.0 + d.0, l1.1 + d.1));
                        ans.push((l1.0 - d.0, l1.1 - d.1));

                        ans.push((l2.0 + d.0, l2.1 + d.1));
                        ans.push((l2.0 - d.0, l2.1 - d.1));

                        ans.iter().for_each(|an| {
                            if self.is_in_map(an) && *an != l1 && *an != l2 {
                                antinodes.insert(an.clone());
                            }
                        });
                    }
                });
        });

        antinodes
    }

    pub fn is_in_map(&self, a: &(i64, i64)) -> bool {
        if a.0 < 0 || a.1 < 0 {
            return false;
        }

        if a.0 > self.max_location.0 || a.1 > self.max_location.1 {
            return false;
        }

        true
    }
}

#[derive(Debug, Hash, Eq, PartialEq)]
struct Antenna {
    antenna_type: AntennaType,
    location: (i64, i64)
}

impl Antenna {
    pub fn from(c: char, pos: (i64, i64)) -> Self {
        Antenna { antenna_type: AntennaType::from(c), location: pos }
    }

    pub fn distance(&self, other: &Antenna) -> (i64, i64) {
        (
            self.location.0 - other.location.0,
            self.location.1 - other.location.1
        )
    }
}

#[derive(Debug, EnumIter, Hash, Eq, PartialEq)]
enum AntennaType {
    TypeA,
    TypeB,
    TypeC,
    TypeD,
    TypeE,
    TypeF,
    TypeG,
    TypeH,
    TypeI,
    TypeN,
    TypeJ,
    TypeK,
    TypeL,
    TypeM,
    TypeO,
    TypeP,
    TypeQ,
    TypeR,
    TypeS,
    TypeT,
    TypeU,
    TypeV,
    TypeW,
    TypeX,
    TypeY,
    TypeZ,
    Typea,
    Typeb,
    Typec,
    Typed,
    Typee,
    Typef,
    Typeg,
    Typeh,
    Typei,
    Typej,
    Typek,
    Typel,
    Typem,
    Typen,
    Typeo,
    Typep,
    Typeq,
    Typer,
    Types,
    Typet,
    Typeu,
    Typev,
    Typew,
    Typex,
    Typey,
    Typez,
    Type0,
    Type1,
    Type2,
    Type3,
    Type4,
    Type5,
    Type6,
    Type7,
    Type8,
    Type9
}

impl AntennaType {
    pub fn from(pos: char) -> Self {
        match pos {
            'A' => AntennaType::TypeA,
            'B' => AntennaType::TypeB,
            'C' => AntennaType::TypeC,
            'D' => AntennaType::TypeD,
            'E' => AntennaType::TypeE,
            'F' => AntennaType::TypeF,
            'G' => AntennaType::TypeG,
            'H' => AntennaType::TypeH,
            'I' => AntennaType::TypeI,
            'J' => AntennaType::TypeJ,
            'K' => AntennaType::TypeK,
            'L' => AntennaType::TypeL,
            'M' => AntennaType::TypeM,
            'N' => AntennaType::TypeN,
            'O' => AntennaType::TypeO,
            'P' => AntennaType::TypeP,
            'Q' => AntennaType::TypeQ,
            'R' => AntennaType::TypeR,
            'S' => AntennaType::TypeS,
            'T' => AntennaType::TypeT,
            'U' => AntennaType::TypeU,
            'V' => AntennaType::TypeV,
            'W' => AntennaType::TypeW,
            'X' => AntennaType::TypeX,
            'Y' => AntennaType::TypeY,
            'Z' => AntennaType::TypeZ,
            'a' => AntennaType::Typea,
            'b' => AntennaType::Typeb,
            'c' => AntennaType::Typec,
            'd' => AntennaType::Typed,
            'e' => AntennaType::Typee,
            'f' => AntennaType::Typef,
            'g' => AntennaType::Typeg,
            'h' => AntennaType::Typeh,
            'i' => AntennaType::Typei,
            'j' => AntennaType::Typej,
            'k' => AntennaType::Typek,
            'l' => AntennaType::Typel,
            'm' => AntennaType::Typem,
            'n' => AntennaType::Typen,
            'o' => AntennaType::Typeo,
            'p' => AntennaType::Typep,
            'q' => AntennaType::Typeq,
            'r' => AntennaType::Typer,
            's' => AntennaType::Types,
            't' => AntennaType::Typet,
            'u' => AntennaType::Typeu,
            'v' => AntennaType::Typev,
            'w' => AntennaType::Typew,
            'x' => AntennaType::Typex,
            'y' => AntennaType::Typey,
            'z' => AntennaType::Typez,
            '0' => AntennaType::Type0,
            '1' => AntennaType::Type1,
            '2' => AntennaType::Type2,
            '3' => AntennaType::Type3,
            '4' => AntennaType::Type4,
            '5' => AntennaType::Type5,
            '6' => AntennaType::Type6,
            '7' => AntennaType::Type7,
            '8' => AntennaType::Type8,
            '9' => AntennaType::Type9,
            x=> panic!("'{}' is an unsupported antenna type", x)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part() {
        let result = process(&read_file(&String::from("../test-input")));

        assert_eq!(result, 14);
    }
}
