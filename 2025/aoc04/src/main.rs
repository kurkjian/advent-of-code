use std::{
    fs::File,
    io::{self, BufRead},
};

const ADJ: [(i32, i32); 8] = [
    (-1, -1),
    (-1, 0),
    (-1, 1),
    (0, -1),
    (0, 1),
    (1, -1),
    (1, 0),
    (1, 1),
];

fn main() {
    let lines = read_lines("input/input.txt");

    let mut grid = Vec::new();
    for line in lines.flatten() {
        let v = line.chars().collect::<Vec<_>>();
        grid.push(v);
    }

    let mut accessible = 0;

    loop {
        let mut cloned = grid.clone();
        for (r, row) in grid.iter().enumerate() {
            for (c, char) in row.iter().enumerate() {
                if *char != '@' {
                    continue;
                }

                let mut paper = 0;
                for (i, j) in ADJ {
                    if grid.get((r as i32 + i) as usize).is_some()
                        && grid[(r as i32 + i) as usize]
                            .get((c as i32 + j) as usize)
                            .is_some()
                    {
                        if grid[(r as i32 + i) as usize][(c as i32 + j) as usize] == '@' {
                            paper += 1;
                        }
                    }
                }

                if paper < 4 {
                    cloned[r][c] = '.';
                    accessible += 1;
                }
            }
        }

        if cloned == grid {
            break;
        }

        grid = cloned;
        println!("Total accessible: {accessible}");
    }
}

fn read_lines(filename: &str) -> io::Lines<io::BufReader<File>> {
    let file = File::open(filename).expect("Could not open file");
    io::BufReader::new(file).lines()
}
