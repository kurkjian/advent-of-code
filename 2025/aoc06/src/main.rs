use std::{
    fs::File,
    io::{self, BufRead},
};

fn main() {
    let lines = read_lines("input/input.txt");

    let mut grid = Vec::new();
    for line in lines.flatten() {
        grid.push(
            line.split(" ")
                .map(|s| s.to_owned())
                .filter(|s| !s.is_empty())
                .collect::<Vec<String>>(),
        );
    }

    let mut total = 0;
    for j in 0..grid[0].len() {
        let mut operands = Vec::with_capacity(grid.len());
        for i in 0..(grid.len() - 1) {
            operands.push(grid[i][j].parse::<usize>().unwrap());
        }

        let operator = grid[grid.len() - 1][j].parse::<char>().unwrap();
        let mut sum = 1;
        for operand in operands {
            if operator == '+' {
                sum += operand;
            } else if operator == '*' {
                sum *= operand;
            }
        }

        if operator == '+' {
            sum -= 1;
        }

        total += sum;
    }

    println!("Grand total: {}", total);
}

fn read_lines(filename: &str) -> io::Lines<io::BufReader<File>> {
    let file = File::open(filename).expect("Could not open file");
    io::BufReader::new(file).lines()
}
