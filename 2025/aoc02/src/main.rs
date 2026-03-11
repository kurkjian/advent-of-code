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

        'outer: for i in start..=end {
            let as_str = i.to_string();

            'length: for j in 1..as_str.len() {
                if as_str.len() % j != 0 {
                    continue 'length;
                }

                let substr = as_str[0..j].to_string();
                for k in (j..as_str.len()).step_by(j) {
                    // println!("DEBUG: len: {j}, k: {k}");
                    let other_substr = as_str[k..k + j].to_string();
                    if substr != other_substr {
                        continue 'length;
                    }
                }

                invalid_sum += i;
                continue 'outer;
            }
            // let (left, right) = as_str.split_at(as_str.len() / 2);
            // if left == right {
            //     invalid_sum += i;
            // }
        }
    }

    println!("Invalid sum: {invalid_sum}");
}

fn read_lines(filename: &str) -> io::Lines<io::BufReader<File>> {
    let file = File::open(filename).expect("Could not open file");
    io::BufReader::new(file).lines()
}
