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
        let elf_pick = line[0] as i8 - 'A' as i8;
        let our_pick = line[2] as i8 - 'X' as i8;
        let result = ((our_pick - elf_pick) + 1).rem_euclid(3); // [1, 2, 3]

        score += i32::from((our_pick + 1) + (result * 3));
    }
    println!("{score}");
}

fn part2() {
    let file = File::open("input.txt").unwrap();
    let reader = BufReader::new(file);

    let mut score: i32 = 0;
    for line_wrapped in reader.lines() {
        let line = line_wrapped.unwrap().into_bytes();
        let elf_pick = line[0] as i8 - 'A' as i8;
        let result = line[2] as i8 - 'Y' as i8; // [-1, 0, 1]
        let our_pick = (elf_pick + result).rem_euclid(3);

        score += i32::from((our_pick + 1) + ((result + 1) * 3));
    }
    println!("{score}");
}
