use core::cmp::max;
use std::cmp::Ordering;
use std::fs::File;
use std::io::{prelude::*, BufReader};
use std::ops::Mul;

fn main() {
    part1();
    part2();
}

struct List {
    value: Option<i32>,
    sub_list: Vec<List>,
}

fn lists_compare(left: &List, right: &List) -> Ordering {
    if left.value.is_some() && right.value.is_some() {
        if left.value.unwrap() < right.value.unwrap() {
            return Ordering::Less;
        } else if left.value.unwrap() > right.value.unwrap() {
            return Ordering::Greater;
        }
        return Ordering::Equal;
    }

    let sub_list_left;
    let sub_list_right;
    let expanded; //to keep it in scope

    if left.value.is_some() {
        expanded = vec![List {
            value: left.value,
            sub_list: Vec::new(),
        }];
        sub_list_left = &expanded;
        sub_list_right = &right.sub_list;
    } else if right.value.is_some() {
        expanded = vec![List {
            value: right.value,
            sub_list: Vec::new(),
        }];
        sub_list_left = &left.sub_list;
        sub_list_right = &expanded;
    } else {
        sub_list_left = &left.sub_list;
        sub_list_right = &right.sub_list;
    }

    for i in 0..max(sub_list_left.len(), sub_list_right.len()) {
        if i >= sub_list_left.len() {
            return Ordering::Less;
        } else if i >= sub_list_right.len() {
            return Ordering::Greater;
        }

        let order = lists_compare(&sub_list_left[i], &sub_list_right[i]);
        if order != Ordering::Equal {
            return order;
        }
    }
    Ordering::Equal
}

fn get_char(line: &Vec<u8>, offset: &mut usize) -> u8 {
    let result = line[*offset];
    *offset += 1;
    result
}

fn start_parse_list(line: &Vec<u8>) -> List {
    let mut offset = 0;
    parse_list(line, &mut offset)
}

fn parse_list(line: &Vec<u8>, offset: &mut usize) -> List {
    let mut sub_list: Vec<List> = Vec::new();

    let mut cur_val: Option<i32> = None;

    while *offset < line.len() {
        let char = get_char(line, offset);
        if char == ']' as u8 || char == ',' as u8 {
            if cur_val.is_some() {
                sub_list.push(List {
                    value: Some(cur_val.unwrap()),
                    sub_list: Vec::new(),
                });
            }
            if char == ']' as u8 {
                break;
            }
            cur_val = None;
        } else if char == '[' as u8 {
            sub_list.push(parse_list(line, offset));
        } else {
            let val = (char - '0' as u8) as i32;
            if cur_val.is_none() {
                cur_val = Some(val);
            } else {
                cur_val = Some(cur_val.unwrap().mul(10) + val);
            }
        }
    }
    List {
        value: None,
        sub_list: sub_list,
    }
}

fn part1() {
    let file = File::open("input.txt").unwrap();
    let reader = BufReader::new(file);

    let mut result = 0;
    let lines: Vec<Vec<u8>> = reader.lines().map(|l| l.unwrap().into_bytes()).collect();
    for i in (0..lines.len()).step_by(3) {
        let left = start_parse_list(&lines[i]);
        let right = start_parse_list(&lines[i + 1]);

        let are_in_order = lists_compare(&left, &right);
        if are_in_order == Ordering::Less {
            result += 1 + (i / 3);
        }
    }
    println!("{result}");
}

fn part2() {
    let file = File::open("input.txt").unwrap();
    let reader = BufReader::new(file);

    let lines: Vec<Vec<u8>> = reader.lines().map(|l| l.unwrap().into_bytes()).collect();
    let mut lists: Vec<List> = Vec::new();
    for i in (0..lines.len()).step_by(3) {
        lists.push(start_parse_list(&lines[i]));
        lists.push(start_parse_list(&lines[i + 1]));
    }
    let temp = "[[2]]".as_bytes().to_vec();
    let divider1 = start_parse_list(&temp);
    let mut divider1_pos = 1;

    let divider2 = start_parse_list(&"[[6]]".as_bytes().to_vec());
    let mut divider2_pos = 1;

    for i in 0..lists.len() {
        if lists_compare(&divider1, &lists[i]) == Ordering::Greater {
            divider1_pos += 1;
        }
        if lists_compare(&divider2, &lists[i]) == Ordering::Greater {
            divider2_pos += 1;
        }
    }
    if lists_compare(&divider1, &divider2) == Ordering::Greater {
        divider1_pos += 1;
    } else {
        divider2_pos += 1;
    }

    let result = divider1_pos * divider2_pos;

    println!("{result}");
}
