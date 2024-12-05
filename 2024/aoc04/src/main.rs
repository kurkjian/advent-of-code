use std::{
    fs::File,
    io::{self, BufRead},
};

#[derive(Clone, Copy)]
enum Direction {
    N,
    NE,
    E,
    SE,
    S,
    SW,
    W,
    NW,
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
    let mut sum = 0;
    for i in 0..input.len() {
        for j in 0..input[0].len() {
            let c = input[i][j];
            if c == 'X' {
                sum += find_xmas(&input, i as i32, j as i32);
            }
        }
    }

    println!("Part 1: {sum}");
}

fn part_2(input: &Vec<Vec<char>>) {
    let mut sum = 0;
    for i in 0..input.len() {
        for j in 0..input[0].len() {
            let c = input[i][j];
            if c == 'A' {
                sum += find_x(&input, i, j);
            }
        }
    }

    println!("Part 2: {sum}");
}

fn find_x(input: &Vec<Vec<char>>, i: usize, j: usize) -> usize {
    if i == 0 || i == input.len() - 1 {
        return 0;
    }
    if j == 0 || j == input[0].len() - 1 {
        return 0;
    }

    if input[i - 1][j - 1] == 'M' && input[i + 1][j + 1] == 'S' {
        if input[i + 1][j - 1] == 'M' && input[i - 1][j + 1] == 'S' {
            return 1;
        }
        if input[i + 1][j - 1] == 'S' && input[i - 1][j + 1] == 'M' {
            return 1;
        }
    } else if input[i - 1][j - 1] == 'S' && input[i + 1][j + 1] == 'M' {
        if input[i + 1][j - 1] == 'M' && input[i - 1][j + 1] == 'S' {
            return 1;
        }
        if input[i + 1][j - 1] == 'S' && input[i - 1][j + 1] == 'M' {
            return 1;
        }
    }

    return 0;
}

fn find_xmas(input: &Vec<Vec<char>>, i: i32, j: i32) -> usize {
    walk(input, i - 1, j, Direction::N, next('X'))
        + walk(input, i - 1, j + 1, Direction::NE, next('X'))
        + walk(input, i, j + 1, Direction::E, next('X'))
        + walk(input, i + 1, j + 1, Direction::SE, next('X'))
        + walk(input, i + 1, j, Direction::S, next('X'))
        + walk(input, i + 1, j - 1, Direction::SW, next('X'))
        + walk(input, i, j - 1, Direction::W, next('X'))
        + walk(input, i - 1, j - 1, Direction::NW, next('X'))
}

fn walk(input: &Vec<Vec<char>>, i: i32, j: i32, dir: Direction, c: char) -> usize {
    if c == '\0' {
        return 1;
    }
    if i == -1 || i == input.len() as i32 {
        return 0;
    }
    if j == -1 || j == input[0].len() as i32 {
        return 0;
    }

    if input[i as usize][j as usize] == c {
        walk(input, in_dir(i, 0, dir), in_dir(j, 1, dir), dir, next(c))
    } else {
        0
    }
}

/// z == 0 -> first dim
/// z != 1 -> second dim
fn in_dir(x: i32, z: usize, dir: Direction) -> i32 {
    if z == 0 {
        match dir {
            Direction::N | Direction::NE | Direction::NW => x - 1,
            Direction::E | Direction::W => x,
            Direction::SE | Direction::S | Direction::SW => x + 1,
        }
    } else {
        match dir {
            Direction::N | Direction::S => x,
            Direction::NE | Direction::E | Direction::SE => x + 1,
            Direction::SW | Direction::W | Direction::NW => x - 1,
        }
    }
}

fn next(c: char) -> char {
    match c {
        'X' => 'M',
        'M' => 'A',
        'A' => 'S',
        'S' => '\0',
        _ => panic!("Invalid char {c}"),
    }
}

fn read_lines(filename: &str) -> io::Lines<io::BufReader<File>> {
    let file = File::open(filename).expect("Could not open file");
    io::BufReader::new(file).lines()
}
