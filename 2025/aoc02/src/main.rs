use std::{
    fs::File,
    io::{self, BufRead},
};

fn main() {
    let lines = read_lines("input/input.txt");
    let line = lines.flatten().next().unwrap();
    let ranges = line.split(',').collect::<Vec<_>>();

    let mut invalid_sum = 0;
    for range in ranges.into_iter() {
        let (start, end) = range.split_once('-').expect("Invalid range");
        let start = start.parse::<usize>().expect("Invalid start");
        let end = end.parse::<usize>().expect("Invalid end");

        for i in start..=end {
            let as_str = i.to_string();
            if as_str.len() % 2 == 1 {
                continue;
            }
            let (left, right) = as_str.split_at(as_str.len() / 2);
            if left == right {
                invalid_sum += i;
            }
        }
    }

    println!("Invalid sum: {invalid_sum}");
}

fn read_lines(filename: &str) -> io::Lines<io::BufReader<File>> {
    let file = File::open(filename).expect("Could not open file");
    io::BufReader::new(file).lines()
}
