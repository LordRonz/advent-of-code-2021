use std::fs::File;
use std::io::{BufRead, BufReader};

fn parse_input() -> (Vec<i32>, Vec<Vec<[i32; 2]>>) {
    let mut f = BufReader::new(File::open("./src/day04/input").unwrap());

    let mut s = String::new();
    f.read_line(&mut s).unwrap();

    let numbers: Vec<i32> = s
        .split(",")
        .filter(|s| s.parse::<i32>().is_ok())
        .map(|s| s.parse().expect("parse error"))
        .collect();

    // let arr: Vec<Vec<[i32; 2]>> = f.lines()
    //     .map(|l| l.unwrap().split(char::is_whitespace)
    //         .map(|number| [number.parse().unwrap(), 0])
    //         .collect())
    //     .collect();

    let mut t = String::new();
    let _ = f.read_line(&mut t);

    let mut boards: Vec<Vec<[i32; 2]>> = Vec::new();
    let mut board: Vec<[i32; 2]> = Vec::new();
    for line in f.lines() {
        let line = line.unwrap();
        let splitted = line
            .split(char::is_whitespace);
        if splitted.clone().count() < 5 {
            boards.push(board);
            board = Vec::new();
            continue;
        }
        board
        .append(& mut splitted
            .filter(|num| num.parse::<i32>().is_ok())
            .map(|num| [num.parse().unwrap(), 0])
            .collect());
    }

    (numbers, boards)
}

pub fn run() {
    let (numbers, mut boards) = parse_input();

    // Part A

    let mut chosen_one: i32 = -1;
    let mut last_num: i32 = -1;
    let row_len = (boards[0].len() as f64).sqrt() as usize;

    for number in numbers.iter() {
        for i in 0..boards.len() {
            for j in 0..boards[i].len() {
                if boards[i][j][0] == *number {
                    boards[i][j][1] = 1;
                }
            }
        }
        for i in 0..boards.len() {
            for j in 0..row_len {
                let mut flag = true;
                let mut flag2 = true;
                for k in 0..row_len {
                    if boards[i][j * row_len + k][1] == 0 {
                        flag = false;
                    }
                    if boards[i][k * row_len + j][1] == 0 {
                        flag2 = false;
                    }
                }
                if flag || flag2 {
                    chosen_one = i as i32;
                    break;
                }
            }
            if chosen_one != -1 {
                break;
            }
        }
        if chosen_one != -1 {
            last_num = *number;
            break;
        }
    }
    let mut sum = 0;
    for num in boards[chosen_one as usize].iter() {
        if num[1] == 0 {
            sum += num[0];
        }
    }

    println!("{}", sum * last_num);
}
