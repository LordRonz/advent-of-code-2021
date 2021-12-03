use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
};

fn lines_from_file(filename: impl AsRef<Path>) -> Vec<Vec<String>> {
    let f = BufReader::new(File::open(filename).unwrap());

    f.lines()
        .map(|l| l.unwrap().split(char::is_whitespace)
            .map(|number| number.parse().unwrap())
            .collect())
        .collect()
}

pub fn run() {
    println!("Day 02:");
    let lines = lines_from_file("./src/day02/input");
    // Part A
    let mut x: i64 = 0;
    let mut y: i64 = 0;

    for line in lines.iter() {
        let d: i64 = line[1].parse::<i64>().unwrap();
        match line[0].as_str() {
            "forward" => {
                x += d;
            }
            "down" => {
                y += d;
            }
            "up" => {
                y -= d;
            }
            _ => {}
        }
    }
    println!("{}", x * y);

    // Part B
    x = 0;
    y = 0;
    let mut aim: i64 = 0;
    for line in lines.iter() {
        let d: i64 = line[1].parse::<i64>().unwrap();
        match line[0].as_str() {
            "forward" => {
                x += d;
                y += aim * d;
            }
            "down" => {
                aim += d;
            }
            "up" => {
                aim -= d;
            }
            _ => {}
        }
    }
    println!("{}", x * y);
}
