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
    let mut puzzle_components = lines.split(|l| l.eq(""));

    let ordering_rules = puzzle_components.next().unwrap();
    let print_updates = puzzle_components.next().unwrap();

    let rule_set = OrderRules::from(ordering_rules);
    let updates: Vec<PrintUpdate> = print_updates.iter().map(|l| PrintUpdate::from(l)).collect();

    let result: i64 = updates.iter()
        .filter(|pu| {
            pu.is_order_valid(&rule_set)
        })
        .map(|pu| {
            pu.middle_page()
        })
        .sum();

    result as usize
}

struct OrderRules {
    rules: HashMap<i64, Vec<i64>>
}

impl OrderRules {
    pub fn from(lines: &[String]) -> Self {
        let mut rules: HashMap<i64, Vec<i64>> = HashMap::new();

        lines.iter().for_each(|line| {
            let mut components = line.split("|");
            let before = components.next().unwrap().parse::<i64>().unwrap();
            let after = components.next().unwrap().parse::<i64>().unwrap();

            if !rules.contains_key(&before) {
                rules.insert(before.clone(), vec![]);
            }

            let rules_for_key = rules.get_mut(&before).unwrap();
            rules_for_key.push(after);
        });

        OrderRules { rules }
    }
}

struct PrintUpdate {
    pages: Vec<i64>
}

impl PrintUpdate {
    pub fn from(line: &String) -> Self {
        let pages = line.split(',')
            .map(|s| s.parse::<i64>().unwrap())
            .collect();

        PrintUpdate { pages }
    }

    pub fn is_order_valid(&self, rules: &OrderRules) -> bool {
        for (index, page) in self.pages.iter().enumerate() {
            // update is invalid if any page which should come after
            // current page is found before this page

            if let Some(rule_set) = rules.rules.get(&page) {
                let prev_pages = self.pages.get(0..index).unwrap();

                let order_violation = rule_set.iter().any(|after_page| {
                   prev_pages.contains(after_page)
                });
                if order_violation {
                    return false;
                }
            } // else: for current page, there is no rule set we can check
        }

        true
    }

    pub fn middle_page(&self) -> &i64 {
        if self.pages.len() % 2 == 0 {
            panic!("Page set has even number of pages. There is no middle page");
        }

        self.pages.get(self.pages.len() / 2).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part() {
        let result = process(&read_file(&String::from("../test-input")));

        assert_eq!(result, 143);
    }
}
