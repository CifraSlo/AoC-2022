use std::fs::File;
use std::io::{prelude::*, BufReader};

fn main() {
    part1();
    part2();
}

fn find_start(msg_size: usize) -> usize {
    let file = File::open("input.txt").unwrap();
    let reader = BufReader::new(file);

    let line_str = reader.lines().next().unwrap().unwrap();
    let line = line_str.as_bytes();

    for i in (msg_size - 1)..line.len() {
        let mut score: u32 = 0;
        for j in 0..msg_size {
            score |= 1 << (line[i - j] - 'a' as u8);
        }
        if score.count_ones() == msg_size as u32 {
            return i + 1;
        }
    }
    0
}

fn part1() {
    println!("{}", find_start(4));
}

fn part2() {
    println!("{}", find_start(14));
}
