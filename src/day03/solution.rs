use std::{
    fs::File,
    io::{prelude::*, BufReader},
    path::Path,
};

fn lines_from_file(filename: impl AsRef<Path>) -> Vec<String> {
    let file = File::open(filename).expect("no such file");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|line| line.unwrap())
        .collect()
}

fn get_filtered_item(items: &Vec<String>, prefer_common: bool) -> i32 {
    let mut current = items.to_vec();
    let mut i = 0;
    while current.len() > 1 {
        let mut count = 0;
        for s in current.iter() {
            if s.chars().nth(i).unwrap() == '1' {
                count += 1;
            }
        }
        let half = current.len() as f32 / 2.0;
        let char_to_remove: char;
        if prefer_common {
            if count as f32 >= half {
                char_to_remove = '1';
            } else {
                char_to_remove = '0';
            }
        } else {
            if count as f32 >= half {
                char_to_remove = '0';
            } else {
                char_to_remove = '1';
            }
        }
        current.retain(|bit| bit.chars().nth(i).unwrap() == char_to_remove);
        i += 1;
    }
    isize::from_str_radix(current[0].as_str(), 2).unwrap() as i32
}

pub fn run() {
    println!("Day 03:");
    let lines = lines_from_file("./src/day03/input");
    // Part A
    let mut counts = vec![0; lines[0].len()];
    for line in lines.iter() {
        for (i, bit) in line.chars().enumerate() {
            if bit == '1' {
                counts[i] += 1;
            }
        }
    }
    let mut gamma = 0;
    let mut epsilon = 0;
    for count in counts.iter() {
        gamma <<= 1;
        epsilon <<= 1;
        let half = lines.len() as f32 / 2.0;
        if *count as f32 >= half {
            gamma += 1;
        } else {
            epsilon += 1;
        }
    }

    println!("{}", gamma * epsilon);

    // Part B

    let oxygen_generator_rating = get_filtered_item(&lines, true);
    let co2_scrubber_rating = get_filtered_item(&lines, false);
    println!("{}", oxygen_generator_rating * co2_scrubber_rating);
}
