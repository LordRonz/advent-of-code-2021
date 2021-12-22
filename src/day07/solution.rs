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

fn part_a(crabs: &Vec<usize>) -> u64 {
    let max_pos = *crabs.iter().max().unwrap();
    let mut possibilities = vec![0u64; max_pos + 1];
    possibilities[0] = u64::MAX;
    for i in 1..possibilities.len() {
        for j in crabs.iter() {
            let mv = (*j as i32 - i as i32).abs() as u64;
            possibilities[i] += mv as u64;
        }
    }

    *possibilities.iter().min().unwrap()
}

fn part_b(crabs: &Vec<usize>) -> u64 {
    let max_pos = *crabs.iter().max().unwrap();
    let mut possibilities = vec![0u64; max_pos + 1];
    possibilities[0] = u64::MAX;
    for i in 1..possibilities.len() {
        for j in crabs.iter() {
            let mv = (*j as i32 - i as i32).abs() as u64;
            possibilities[i] += mv * (mv + 1) / 2 as u64;
        }
    }

    *possibilities.iter().min().unwrap()
}

pub fn run() {
    println!("Day 07:");
    let input = parse_input("./src/day07/input");

    println!("{}", part_a(&input));
    println!("{}", part_b(&input));
}
