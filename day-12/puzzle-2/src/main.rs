use std::collections::{HashSet, VecDeque};
use std::env;
use std::fs;
use std::hash::{Hash, Hasher};
use itertools::Itertools;
use itertools::MinMaxResult::{OneElement, MinMax};

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
        let segments = compute_segments(f, &map);
        r + segments * f.len()
    })
}

// for segment calculation, find continuous segments (top, bottom, left, right separately)
// by going row/column wise through all points
fn compute_segments(field: &HashSet<Plant>, map: &HashSet<Plant>) -> usize {
    let (top_segments, bottom_segments) = compute_top_bottom_segments(field, map);
    let (left_segments, right_segments) = compute_left_right_segments(field, map);

    top_segments + bottom_segments + left_segments + right_segments
}

// a top/bottom segment is a plant, where the top neighbour has a different PlantType Or
// if the plant is at a border
fn compute_top_bottom_segments(field: &HashSet<Plant>, map: &HashSet<Plant>) -> (usize, usize) {
    let minmax = field.iter().minmax_by_key(|p| p.y);
    let (min_row, max_row) = match minmax {
        OneElement(p) => (p.y, p.y),
        MinMax(p_min, p_max) => (p_min.y, p_max.y),
        _ => panic!("No minmax element found")
    };

    let mut total_top_segments = 0;
    let mut total_bottom_segments = 0;
    (min_row..=max_row).for_each(|row| {
        let plants = get_plants_by_row(row, field);
        let top_plants: Vec<&Plant> = plants.iter().copied().filter(|p| is_plant_top_element(&p, map)).collect();
        let bottom_plants: Vec<&Plant> = plants.iter().copied().filter(|p| is_plant_bottom_element(&p, map)).collect();

        let top_x_values = get_ordered_x_values(&top_plants);
        total_top_segments += count_segments(&top_x_values);

        let bottom_x_values = get_ordered_x_values(&bottom_plants);
        total_bottom_segments += count_segments(&bottom_x_values);
    });

    (total_top_segments, total_bottom_segments)
}

fn compute_left_right_segments(field: &HashSet<Plant>, map: &HashSet<Plant>) -> (usize, usize) {
    let minmax = field.iter().minmax_by_key(|p| p.x);
    let (min_column, max_column) = match minmax {
        OneElement(p) => (p.x, p.x),
        MinMax(p_min, p_max) => (p_min.x, p_max.x),
        _ => panic!("No minmax element found")
    };

    let mut total_left_segments = 0;
    let mut total_right_segments = 0;
    (min_column..=max_column).for_each(|row| {
        let plants = get_plants_by_column(row, field);
        let left_plants: Vec<&Plant> = plants.iter().copied().filter(|p| is_plant_left_element(&p, map)).collect();
        let right_plants: Vec<&Plant> = plants.iter().copied().filter(|p| is_plant_right_element(&p, map)).collect();

        let left_y_values = get_ordered_y_values(&left_plants);
        total_left_segments += count_segments(&left_y_values);

        let right_y_values = get_ordered_y_values(&right_plants);
        total_right_segments += count_segments(&right_y_values);
    });

    (total_left_segments, total_right_segments)
}

fn count_segments(numbers: &Vec<i64>) -> usize {
    if numbers.len() == 0 {
        return 0;
    }

    let mut segment_counter = 1usize;

    // we have a list of ids. Count the number of increasing segments
    // e.g. 1 2 3 5 6 8
    // has 3 segments. 1-3, 5-6 and 8
    numbers.windows(2)
        .for_each(|n| {
           if n[1] > n[0] + 1 {
               segment_counter += 1;
           }
        });

    segment_counter
}

fn get_ordered_x_values(plants: &Vec<&Plant>) -> Vec<i64> {
    plants.iter()
        .map(|p| p.x)
        .sorted()
        .collect()
}

fn get_ordered_y_values(plants: &Vec<&Plant>) -> Vec<i64> {
    plants.iter()
        .map(|p| p.y)
        .sorted()
        .collect()
}

fn is_plant_left_element(plant: &Plant, map: &HashSet<Plant>) -> bool {
    let left_neighbour = map.get(&Plant { x: plant.x - 1, y: plant.y, plant_type: plant.plant_type});
    match left_neighbour {
        Some(n) => n.plant_type != plant.plant_type,
        None => true
    }
}

fn is_plant_right_element(plant: &Plant, map: &HashSet<Plant>) -> bool {
    let left_neighbour = map.get(&Plant { x: plant.x + 1, y: plant.y, plant_type: plant.plant_type});
    match left_neighbour {
        Some(n) => n.plant_type != plant.plant_type,
        None => true
    }
}

fn is_plant_top_element(plant: &Plant, map: &HashSet<Plant>) -> bool {
    let top_neighbour = map.get(&Plant { x: plant.x, y: plant.y - 1, plant_type: plant.plant_type});
    match top_neighbour {
        Some(n) => n.plant_type != plant.plant_type,
        None => true
    }
}

fn is_plant_bottom_element(plant: &Plant, map: &HashSet<Plant>) -> bool {
    let top_neighbour = map.get(&Plant { x: plant.x, y: plant.y + 1, plant_type: plant.plant_type});
    match top_neighbour {
        Some(n) => n.plant_type != plant.plant_type,
        None => true
    }
}

fn get_plants_by_row(row: i64, field: &HashSet<Plant>) -> Vec<&Plant> {
    let row: Vec<&Plant> = field.iter().filter(|p| {
       p.y ==  row
    }).collect();

    row
}

fn get_plants_by_column(column: i64, field: &HashSet<Plant>) -> Vec<&Plant> {
    let column: Vec<&Plant> = field.iter().filter(|p| {
        p.x ==  column
    }).collect();

    column
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
        let result = process(&read_file(&String::from("../test-input-1")));
        assert_eq!(result, 1206);
    }

    #[test]
    fn test_part_file2() {
        let result = process(&read_file(&String::from("../test-input-2")));
        assert_eq!(result, 436);
    }

    #[test]
    fn test_part_file3() {
        let result = process(&read_file(&String::from("../test-input-3")));
        assert_eq!(result, 80);
    }

    #[test]
    fn test_part_file4() {
        let result = process(&read_file(&String::from("../test-input-4")));
        assert_eq!(result, 368);
    }
}
