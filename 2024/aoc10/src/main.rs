use std::{
    collections::HashSet,
    fs::File,
    io::{self, BufRead},
};

fn main() {
    let lines = read_lines("input/input.txt");
    let mut input: Vec<Vec<usize>> = Vec::new();
    for line in lines.flatten() {
        input.push(
            line.chars()
                .map(|x| x.to_string().parse::<usize>().unwrap())
                .collect(),
        );
    }

    let (mut part_1, mut part_2) = (0, 0);
    for i in 0..input.len() {
        for j in 0..input[0].len() {
            if input[i][j] == 0 {
                let mut set = HashSet::new();
                let r = trailhead(&input, i, j, 1, &mut set);
                part_1 += r.0;
                part_2 += r.1;
            }
        }
    }

    println!("Part 1: {part_1:?}");
    println!("Part 2: {part_2:?}");
}

fn trailhead(
    input: &Vec<Vec<usize>>,
    i: usize,
    j: usize,
    find: usize,
    set: &mut HashSet<(usize, usize)>,
) -> (usize, usize) {
    let mut sum = (0, 0);
    if input[i][j] == 9 && find == 10 {
        if set.insert((i, j)) {
            return (1, 1);
        } else {
            return (0, 1);
        }
    }
    if find == 10 {
        return (0, 0);
    }
    if i > 0 && input[i - 1][j] == find {
        let r = trailhead(input, i - 1, j, find + 1, set);
        sum.0 += r.0;
        sum.1 += r.1;
    }
    if i < input.len() - 1 && input[i + 1][j] == find {
        let r = trailhead(input, i + 1, j, find + 1, set);
        sum.0 += r.0;
        sum.1 += r.1;
    }
    if j > 0 && input[i][j - 1] == find {
        let r = trailhead(input, i, j - 1, find + 1, set);
        sum.0 += r.0;
        sum.1 += r.1;
    }
    if j < input[0].len() - 1 && input[i][j + 1] == find {
        let r = trailhead(input, i, j + 1, find + 1, set);
        sum.0 += r.0;
        sum.1 += r.1;
    }

    sum
}

fn read_lines(filename: &str) -> io::Lines<io::BufReader<File>> {
    let file = File::open(filename).expect("Could not open file");
    io::BufReader::new(file).lines()
}
