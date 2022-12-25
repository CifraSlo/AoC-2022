use std::fs::File;
use std::io::{prelude::*, BufReader};

const SIZE: usize = 800;
const OFFSET: i32 = 300;

fn move_head(head: &mut [i32; 2], dir: char) {
    match dir {
        'U' => head[0] -= 1,
        'D' => head[0] += 1,
        'L' => head[1] -= 1,
        'R' => head[1] += 1,
        _ => panic!(),
    };
}

fn tail_catch(tail: &mut [i32; 2], head: &[i32; 2], field: &mut [[bool; SIZE]; SIZE], visited_count: &mut usize, check_visited: bool) {
    let diff_y = head[0] - tail[0];
    let diff_x = head[1] - tail[1];

    if diff_y.abs() >= 2 {
        tail[0] = head[0] - diff_y.signum();
        tail[1] += diff_x.signum();
    } else if diff_x.abs() >= 2 {
        tail[0] += diff_y.signum();
        tail[1] = head[1] - diff_x.signum();
    }

    if !field[tail[0] as usize][tail[1] as usize] && check_visited {
        field[tail[0] as usize][tail[1] as usize] = true;
        *visited_count += 1;
    }
}

fn solve(lines: &Vec<String>, snake_len: usize) {
    let mut snake = vec![[OFFSET, OFFSET]; snake_len];
    let mut field = [[false; SIZE]; SIZE];
    let mut visited_count = 0;

    for line in lines {
        let dir = line.as_bytes()[0] as char;
        let count = line[2..].parse().unwrap();
        for _ in 0..count {
            move_head(&mut snake[0], dir);
            let mut prev = snake[0];
            for i in 1..(snake.len() - 1) {
                tail_catch(&mut snake[i], &prev, &mut field, &mut visited_count, false);
                prev = snake[i];
            }
            tail_catch(&mut snake[snake_len - 1], &prev, &mut field, &mut visited_count, true);
        }
    }
    println!("{visited_count}");
}

fn main() {
    let file = File::open("input.txt").unwrap();
    let reader = BufReader::new(file);

    let lines: Vec<String> = reader.lines().map(|l| l.unwrap()).collect();
    part1(&lines);
    part2(&lines);
}

fn part1(lines: &Vec<String>) {
    solve(lines, 2);
}

fn part2(lines: &Vec<String>) {
    solve(lines, 10);
}
