use std::{
    fs::File,
    io::{self, BufRead},
};

fn main() {
    let lines = read_lines("input/input.txt");

    let mut bat2 = 0;
    let mut bat12 = 0;
    for line in lines.flatten() {
        let digits = line
            .chars()
            .map(|d| d.to_digit(10).unwrap())
            .collect::<Vec<_>>();

        bat2 += max_joltage(&digits, 2);
        bat12 += max_joltage(&digits, 12);
    }

    println!("Total joltage (2 battery): {}", bat2);
    println!("Total joltage (12 battery): {}", bat12);
}

fn read_lines(filename: &str) -> io::Lines<io::BufReader<File>> {
    let file = File::open(filename).expect("Could not open file");
    io::BufReader::new(file).lines()
}

fn max_joltage(digits: &[u32], num_batteries: usize) -> usize {
    let mut indicies = Vec::with_capacity(num_batteries);
    let mut max_idx = 0;
    for (idx, d) in digits
        .iter()
        .take(digits.len() - num_batteries + 1)
        .enumerate()
    {
        if d > &digits[max_idx] {
            max_idx = idx;
        }
    }
    indicies.push(max_idx);

    for n in 1..num_batteries {
        let mut next_max = indicies[indicies.len() - 1] + 1;
        for (idx, d) in digits
            .iter()
            .skip(indicies[indicies.len() - 1] + 1)
            .enumerate()
        {
            if d > &digits[next_max] {
                next_max = idx + indicies[indicies.len() - 1] + 1;
            }

            if idx + indicies[indicies.len() - 1] + 1 + (num_batteries - n) == digits.len() {
                break;
            }
        }

        indicies.push(next_max);
    }

    let mut pow = indicies.len() as u32;
    let mut joltage = 0;
    for d in indicies.into_iter() {
        pow -= 1;
        joltage += digits[d] as usize * 10_usize.pow(pow);
    }
    joltage
}
