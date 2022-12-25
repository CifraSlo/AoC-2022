use std::fs::File;
use std::io::{prelude::*, BufReader};

fn main() {
    let file = File::open("input.txt").unwrap();
    let reader = BufReader::new(file);

    let lines: Vec<Vec<u8>> = reader.lines().map(|l| l.unwrap().into_bytes()).collect();

    let mut total = 0;

    for line in &lines {
        let mut current = 0;
        for digit in line {
            current *= 5;
            current += match *digit as char {
                '=' => -2,
                '-' => -1,
                _ => (digit - '0' as u8) as i64,
            };
        }
        total += current;
    }

    let mut result = Vec::new();

    while total != 0 {
        let current = total % 5;
        total /= 5;
        if current == 4 {
            result.push('-');
            total += 1;
        } else if current == 3 {
            result.push('=');
            total += 1;
        } else {
            result.push(((current as u8) + ('0' as u8)) as char);
        }
    }

    for digit in (0..result.len()).rev() {
        print!("{}", result[digit]);
    }
}
