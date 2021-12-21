use itertools::Itertools;
use std::{
    cmp,
    fs::File,
    io::{prelude::*, BufReader},
};

struct Line {
    x0: u16,
    x1: u16,
    y0: u16,
    y1: u16,
}

fn parse_line(s: &String) -> Line {
    let ends: ((u16, u16), (u16, u16)) = s
        .split(" -> ")
        .map(|s| {
            s.split(',')
                .map(|n| n.parse().unwrap())
                .collect_tuple()
                .unwrap()
        })
        .collect_tuple()
        .unwrap();

    Line {
        x0: ends.0 .0,
        y0: ends.0 .1,
        x1: ends.1 .0,
        y1: ends.1 .1,
    }
}

fn part_a(l: &Vec<Line>) -> usize {
    let mut matrixes = vec![vec![0u16; 1000]; 1000];

    for line in l {
        if line.x0 == line.x1 {
            let min_y = cmp::min(line.y0, line.y1);
            let max_y = cmp::max(line.y0, line.y1);
            for i in min_y..=max_y {
                matrixes[i as usize][line.x0 as usize] += 1;
            }
        } else if line.y0 == line.y1 {
            let min_x = cmp::min(line.x0, line.x1);
            let max_x = cmp::max(line.x0, line.x1);
            for i in min_x..=max_x {
                matrixes[line.y0 as usize][i as usize] += 1;
            }
        }
    }
    matrixes
        .iter()
        .map(|row| row.iter().filter(|n| **n > 1).count())
        .sum::<usize>()
}

fn part_b(l: &Vec<Line>) -> usize {
    let mut matrixes = vec![vec![0u16; 1000]; 1000];

    for line in l {
        if line.x0 == line.x1 {
            let min_y = cmp::min(line.y0, line.y1);
            let max_y = cmp::max(line.y0, line.y1);
            for i in min_y..=max_y {
                matrixes[i as usize][line.x0 as usize] += 1;
            }
        } else if line.y0 == line.y1 {
            let min_x = cmp::min(line.x0, line.x1);
            let max_x = cmp::max(line.x0, line.x1);
            for i in min_x..=max_x {
                matrixes[line.y0 as usize][i as usize] += 1;
            }
        } else if (line.x1 as i32 - line.x0 as i32).abs() == (line.y1 as i32 - line.y0 as i32).abs()
        {
            let x_iter = if line.x0 < line.x1 {
                (line.x0..=line.x1).collect_vec().into_iter()
            } else {
                (line.x1..=line.x0).rev().collect_vec().into_iter()
            };

            let y_iter = if line.y0 < line.y1 {
                (line.y0..=line.y1).collect_vec().into_iter()
            } else {
                (line.y1..=line.y0).rev().collect_vec().into_iter()
            };

            for (x, y) in x_iter.zip(y_iter).rev() {
                matrixes[y as usize][x as usize] += 1;
            }
        }
    }
    matrixes
        .iter()
        .map(|row| row.iter().filter(|n| **n > 1).count())
        .sum::<usize>()
}

pub fn run() {
    println!("Day 05:");

    let file = File::open("./src/day05/input").expect("no such file");
    let buf = BufReader::new(file);
    let s: Vec<String> = buf.lines().map(|s| s.unwrap()).collect();
    let input: Vec<_> = s.iter().map(parse_line).collect();

    println!("{}", part_a(&input));
    println!("{}", part_b(&input));
}
