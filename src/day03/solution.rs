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
    println!("Day 03:");
    // let lines = lines_from_file("./src/day03/input");
    // Part A

    // Part B
}
