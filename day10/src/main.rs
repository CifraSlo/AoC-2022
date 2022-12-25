use std::fs::File;
use std::io::{prelude::*, BufReader};

fn main() {
    part1();
    part2();
}

fn part1() {
    let file = File::open("input.txt").unwrap();
    let reader = BufReader::new(file);

    let lines = reader.lines().map(|l| l.unwrap());
    let mut x = 1;
    let stops = [20, 60, 100, 140, 180, 220, 99999999];
    let mut next_stop = 0;
    let mut cycles = 0;
    let mut result = 0;
    for line in lines {
        let is_add = line.as_bytes()[0] == 'a' as u8;
        cycles += 1;
        if is_add {
            cycles += 1;
        }

        if cycles >= stops[next_stop] {
            result += stops[next_stop] * x;
            next_stop += 1;
        }

        if is_add {
            x += line[5..].parse::<i32>().unwrap();
        }
    }
    println!("{}", result);
}

fn print(x: i32, cycle: &mut i32) {
    if *cycle >= 40 {
        *cycle = 0;
        print!("\n");
    }
    if *cycle >= x - 1 && *cycle <= x + 1 {
        print!("#");
    } else {
        print!(".");
    }
}

fn part2() {
    let file = File::open("input.txt").unwrap();
    let reader = BufReader::new(file);

    let lines = reader.lines().map(|l| l.unwrap());
    let mut x = 1;
    let mut cycles = 0;
    for line in lines {
        let is_add = line.as_bytes()[0] == 'a' as u8;
        print(x, &mut cycles);
        cycles += 1;
        if is_add {
            print(x, &mut cycles);
            cycles += 1;
        }

        if is_add {
            x += line[5..].parse::<i32>().unwrap();
        }
    }
}
