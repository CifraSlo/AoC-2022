use std::fs::File;
use std::io::{prelude::*, BufReader};

fn main() {
    part1();
    part2();
}

fn part1() {
    let file = File::open("input.txt").unwrap();
    let reader = BufReader::new(file);

    let mut current = 0;
    let mut max = 0;
    for line in reader.lines() {
        let l = line.unwrap();
        if l.is_empty() {
            if current > max {
                max = current;
            }
            current = 0;
        } else {
            current += l.parse::<i32>().unwrap();
        }
    }
    println!("{max}");
}

fn part2() {
    let file = File::open("input.txt").unwrap();
    let reader = BufReader::new(file);

    let mut current = 0;
    let mut top3 = [0, 0, 0];
    for line in reader.lines() {
        let l = line.unwrap();
        if l.is_empty() {
            if current > top3[0] {
                top3[0] = current;
                top3.sort();
            }
            current = 0;
        } else {
            current += l.parse::<i32>().unwrap();
        }
    }
    println!("{}", top3.iter().sum::<i32>());
}