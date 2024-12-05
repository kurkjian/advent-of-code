use std::{
    collections::HashMap,
    fs::File,
    io::{self, BufRead},
};

fn main() {
    let mut lines = read_lines("input/input.txt").flatten();
    let mut rules = HashMap::new();

    while let Some(line) = lines.next() {
        if line.is_empty() {
            break;
        }

        let (left, right) = line
            .split_once('|')
            .map(|(l, r)| (l.to_owned(), r.to_owned()))
            .unwrap();
        rules
            .entry(left)
            .and_modify(|v: &mut Vec<String>| v.push(right.clone()))
            .or_insert(vec![right]);
    }

    let mut valid = 0;
    let mut invalid = 0;
    while let Some(line) = lines.next() {
        let pages: Vec<String> = line.split(',').map(|s| s.to_string()).collect();

        if is_valid(&pages, &rules) {
            valid += pages[pages.len() / 2].parse::<usize>().unwrap();
        } else {
            invalid += make_valid(&pages, &rules);
        }
    }
    println!("Part 1: {valid:?}");
    println!("Part 2: {invalid:?}");
}

fn is_valid(pages: &Vec<String>, rules: &HashMap<String, Vec<String>>) -> bool {
    let mut seen = Vec::with_capacity(pages.len());
    for page in pages {
        let rule = rules.get(page);
        if let Some(v) = rule {
            for s in v {
                if seen.contains(s) {
                    return false;
                }
            }
        }

        seen.push(page.to_string());
    }
    return true;
}

fn make_valid(pages: &Vec<String>, rules: &HashMap<String, Vec<String>>) -> usize {
    let mut seen = Vec::with_capacity(pages.len());
    'outer: for page in pages {
        let rule = rules.get(page);
        if let Some(v) = rule {
            for s in v {
                if seen.contains(s) {
                    for i in 0..seen.len() {
                        let mut clone = seen.clone();
                        clone.insert(i, page.to_owned());
                        if is_valid(&clone, rules) {
                            seen = clone;
                            continue 'outer;
                        }
                    }
                }
            }
        }

        seen.push(page.to_string());
    }
    return seen[seen.len() / 2].parse::<usize>().unwrap();
}

fn read_lines(filename: &str) -> io::Lines<io::BufReader<File>> {
    let file = File::open(filename).expect("Could not open file");
    io::BufReader::new(file).lines()
}
