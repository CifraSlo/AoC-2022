use core::panic;
use std::fs::File;
use std::io::{prelude::*, BufReader};

fn main() {
    part1();
    part2();
}

fn get_empty_line_index(lines: &Vec<String>) -> usize {
    for i in 0..lines.len() {
        if lines[i].is_empty() {
            return i;
        }
    }
    panic!("invalid input");
}

fn part1() {
    let file = File::open("input.txt").unwrap();
    let reader = BufReader::new(file);

    let lines: Vec<String> = reader.lines().map(|l| l.unwrap()).collect();

    let empty_line_index = get_empty_line_index(&lines);
    let setup = &lines[..empty_line_index];

    let columns = *setup
        .last()
        .unwrap()
        .as_bytes()
        .get(setup.last().unwrap().len() - 2)
        .unwrap()
        - 48;
    let mut state: Vec<Vec<u8>> = vec![Vec::new(); columns as usize];

    for i in (0..(setup.len() - 1)).rev() {
        let line = setup.get(i).unwrap().as_bytes();
        for column_index in 0..columns {
            let char = *line.get(1 + (column_index * 4) as usize).unwrap();
            if (char != ' ' as u8) {
                state.get_mut(column_index as usize).unwrap().push(char);
            }
        }
    }

    let commands = &lines[(empty_line_index + 1)..];
    for command in commands {
        let values: Vec<&str> = command.split_whitespace().collect();
        let repeats = values.get(1).unwrap().parse::<i32>().unwrap();
        let src = values.get(3).unwrap().parse::<usize>().unwrap() - 1;
        let dst = values.get(5).unwrap().parse::<usize>().unwrap() - 1;

        for i in 00..repeats {
            let val = state.get_mut(src).unwrap().pop().unwrap();
            state.get_mut(dst).unwrap().push(val);
        }
    }

    for i in 0..columns {
        let top = *state.get(i as usize).unwrap().last().unwrap() as char;
        print!("{top}");
    }
    println!();
}

fn part2() {
    let file = File::open("input.txt").unwrap();
    let reader = BufReader::new(file);

    let lines: Vec<String> = reader.lines().map(|l| l.unwrap()).collect();

    let empty_line_index = get_empty_line_index(&lines);
    let setup = &lines[..empty_line_index];

    let columns = *setup
        .last()
        .unwrap()
        .as_bytes()
        .get(setup.last().unwrap().len() - 2)
        .unwrap()
        - 48;
    let mut state: Vec<Vec<u8>> = vec![Vec::new(); columns as usize];

    for i in (0..(setup.len() - 1)).rev() {
        let line = setup.get(i).unwrap().as_bytes();
        for column_index in 0..columns {
            let char = *line.get(1 + (column_index * 4) as usize).unwrap();
            if (char != ' ' as u8) {
                state.get_mut(column_index as usize).unwrap().push(char);
            }
        }
    }

    let commands = &lines[(empty_line_index + 1)..];
    for command in commands {
        let values: Vec<&str> = command.split_whitespace().collect();
        let repeats = values.get(1).unwrap().parse::<i32>().unwrap();
        let src = values.get(3).unwrap().parse::<usize>().unwrap() - 1;
        let dst = values.get(5).unwrap().parse::<usize>().unwrap() - 1;

        let mut temp_stack: Vec<u8> = Vec::new();

        for i in 00..repeats {
            temp_stack.push(state.get_mut(src).unwrap().pop().unwrap());
        }
        for i in 00..repeats {
            state.get_mut(dst).unwrap().push(temp_stack.pop().unwrap());
        }
    }

    for i in 0..columns {
        let top = *state.get(i as usize).unwrap().last().unwrap() as char;
        print!("{top}");
    }
    println!();
}
