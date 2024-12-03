use std::fs::{self};

use regex::Regex;

fn main() {
    let content = fs::read_to_string("input/input.txt").unwrap();

    let re = Regex::new(r"mul\((?<left>\d{1,3}),(?<right>\d{1,3})\)").unwrap();
    let re_do = Regex::new(r"do\(\)").unwrap();
    let re_dont = Regex::new(r"don't\(\)").unwrap();

    let mut enabled = Vec::new();
    enabled.push((0, true));
    let mut idx = 0;
    let mut e = true;

    let mut part_1 = 0;
    let mut part_2 = 0;
    re_do.captures_iter(&content).for_each(|c| {
        enabled.push((c.get(0).unwrap().start(), true));
    });
    re_dont.captures_iter(&content).for_each(|c| {
        enabled.push((c.get(0).unwrap().start(), false));
    });

    enabled.sort_by(|a, b| a.0.cmp(&b.0));

    for (left, right, index) in re.captures_iter(&content).map(|c| {
        let (_, [left, right]) = c.extract();
        let index = c.get(0).unwrap().start();

        (left, right, index)
    }) {
        if idx != enabled.len() - 1 && index > enabled[idx + 1].0 {
            e = enabled[idx + 1].1;
            idx += 1;
        }

        part_1 += left.parse::<i32>().unwrap() * right.parse::<i32>().unwrap();
        if e {
            part_2 += left.parse::<i32>().unwrap() * right.parse::<i32>().unwrap();
        }
    }

    println!("Part 1: {part_1}");
    println!("Part 2: {part_2}");
}
