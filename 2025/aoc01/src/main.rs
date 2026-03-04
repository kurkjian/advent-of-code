use std::{
    fs::File,
    io::{self, BufRead},
};

fn main() {
    let lines = read_lines("input/input.txt");

    let mut pos = 50;
    let mut counter = 0;
    for line in lines.flatten() {
        let mut chars = line.chars();
        let dir = chars.next().expect("Missing direction");
        let rot = chars.as_str().parse::<i32>().expect("Invalid number");

        match dir {
            'L' => pos = (pos - rot).rem_euclid(100),
            'R' => pos = (pos + rot).rem_euclid(100),
            _ => panic!("Invalid direction"),
        }

        if pos == 0 {
            counter += 1;
        }
    }

    println!("Final count @ 0: {}", counter);
}

fn read_lines(filename: &str) -> io::Lines<io::BufReader<File>> {
    let file = File::open(filename).expect("Could not open file");
    io::BufReader::new(file).lines()
}
