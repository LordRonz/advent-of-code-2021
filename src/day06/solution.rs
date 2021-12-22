use std::{
    fs::File,
    io::{prelude::*, BufReader},
    path::Path,
};

fn parse_input(filename: impl AsRef<Path>) -> Vec<usize> {
    let file = File::open(filename).expect("no such file");
    let mut buf = BufReader::new(file);
    let mut s = String::new();
    buf.read_line(&mut s).unwrap();

    s.split(",")
        .filter(|s| s.parse::<usize>().is_ok())
        .map(|s| s.parse().expect("parse error"))
        .collect()
}

fn solve(input: &Vec<usize>, max_day: u64) -> u64 {
    let mut fishy = vec![0u64; 10];
    for i in input.iter() {
        fishy[*i] += 1;
    }

    for _ in 0..max_day {
        let mut new_fishy = vec![0u64; 10];
        for i in 0..fishy.len() {
            if i == 0 {
                new_fishy[6] += fishy[0];
                new_fishy[8] += fishy[0];
            } else {
                new_fishy[i - 1] += fishy[i];
            }
        }
        fishy = new_fishy.to_vec();
    }

    fishy.iter().sum()
}

pub fn run() {
    println!("Day 06:");
    let input = parse_input("./src/day06/input");

    println!("{}", solve(&input, 80));
    println!("{}", solve(&input, 256));
}
