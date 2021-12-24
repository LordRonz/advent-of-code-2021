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

fn part_b(input: &Vec<(String, String)>) -> usize {
    let mut sum_output = 0;
    for i in input.into_iter() {
        let (signal, output) = i;
        let outputs: Vec<String> = output
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect_vec();
        let mut signals: Vec<String> = signal
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect_vec();
        signals.sort_by_key(|s| s.len());

        let mut mappings = vec!["".to_string(); 10];

        for sig in signals.into_iter() {
            let tra = sig.len();
            if tra == 2 {
                mappings[1] = sig;
            } else if tra == 3 {
                mappings[7] = sig;
            } else if tra == 4 {
                mappings[4] = sig;
            } else if tra == 5 {
                if mappings[1].chars().map(|c| sig.contains(c)).all(|f| f) {
                    mappings[3] = sig;
                } else if mappings[4].chars().map(|c| sig.contains(c)).filter(|p| *p).count() == 3 {
                    mappings[5] = sig;
                } else {
                    mappings[2] = sig;
                }
            } else if tra == 6 {
                if mappings[4].chars().map(|c| sig.contains(c)).all(|f| f) {
                    mappings[9] = sig;
                } else if mappings[7].chars().map(|c| sig.contains(c)).filter(|p| *p).count() == 3 {
                    mappings[0] = sig;
                } else {
                    mappings[6] = sig;
                }
            } else {
                mappings[8] = sig;
            }
        }

        let mut parsed = 0;

        for (j, n) in outputs.iter().rev().enumerate() {
            for i in 0..10 {
                if mappings[i].chars().map(|c| n.contains(c)).all(|f| f)
                    && mappings[i].len() == n.len() {
                    parsed += i * 10_usize.pow(j.try_into().unwrap());
                }
            }
        }

        sum_output += parsed
    }

    sum_output
}

pub fn run() {
    println!("Day 08:");
    let input = parse_input("./src/day08/input");

    println!("{}", part_a(&input));
    println!("{}", part_b(&input));
}
