use std::collections::{HashSet, VecDeque};
use std::env;
use std::fs;
use std::hash::{Hash, Hasher};

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
    let mut map: HashSet<Plant> = HashSet::new();
    for (row, line) in lines.iter().enumerate() {
        for (column, item) in line.chars().enumerate() {
            let pt = PlantType::from(item);

            map.insert(Plant { x: column as i64, y: row as i64, plant_type: pt});
        }
    }

    // idea: try each item in map, start a flood fill from there
    // within flood fill: find each neighbouring plant with same type
    // return list of neighboured plants
    let mut available_plants = HashSet::clone(&map);
    let mut fields: Vec<HashSet<Plant>> = vec![];
    loop {
        if available_plants.is_empty() {
            break;
        }
        let next_start = available_plants.iter().next().unwrap();
        let field = flood_fill(next_start, &map);
        field.iter().for_each(|p| {
            available_plants.remove(p);
        });
        fields.push(field);
    }

    fields.iter().fold(0, |r, f| {
        let perimeter = compute_perimeter(f, &map);
        r + perimeter * f.len()
    })
}

// for calculation: for each group of plants: loop through each plant
// number of fences is the number of neighbours (in global map) with a different plant type
fn compute_perimeter(f: &HashSet<Plant>, map: &HashSet<Plant>) -> usize {
    f.iter().fold(0, |r, p| {
        let neighbours = plant_neighbours(p, map);
        let border_count = neighbours.iter().fold(0, |r, n| {
            if n.plant_type != p.plant_type {
                r + 1
            } else {
                r
            }
        });

        // if neighbours is less than 4, it means we have a border field.
        // each border field has a fence by definition

        r + border_count + (4 - neighbours.len())
    })
}

fn flood_fill(start: &Plant, map: &HashSet<Plant>) -> HashSet<Plant> {
    let mut field: HashSet<Plant> = HashSet::new();
    let mut item_stack: VecDeque<Plant> = VecDeque::new();
    item_stack.push_front(start.clone());

    loop {
        if item_stack.is_empty() {
            break;
        }
        let current_item = item_stack.pop_front().unwrap();
        field.insert(current_item.clone());

        plant_neighbours(&current_item, map).iter().for_each(|n| {
            // we are only interested in unseen neighbours of same type
            if n.plant_type == current_item.plant_type && !field.contains(n) {
                field.insert(n.clone());
                item_stack.push_front(n.clone());
            }
        });
    }

    field
}

fn plant_neighbours(plant: &Plant, map: &HashSet<Plant>) -> Vec<Plant> {
    let mut neighbours: Vec<Plant> = vec![];
    vec![
        map.get(&Plant { x: plant.x - 1, y: plant.y, plant_type: plant.plant_type }),
        map.get(&Plant { x: plant.x + 1, y: plant.y, plant_type: plant.plant_type }),
        map.get(&Plant { x: plant.x, y: plant.y - 1, plant_type: plant.plant_type }),
        map.get(&Plant { x: plant.x, y: plant.y + 1, plant_type: plant.plant_type })
    ].iter().for_each(|n| {
        if let Some(n) = n {
            neighbours.push((*n).clone());
        }
    });

    neighbours
}

#[derive(Copy, Clone, Debug)]
struct Plant {
    x: i64,
    y: i64,
    plant_type: PlantType
}

impl Hash for Plant {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.x.hash(state);
        self.y.hash(state);
    }
}

impl PartialEq<Self> for Plant {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

impl Eq for Plant {

}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
enum PlantType {
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
    TypeZ
}

impl PlantType {
    pub fn from(pos: char) -> Self {
        match pos {
            'A' => PlantType::TypeA,
            'B' => PlantType::TypeB,
            'C' => PlantType::TypeC,
            'D' => PlantType::TypeD,
            'E' => PlantType::TypeE,
            'F' => PlantType::TypeF,
            'G' => PlantType::TypeG,
            'H' => PlantType::TypeH,
            'I' => PlantType::TypeI,
            'J' => PlantType::TypeJ,
            'K' => PlantType::TypeK,
            'L' => PlantType::TypeL,
            'M' => PlantType::TypeM,
            'N' => PlantType::TypeN,
            'O' => PlantType::TypeO,
            'P' => PlantType::TypeP,
            'Q' => PlantType::TypeQ,
            'R' => PlantType::TypeR,
            'S' => PlantType::TypeS,
            'T' => PlantType::TypeT,
            'U' => PlantType::TypeU,
            'V' => PlantType::TypeV,
            'W' => PlantType::TypeW,
            'X' => PlantType::TypeX,
            'Y' => PlantType::TypeY,
            'Z' => PlantType::TypeZ,
            x=> panic!("'{}' is an unsupported antenna type", x)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_file1() {
        let result1 = process(&read_file(&String::from("../test-input-1")));
        assert_eq!(result1, 1930);
    }

    #[test]
    fn test_part_file2() {
        let result2 = process(&read_file(&String::from("../test-input-2")));
        assert_eq!(result2, 772);
    }

    #[test]
    fn test_part_file3() {
        let result3 = process(&read_file(&String::from("../test-input-3")));
        assert_eq!(result3, 140);
    }
}
