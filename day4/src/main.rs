use std::fs::File;
use std::io::{prelude::*, BufReader};

fn main() {
    part1();
    part2();
}

fn part1() {
    let file = File::open("input.txt").unwrap();
    let reader = BufReader::new(file);

    let mut contained = 0;
    for line_wrapped in reader.lines() {
        let nums: Vec<i32> = line_wrapped
            .unwrap()
            .split([',', '-'])
            .map(|s| s.parse().unwrap())
            .collect();

        if (nums.get(0).unwrap() >= nums.get(2).unwrap() && nums.get(1).unwrap() <= nums.get(3).unwrap())
            || (nums.get(0).unwrap() <= nums.get(2).unwrap() && nums.get(1).unwrap() >= nums.get(3).unwrap())
        {
            contained += 1;
        }
    }
    println!("{contained}");
}

fn part2() {
    let file = File::open("input.txt").unwrap();
    let reader = BufReader::new(file);

    let mut overlapped = 0;
    for line_wrapped in reader.lines() {
        let nums: Vec<i32> = line_wrapped
            .unwrap()
            .split([',', '-'])
            .map(|s| s.parse().unwrap())
            .collect();

        if (nums.get(0).unwrap() >= nums.get(2).unwrap() && nums.get(0).unwrap() <= nums.get(3).unwrap())
            || (nums.get(2).unwrap() >= nums.get(0).unwrap() && nums.get(2).unwrap() <= nums.get(1).unwrap())
        {
            overlapped += 1;
        }
    }
    println!("{overlapped}");
}
