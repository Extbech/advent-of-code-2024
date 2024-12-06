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

    fn part_one(&self) -> u16 {
        self.pages
            .iter()
            .filter_map(|page| {
                if page_is_correct(page, &self.rules) {
                    Some(page[page.len() / 2] as u16)
                } else {
                    None
                }
            })
            .sum()
    }

    fn part_two(&self) -> u16 {
        let mut count = 0;
        let mut buf = Vec::new();
        for page in find_incorrect_pages(&self.pages, &self.rules) {
            count += get_fixed_page_middle(&mut buf, page, &self.rules);
        }
        count
    }
}

fn parse_input(input: String) -> (Vec<Vec<u8>>, HashMap<u8, Vec<u8>>) {
    let mut rules: HashMap<u8, Vec<u8>> = HashMap::new();
    let mut pages = Vec::new();

    let mut lines = input.lines();
    for line in lines.by_ref() {
        if line.is_empty() {
            break;
        }
        let parts: Vec<u8> = line.split('|').map(|x| x.parse::<u8>().unwrap()).collect();
        if let std::collections::hash_map::Entry::Vacant(e) = rules.entry(parts[0]) {
            e.insert(vec![parts[1]]);
        } else {
            rules.get_mut(&parts[0]).unwrap().push(parts[1]);
        }
    }

    for line in lines {
        pages.push(line.split(',').map(|s| s.parse::<u8>().unwrap()).collect());
    }

    (pages, rules)
}

fn find_incorrect_pages<'a, 'b: 'a>(
    pages: &'a [Vec<u8>],
    rules: &'b HashMap<u8, Vec<u8>>,
) -> impl Iterator<Item = &'a [u8]> + 'a {
    pages.iter().filter_map(|page| {
        for j in 1..page.len() {
            if let Some(rule) = rules.get(&page[j]) {
                if page[0..j].iter().any(|x| rule.contains(x)) {
                    return Some(page.as_ref());
                }
            }
        }
        None
    })
}

fn get_fixed_page_middle(buf: &mut Vec<u8>, page: &[u8], rules: &HashMap<u8, Vec<u8>>) -> u16 {
    buf.clear();
    buf.extend_from_slice(page);
    fix_page(buf, rules);
    buf[buf.len() / 2] as u16
}

fn fix_page(buf: &mut [u8], rules: &HashMap<u8, Vec<u8>>) {
    let mut i = buf.len() - 1;
    loop {
        if let Some(rule) = rules.get(&buf[i]) {
            if let Some(fault) = buf[..i].iter().position(|x| rule.contains(x)) {
                buf.swap(i, fault);
            } else {
                if i == 0 {
                    return;
                }
                i -= 1;
            }
        }
    }
}

fn page_is_correct(page: &[u8], rules: &HashMap<u8, Vec<u8>>) -> bool {
    for i in 1..page.len() {
        if let Some(rule) = rules.get(&page[i]) {
            if page[0..i].iter().any(|x| rule.contains(x)) {
                return false;
            }
        }
    }
    true
}
