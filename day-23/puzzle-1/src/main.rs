use std::collections::{HashMap, HashSet};
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
    let g = Graph::from(lines);

    g.number_of_filtered_3_cliques()
}

struct Graph {
    edges: HashMap<String, Vec<String>>
}

impl Graph {
    pub fn from(lines: &Vec<String>) -> Self {
        let mut edges: HashMap<String, Vec<String>> = HashMap::new();

        for line in lines {
            let mut components = line.split("-");
            let n1 = String::from(components.next().unwrap());
            let n2 = String::from(components.next().unwrap());

            Graph::add_edge(n1, n2, &mut edges);
        }

        Graph { edges }
    }

    fn add_edge(from: String, to: String, edges: &mut HashMap<String, Vec<String>>) {
        if !edges.contains_key(&from) {
            edges.insert(from.clone(), vec![]);
        }
        if !edges.contains_key(&to) {
            edges.insert(to.clone(), vec![]);
        }

        edges.get_mut(&from).unwrap().push(to.clone());
        edges.get_mut(&to).unwrap().push(from.clone());
    }

    fn find_cliques(&self) -> HashSet<[String; 3]>  {
        let mut result: HashSet<[String; 3]> = HashSet::new();

        // for each node, take a window of 2 for all neighbours.
        // to be a 3-clique each node in the window must be connected with the other
        self.edges.iter().for_each(|(n, all_neighbours)| {
            all_neighbours.iter().cartesian_product(all_neighbours.clone()).for_each(|neighbours| {
                // we know already n -> neighbours.0
                // we know already n -> neighbours.1
                // check if neighbours[0] -> neighbours[1]
                if self.edges.get(neighbours.0).unwrap().contains(&neighbours.1) {
                    let mut clique = [n.clone(), neighbours.0.clone(), neighbours.1];
                    clique.sort();
                    result.insert(clique);
                }
            });
        });

        result
    }

    pub fn number_of_filtered_3_cliques(&self) -> usize {
        let all_cliques = self.find_cliques();
        // only count clique if a node starting with "t" is included
        all_cliques.iter().fold(0, |r, clique| {
            let c = if clique.iter().any(|node| node.starts_with("t")) {
                1
            } else {
                0
            };

            r + c
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part() {
        let result = process(&read_file(&String::from("../test-input")));

        assert_eq!(result, 7);
    }
}
