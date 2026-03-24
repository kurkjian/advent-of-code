use std::{
    fs::File,
    io::{self, BufRead},
};

fn main() {
    let lines = read_lines("input/input.txt");
    let lines: Vec<_> = lines.flatten().collect();

    let mut ranges = Vec::new();
    for line in lines.iter() {
        if line.is_empty() {
            break;
        }

        let (start, end) = line.split_once('-').unwrap();
        let start = start.parse::<usize>().unwrap();
        let end = end.parse::<usize>().unwrap();

        let range = start..=end;
        ranges.push(range);
    }

    ranges.sort_by_key(|r| *r.start());

    let mut reduced = Vec::with_capacity(ranges.len());
    let mut prev_end = 0;
    for range in ranges.iter() {
        if *range.start() > prev_end {
            reduced.push(range.clone());
        } else {
            let last = reduced.last_mut().unwrap();

            let s = *last.start();
            let e = std::cmp::max(*last.end(), *range.end());
            *last = s..=e;
        }

        prev_end = *range.end();
    }

    let mut sum = 0;
    for range in reduced.into_iter() {
        sum += *range.end() - *range.start() + 1;
    }
    println!("Count of all possible fresh IDs: {sum}");

    let mut count = 0;
    for line in lines {
        if let Ok(id) = line.parse::<usize>() {
            for range in ranges.iter() {
                if range.contains(&id) {
                    count += 1;
                    break;
                }
            }
        }
    }

    println!("Fresh ids: {count}");
}

fn read_lines(filename: &str) -> io::Lines<io::BufReader<File>> {
    let file = File::open(filename).expect("Could not open file");
    io::BufReader::new(file).lines()
}
