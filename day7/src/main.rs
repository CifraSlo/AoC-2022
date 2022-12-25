use core::panic;
use std::collections::{HashMap, VecDeque};
use std::fs::File;
use std::io::{prelude::*, BufReader};

struct Folder {
    sub_folders: HashMap<String, usize>,
    sub_files: HashMap<String, i64>,
    size: Option<i64>,
}

fn parse_directory(lines: &Vec<String>) -> Vec<Folder> {
    let mut folders = Vec::new();
    folders.push(Folder {
        sub_folders: HashMap::new(),
        sub_files: HashMap::new(),
        size: None,
    });
    let mut current = 0;
    let mut stack = VecDeque::new();
    let mut line_idx = 0;
    while line_idx < lines.len() {
        let command = &lines[line_idx];
        line_idx += 1;
        if command[0..4] == *"$ cd" {
            if command.len() == 7 && command[5..7] == *".." {
                current = stack.pop_back().unwrap();
            } else if command[5..6] == *"/" {
                stack.clear();
                current = 0;
            } else {
                stack.push_back(current);
                current = folders[current].sub_folders[&command[5..]];
            }
        } else if command[0..4] == *"$ ls" {
            while line_idx < lines.len() && lines[line_idx].as_bytes()[0] != '$' as u8 {
                let sub_line = &lines[line_idx];
                line_idx += 1;
                if sub_line[0..3] == *"dir" {
                    let idx = folders.len();
                    folders[current].sub_folders.insert(sub_line[4..].to_string(), idx);
                    folders.push(Folder {
                        sub_folders: HashMap::new(),
                        sub_files: HashMap::new(),
                        size: None,
                    });
                } else {
                    let split: Vec<&str> = sub_line.split(' ').collect();
                    folders[current].sub_files.insert(split[1].to_string(), split[0].parse().unwrap());
                }
            }
        } else {
            panic!("how?");
        }
    }

    folders
}

fn calculate_sizes(folders: &mut Vec<Folder>, index: usize) -> i64 {
    if folders[index].size.is_some() {
        return folders[index].size.unwrap();
    }

    let mut size = 0;
    for sub_file in folders[index].sub_files.iter() {
        size += sub_file.1;
    }
    let sub_folder_indices: Vec<usize> = folders[index].sub_folders.values().map(|i| *i).collect();
    for sub_folder_index in sub_folder_indices {
        size += calculate_sizes(folders, sub_folder_index);
    }

    folders[index].size = Some(size);

    size
}

fn main() {
    let file = File::open("input.txt").unwrap();
    let reader = BufReader::new(file);

    let lines: Vec<String> = reader.lines().map(|l| l.unwrap()).collect();

    part1(&lines);
}

fn part1(lines: &Vec<String>) {
    let mut folders = parse_directory(lines);
    _ = calculate_sizes(&mut folders, 0);

    let mut result = 0;
    for folder in &folders {
        if folder.size.unwrap() <= 100000 {
            result += folder.size.unwrap();
        }
    }

    println!("{}", result);
    part2(folders);
}

fn part2(folders: Vec<Folder>) {
    let total_occupied = folders[0].size.unwrap();
    let unused = 70000000 - total_occupied;
    let required = 30000000 - unused;

    let mut min = i64::MAX;
    for folder in &folders {
        if folder.size.unwrap() > required && folder.size.unwrap() < min {
            min = folder.size.unwrap();
        }
    }
    println!("{}", min);
}
