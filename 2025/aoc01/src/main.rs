use std::{
    fs::File,
    io::{self, BufRead},
};

fn main() {
    let lines = read_lines("input/input.txt");

    let mut pos = 50;
    let mut finished_zero = 0;
    let mut hit_zero = 0;
    for line in lines.flatten() {
        let mut chars = line.chars();
        let dir = chars.next().expect("Missing direction");
        let rot = chars.as_str().parse::<i32>().expect("Invalid number");

        let spins = rot / 100;
        let rot = rot % 100;

        match dir {
            'L' => {
                if pos - rot < 0 && pos > 0 {
                    hit_zero += 1;
                }
                pos = (pos - rot).rem_euclid(100);
            }
            'R' => {
                if pos + rot > 100 {
                    hit_zero += 1;
                }
                pos = (pos + rot).rem_euclid(100);
            }
            _ => panic!("Invalid direction"),
        }

        if pos == 0 {
            finished_zero += 1;
        }

        hit_zero += spins;
    }

    println!("Final count @ 0: {}", finished_zero);
    println!("Clicked zero: {}", hit_zero + finished_zero);
}

fn read_lines(filename: &str) -> io::Lines<io::BufReader<File>> {
    let file = File::open(filename).expect("Could not open file");
    io::BufReader::new(file).lines()
}
