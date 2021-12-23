use std::{
    fs::File,
    io::{prelude::*, BufReader},
    path::Path,
    vec,
};

use itertools::Itertools;

fn parse_input(filename: impl AsRef<Path>) -> Vec<(String, String)> {
    let file = File::open(filename).expect("no such file");
    let buf = BufReader::new(file);

    buf.lines()
        .map(|s| {
            s.expect("parse error")
                .split(" | ")
                .map(|s| s.parse::<String>().unwrap())
                .collect_tuple::<(String, String)>()
                .unwrap()
        })
        .collect_vec()
}

fn part_a(input: &Vec<(String, String)>) -> usize {
    let only_includes = vec![2, 4, 3, 7];
    let mut occurences = 0;
    for i in input.into_iter() {
        let (_, output) = i;
        let outputs: Vec<String> = output
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect_vec();

        occurences += outputs
            .into_iter()
            .filter(|l| only_includes.contains(&l.len()))
            .count()
    }

    occurences
}

pub fn run() {
    println!("Day 08:");
    let input = parse_input("./src/day08/input");

    println!("{}", part_a(&input));
}
