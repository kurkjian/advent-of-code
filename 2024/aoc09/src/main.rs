use std::{
    collections::HashSet,
    fs::File,
    io::{self, BufRead},
};

fn main() {
    let lines = read_lines("input/input.txt");

    let disk = lines.flatten().next().unwrap();
    let mut v = Vec::new();
    let mut x = true;
    let mut id = 0;
    for d in disk.chars() {
        if x {
            for _ in 0..d.to_digit(10).unwrap() {
                v.push(id.to_string());
            }
            id += 1;
        } else {
            for _ in 0..d.to_digit(10).unwrap() {
                v.push("x".to_string());
            }
        }
        x = !x;
    }

    part1(v.clone());
    part2(v);
}

fn part1(mut fs: Vec<String>) {
    let mut front_ptr = 0;
    let mut back_ptr = fs.len() - 1;
    while front_ptr <= back_ptr {
        while fs[front_ptr] != "x" {
            front_ptr += 1;
        }
        while fs[back_ptr] == "x" {
            back_ptr -= 1;
        }

        fs[front_ptr] = fs[back_ptr].clone();
        fs[back_ptr] = "x".to_string();
    }

    fs[back_ptr] = fs[front_ptr].clone();
    fs[front_ptr] = "x".to_string();

    let mut p1 = 0;
    for (i, s) in fs.iter().enumerate() {
        if let Ok(d) = s.parse::<usize>() {
            p1 += i * d;
        } else {
            break;
        }
    }

    println!("Part 1: {p1:?}");
}

fn part2(mut fs: Vec<String>) {
    let mut idx = fs.len() - 1;
    let mut moved = HashSet::new();
    while fs[idx] != "0" {
        let id = fs[idx].clone();
        let mut n = 1;
        while fs[idx - n] == id {
            n += 1;
        }

        if fs[idx] == "x" {
            idx -= n;
            continue;
        }

        if moved.contains(&id) {
            idx -= n;
            continue;
        }

        let mut test = 0;
        let mut found = false;

        let mut i = 0;
        'outer: while i < fs.len() {
            if fs[i] == "x" {
                for j in 1..n {
                    if i + j > fs.len() - 1 {
                        break 'outer;
                    }
                    if fs[i + j] != "x" {
                        i += j;
                        continue 'outer;
                    }
                }
                found = true;
                test = i;
                break 'outer;
            }
            i += 1;
        }

        moved.insert(id.clone());
        if found {
            if test >= idx {
                idx -= n;
                continue;
            }
            for i in test..(test + n) {
                fs[i] = id.clone();
            }

            for _ in idx - n..idx {
                fs[idx] = "x".to_string();
                idx -= 1;
            }
        } else {
            idx -= n;
        }
    }

    let mut p2 = 0;
    for (i, s) in fs.iter().enumerate() {
        if let Ok(d) = s.parse::<usize>() {
            p2 += i * d;
        }
    }

    println!("Part 2: {p2:?}");
}

fn read_lines(filename: &str) -> io::Lines<io::BufReader<File>> {
    let file = File::open(filename).expect("Could not open file");
    io::BufReader::new(file).lines()
}
