use std::{
    collections::VecDeque,
    fs::File,
    io::{self, BufRead},
    usize,
};

fn main() {
    let lines = read_lines("input/input.txt");
    let mut p1 = 0;
    let mut p2 = 0;

    for line in lines.flatten() {
        let line = line.replace(":", "");
        let nums = line.split(" ").collect::<Vec<_>>();
        let ans = nums[0].parse::<usize>().unwrap();

        let mut operands = VecDeque::with_capacity(nums.len() - 1);
        for i in 1..nums.len() {
            operands.push_back(nums[i].parse::<usize>().unwrap());
        }

        if eval(&operands, ans, &vec!["+", "*"]) {
            p1 += ans;
        }

        if eval(&operands, ans, &vec!["+", "*", "||"]) {
            p2 += ans;
        }
    }

    println!("Part 1: {p1}");
    println!("Part 2: {p2}");
}

fn eval(operands: &VecDeque<usize>, target: usize, ops: &Vec<&str>) -> bool {
    if operands.len() == 1 {
        return operands[0] == target;
    }

    let mut equal = false;
    let mut cloned = operands.clone();
    let x = cloned.pop_front().unwrap();
    let y = cloned.pop_front().unwrap();
    for op in ops {
        let res = apply(op, x, y);
        cloned.push_front(res);
        equal |= eval(&cloned, target, ops);
        cloned.pop_front();
    }

    equal
}

fn apply(op: &str, x: usize, y: usize) -> usize {
    if op == "+" {
        x + y
    } else if op == "*" {
        x * y
    } else {
        format!("{x}{y}").parse::<usize>().unwrap()
    }
}

fn read_lines(filename: &str) -> io::Lines<io::BufReader<File>> {
    let file = File::open(filename).expect("Could not open file");
    io::BufReader::new(file).lines()
}
