use std::fs::File;
use std::io::{prelude::*, BufReader};

fn main() {
    part1();
    part2();
}

fn part1() {
    let file = File::open("input.txt").unwrap();
    let reader = BufReader::new(file);

    let mut score = 0;
    for line_wrapped in reader.lines() {
        let line = line_wrapped.unwrap().into_bytes();
        let mut found = [false; 128];

        for i in 0..(line.len() / 2) {
            found[line[i] as usize] = true;
        }
        for i in (line.len() / 2)..line.len() {
            let cur = line[i] as usize;
            if found[cur] {
                if cur > 'Z' as usize {
                    score += cur - 96;
                } else {
                    score += cur - 38;
                }
                break;
            }
        }
    }
    println!("{score}");
}

fn part2() {
    let file = File::open("input.txt").unwrap();
    let reader = BufReader::new(file);

    let mut score = 0;
    let mut lines = reader.lines();
    while let (Some(line1), Some(line2), Some(line3)) = (lines.next(), lines.next(), lines.next()) {
        let mut found1 = [false; 128];
        let mut found2 = [false; 128];
        for item in line1.unwrap().into_bytes() {
            found1[item as usize] = true;
        }
        for item in line2.unwrap().into_bytes() {
            found2[item as usize] = true;
        }
        for item in line3.unwrap().into_bytes() {
            let cur = item as usize;
            if found1[cur] && found2[cur]  {
                if cur > 'Z' as usize {
                    score += cur - 96;
                } else {
                    score += cur - 38;
                }
                break;
            }
        }
    }
    println!("{score}");
}