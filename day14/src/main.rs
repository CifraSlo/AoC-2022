use core::cmp::max;
use std::fs::File;
use std::io::{prelude::*, BufReader};

fn main() {
    part1();
    part2();
}

const SIZE: usize = 750;
const SAND_X: usize = 500;
const SAND_Y: usize = 0;

struct Coord {
    x: i32,
    y: i32,
}

fn fill_line(field: &mut [[bool; SIZE]; SIZE], start: &Coord, end: &Coord) {
    let offset_x = end.x - start.x;
    let offset_y = end.y - start.y;
    let dist = max(offset_x.abs(), offset_y.abs());
    let step_x = offset_x.signum();
    let step_y = offset_y.signum();
    let mut x = start.x;
    let mut y = start.y;

    for _ in 0..(dist + 1) {
        field[y as usize][x as usize] = true;
        y += step_y;
        x += step_x;
    }
}

fn part1() {
    let file = File::open("input.txt").unwrap();
    let reader = BufReader::new(file);

    let mut field = [[false; SIZE]; SIZE];

    for line in reader.lines().map(Result::unwrap) {
        let coords: Vec<Coord> = line
            .split(" -> ")
            .map(|cs| {
                let vals: Vec<i32> = cs.split(',').map(|n| n.parse().unwrap()).collect();
                Coord { x: vals[0], y: vals[1] }
            })
            .collect();

        for i in 1..coords.len() {
            fill_line(&mut field, &coords[i - 1], &coords[i])
        }
    }

    let mut units = 0;
    loop {
        let mut x = SAND_X;
        let mut y = SAND_Y;

        let mut stopped = false;
        while y < SIZE - 1 {
            if !field[y + 1][x] {
            } else if !field[y + 1][x - 1] {
                x -= 1;
            } else if !field[y + 1][x + 1] {
                x += 1;
            } else {
                stopped = true;
                field[y][x] = true;
                break;
            }
            y += 1;
        }
        if !stopped {
            break;
        }
        units += 1;
    }

    println!("{units}");
}

fn part2() {
    let file = File::open("input.txt").unwrap();
    let reader = BufReader::new(file);

    let mut field = [[false; SIZE]; SIZE];

    let mut floor: usize = 0;

    for line in reader.lines().map(Result::unwrap) {
        let coords: Vec<Coord> = line
            .split(" -> ")
            .map(|cs| {
                let vals: Vec<i32> = cs.split(',').map(|n| n.parse().unwrap()).collect();
                if (vals[1] + 2) as usize > floor {
                    floor = (vals[1] + 2) as usize;
                }
                Coord { x: vals[0], y: vals[1] }
            })
            .collect();

        for i in 1..coords.len() {
            fill_line(&mut field, &coords[i - 1], &coords[i])
        }
    }

    let mut units = 0;
    while !field[SAND_Y][SAND_X] {
        let mut x = SAND_X;
        let mut y = SAND_Y;

        while y < floor - 1 {
            if !field[y + 1][x] {
            } else if !field[y + 1][x - 1] {
                x -= 1;
            } else if !field[y + 1][x + 1] {
                x += 1;
            } else {
                break;
            }
            y += 1;
        }
        field[y][x] = true;
        units += 1;
    }

    println!("{units}");
}
