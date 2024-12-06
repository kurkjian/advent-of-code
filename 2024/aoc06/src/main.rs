use std::{
    fs::File,
    io::{self, BufRead},
};

#[derive(Clone, Copy, PartialEq)]
enum Direction {
    N,
    E,
    S,
    W,
}

fn main() {
    let lines = read_lines("input/input.txt");
    let mut input: Vec<Vec<char>> = Vec::new();

    for line in lines.flatten() {
        input.push(line.chars().collect());
    }

    part_1(&input);
    part_2(&input);
}

fn part_1(input: &Vec<Vec<char>>) {
    let mut cloned= input.clone();

    for i in 0..input.len() {
        for j in 0..input[0].len() {
            let c = input[i][j];
            if c == '^' {
                walk(i, j, input, &mut cloned);
            }
        }
    }

    let mut x = 0;
    for i in 0..cloned.len() {
        for j in 0..cloned[0].len() {
            let c = cloned[i][j];
            if c == 'X' {
                x += 1;
            }
        }
    }

    println!("Part 1: {x}");
}

fn walk(mut i: usize, mut j: usize, input: &Vec<Vec<char>>, cloned: &mut Vec<Vec<char>>) -> bool {
    let timeout = 100_000;
    let mut direction = Direction::N;
    let mut iter = 0;
    loop {
        iter += 1;
        cloned[i][j] = 'X';
        if left(i, j, input, direction) {
            return false;
        }
        if iter == timeout {
            return true;
        }

         let (di, dj) = step(i, j, direction);
        if input[di][dj] == '.' || input[di][dj] == '^' {
            i = di;
            j = dj;
        } else {
            direction = match direction {
                Direction::N => Direction::E,
                Direction::E => Direction::S,
                Direction::S => Direction::W,
                Direction::W => Direction::N,
            }
        }
    }
}

fn left(i: usize, j: usize, input: &Vec<Vec<char>>, dir: Direction) -> bool {
    if dir == Direction::N && i == 0 {
        true
    } else if dir == Direction::E && j == input[0].len() - 1 {
        true
    } else if dir == Direction::S && i == input.len() - 1 {
        true
    } else if dir == Direction::W && j == 0 {
        true
    } else {
        false
    }
}

fn step(i: usize, j: usize, dir: Direction) -> (usize, usize) {
    if dir == Direction::N {
        (i - 1, j)
    } else if dir == Direction::E {
        (i, j + 1)
    } else if dir == Direction::S {
        (i + 1, j)
    } else{
        (i, j - 1)
    }
}

/// 🙈
fn part_2(input: &Vec<Vec<char>>) {
    let mut start_i = 0;
    let mut start_j = 0;
    'outer: for i in 0..input.len() {
        for j in 0..input[0].len() {
            let c = input[i][j];
            if c == '^' {
                start_i = i;
                start_j = j;
                break 'outer;
            }
        }
    }

    let mut sum = 0;

    for i in 0..input.len() {
        for j in 0..input[0].len() {
            let c = input[i][j];
            if c == '.' {
                let mut attempt = input.clone();
                attempt[i][j] = '#';

                if walk(start_i, start_j, &attempt, &mut attempt.clone()) {
                    sum += 1;
                }
            }
        }
    }

    println!("Part 2: {sum}");
}


fn read_lines(filename: &str) -> io::Lines<io::BufReader<File>> {
    let file = File::open(filename).expect("Could not open file");
    io::BufReader::new(file).lines()
}
