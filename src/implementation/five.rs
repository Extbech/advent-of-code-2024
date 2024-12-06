use std::collections::HashMap;

use crate::Solution;

pub struct DayFiveSolution {
    pages: Vec<Vec<u8>>,
    rules: HashMap<u8, Vec<u8>>,
}

impl Solution for DayFiveSolution {
    const DAY: u8 = 5;

    fn new() -> Self {
        let parsed_input = parse_input(Self::read_data_to_string().unwrap());
        DayFiveSolution {
            pages: parsed_input.0,
            rules: parsed_input.1,
        }
    }

    /// Needs refactoring !!!
    fn part_one(&self) -> u16 {
        self.pages.iter().filter_map(|page| {
                for i in 1..page.len() {
                    if let Some(rule) = self.rules.get(&page[i]) {
                        if page[0..i].iter().any(|x| rule.contains(x)) {
                            return None
                        }
                    }
                }
                return Some(page[page.len() / 2] as u16);
            }
        ).sum()
    }

    fn part_two(&self) -> u16 {
        let mut wrong_pages = find_incorrect_pages(&self.pages, &self.rules);
        let mut count = 0;
        for page in wrong_pages.iter_mut() {
            count += get_fixed_page_sum(page, &self.rules);
        }
        count
    }
}

fn parse_input(input: String) -> (Vec<Vec<u8>>, HashMap<u8, Vec<u8>>) {
    let mut rules: HashMap<u8, Vec<u8>> = HashMap::new();
    let mut pages = Vec::new();

    let mut lines = input.lines();
    while let Some(line) = lines.next() {
        if line.is_empty() {
            break;
        }
        let parts: Vec<u8> = line.split("|").map(|x| x.parse::<u8>().unwrap()).collect();
        if rules.contains_key(&parts[0]) {
            rules.get_mut(&parts[0]).unwrap().push(parts[1]);
        } else {
            rules.insert(parts[0], vec![parts[1]]);
        }
    }

    for line in lines {
        pages.push(line.split(",").map(|s| s.parse::<u8>().unwrap()).collect());
    }

    (pages, rules)
}

fn find_incorrect_pages(pages: &Vec<Vec<u8>>, rules: &HashMap<u8, Vec<u8>>) -> Vec<Vec<u8>> {
    let mut res: Vec<Vec<u8>> = vec![];
    for i in 0..pages.len() {
        for j in 1..pages[i].len() {
            if let Some(rule) = rules.get(&pages[i][j]) {
                if pages[i][0..j].iter().any(|x| rule.contains(x)) {
                    res.push(pages[i].clone());
                    break;
                }
            }
        }
    }
    res
}

fn get_fixed_page_sum(page: &mut Vec<u8>, rules: &HashMap<u8, Vec<u8>>) -> u16 {
    // Early check for middle is correct.
    if let Some(rule) = rules.get(&page[page.len() / 2]) {
        if !page[0..page.len() / 2].iter().any(|x| rule.contains(x))
            && !page[page.len() / 2..page.len()]
                .iter()
                .any(|x| rules.get(x).unwrap().contains(&page[page.len() / 2]))
        {
            return page[page.len() / 2] as u16;
        }
    }
    while !page_is_correct(page, rules) {
        for i in 1..page.len() {
            if let Some(rule) = rules.get(&page[i]) {
                if let Some(fault) = page.iter().position(|x| rule.contains(x)) {
                    let intermediate = page[i];
                    page[i] = page[fault];
                    page[fault] = intermediate;
                }
            }
        }
    }
    page[page.len() / 2] as u16
}

fn page_is_correct(page: &Vec<u8>, rules: &HashMap<u8, Vec<u8>>) -> bool {
    for i in 1..page.len() {
        if let Some(rule) = rules.get(&page[i]) {
            if page[0..i].iter().any(|x| rule.contains(x)) {
                return false;
            }
        }
    }
    true
}
