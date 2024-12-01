use std::{
    collections::{BinaryHeap, HashMap},
    fs::File,
    io::{self, BufRead},
};

fn main() {
    let lines = read_lines("input/input.txt");
    let mut left = BinaryHeap::new();
    let mut right = BinaryHeap::new();
    let mut right_occurrence = HashMap::new();

    for line in lines.flatten() {
        let mut iter = line.split_whitespace();
        let first = iter.next().unwrap().parse::<u64>().unwrap();
        let second = iter.next().unwrap().parse::<u64>().unwrap();

        left.push(first);
        right.push(second);
        right_occurrence
            .entry(second)
            .and_modify(|x| *x += 1)
            .or_insert(1);
    }

    let mut total = 0;
    let mut similarity = 0;
    while !left.is_empty() {
        let l = left.pop().unwrap();
        let r = right.pop().unwrap();
        let diff = l.abs_diff(r);

        total += diff;
        similarity += l * right_occurrence.get(&l).unwrap_or(&0);
    }

    println!("Total: {total}");
    println!("Similarity: {similarity}");
}

fn read_lines(filename: &str) -> io::Lines<io::BufReader<File>> {
    let file = File::open(filename).expect("Could not open file");
    io::BufReader::new(file).lines()
}
