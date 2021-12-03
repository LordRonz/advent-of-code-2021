use std::{
    fs::File,
    io::{prelude::*, BufReader},
    path::Path,
};

fn lines_from_file(filename: impl AsRef<Path>) -> Vec<i64> {
    let file = File::open(filename).expect("no such file");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|line| line.unwrap().parse::<i64>().unwrap())
        .collect()
}

pub fn run() {
    println!("Day 01:");
    let lines = lines_from_file("./src/day01/input");
    // Part A
    let mut res = 0;
    let mut prev = -1;
    for line in lines.iter() {
        if *line > prev && prev != -1 {
            res += 1;
        }
        prev = *line;
    }
    println!("{}", res);

    // Part B
    res = 0;

    for i in 0..lines.len() - 3 {
        if lines[i + 1] + lines[i + 2] + lines[i + 3] > lines[i] + lines[i + 1] + lines[i + 2] {
            res += 1;
        }
    }
    println!("{}", res);
}
