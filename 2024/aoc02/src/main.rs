use std::{
    collections::HashSet,
    fs::File,
    io::{self, BufRead},
};

use either::Either;

fn main() {
    let f_name = "input/input.txt";
    let mut safe = HashSet::new();

    walk(f_name, false, &mut safe);
    walk(f_name, true, &mut safe);

    println!("Safe: {}", safe.len());
}

fn walk(f_name: &str, rev: bool, safe: &mut HashSet<usize>) {
    let lines = read_lines(&f_name);
    'outer: for (idx, line) in lines.flatten().enumerate() {
        let mut iter = if rev {
            Either::Left(line.split_whitespace().rev())
        } else {
            Either::Right(line.split_whitespace())
        };
        let mut prev = iter.next().unwrap().parse::<i32>().unwrap();
        let mut dir = 0;
        let mut damped = false;

        while let Some(x) = iter.next().map(|x| x.parse::<i32>().unwrap()) {
            let diff = prev - x;
            let abs = diff.abs();
            if abs < 1 || abs > 3 {
                if !damped {
                    damped = true;
                    continue;
                }
                continue 'outer;
            }
            if dir == 0 {
                dir = diff / abs;
            }

            if diff / abs != dir {
                if !damped {
                    damped = true;
                    continue;
                }
                continue 'outer;
            }

            prev = x;
        }

        safe.insert(idx);
    }
}

fn read_lines(filename: &str) -> io::Lines<io::BufReader<File>> {
    let file = File::open(filename).expect("Could not open file");
    io::BufReader::new(file).lines()
}
