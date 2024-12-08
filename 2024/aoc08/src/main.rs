use std::{
    collections::{HashMap, HashSet},
    fs::File,
    io::{self, BufRead},
};

fn main() {
    let lines = read_lines("input/input.txt");
    let mut p1 = HashSet::new();
    let mut p2 = HashSet::new();

    let mut input: Vec<Vec<char>> = Vec::new();
    let mut nodes = HashMap::new();

    for line in lines.flatten() {
        input.push(line.chars().collect());
    }

    for (i, v) in input.iter().enumerate() {
        for (j, c) in v.iter().enumerate() {
            if *c != '.' {
                nodes
                    .entry(c)
                    .and_modify(|v: &mut Vec<(i32, i32)>| v.push((i as i32, j as i32)))
                    .or_insert(vec![(i as i32, j as i32)]);
            }
        }
    }

    for (_, v) in nodes.iter() {
        for (i, j) in v {
            for (x, y) in v {
                if i == x && j == y {
                    continue;
                }

                let mut n = 1;
                loop {
                    let antinode = (i + ((x - i) * n), j + ((y - j) * n));
                    if antinode.0 >= 0
                        && antinode.0 < input.len() as i32
                        && antinode.1 >= 0
                        && antinode.1 < input[0].len() as i32
                    {
                        if n == 2 {
                            p1.insert((antinode.0, antinode.1));
                        }

                        p2.insert((antinode.0, antinode.1));
                    } else {
                        break;
                    }
                    n += 1;
                }
            }
        }
    }

    println!("Part 1: {}", p1.len());
    println!("Part 2: {}", p2.len());
}

fn read_lines(filename: &str) -> io::Lines<io::BufReader<File>> {
    let file = File::open(filename).expect("Could not open file");
    io::BufReader::new(file).lines()
}
