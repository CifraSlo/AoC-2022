use std::collections::VecDeque;
use std::fs::File;
use std::io::{prelude::*, BufReader};

const SIZE: usize = 25;

fn get_occupied_neighbors(field: &Vec<Vec<Vec<bool>>>, cube: &Vec<usize>) -> i32 {
    let mut occupied_neighbors = 0;
    if field[cube[2]][cube[1]][cube[0] + 1] {
        occupied_neighbors += 1;
    }
    if field[cube[2]][cube[1]][cube[0] - 1] {
        occupied_neighbors += 1;
    }
    if field[cube[2]][cube[1] + 1][cube[0]] {
        occupied_neighbors += 1;
    }
    if field[cube[2]][cube[1] - 1][cube[0]] {
        occupied_neighbors += 1;
    }
    if field[cube[2] + 1][cube[1]][cube[0]] {
        occupied_neighbors += 1;
    }
    if field[cube[2] - 1][cube[1]][cube[0]] {
        occupied_neighbors += 1;
    }
    occupied_neighbors
}

fn measure_exterior(field: &Vec<Vec<Vec<bool>>>, z: i32, y: i32, x: i32) -> i32 {
    let mut queue = VecDeque::new();
    let mut processed = vec![vec![vec![false; SIZE]; SIZE]; SIZE];
    queue.push_back(vec![z, y, x]);
    processed[z as usize][y as usize][x as usize] = true;

    let mut surface = 0;

    while !queue.is_empty() {
        let current = queue.pop_front().unwrap();
        for i in 0..current.len() {
            for offset in ((-1)..2).step_by(2) {
                if current[i] + offset < 0 || current[i] + offset >= SIZE as i32 {
                    continue;
                }
                let mut cur = current.clone();
                cur[i] += offset;
                if field[cur[0] as usize][cur[1] as usize][cur[2] as usize] {
                    surface += 1;
                } else if !processed[cur[0] as usize][cur[1] as usize][cur[2] as usize] {
                    processed[cur[0] as usize][cur[1] as usize][cur[2] as usize] = true;
                    queue.push_back(cur);
                }
            }
        }
    }
    surface
}

fn main() {
    let file = File::open("input.txt").unwrap();
    let reader = BufReader::new(file);

    let coords: Vec<Vec<usize>> = reader
        .lines()
        .map(|l| l.unwrap().split(',').map(|ns| ns.parse::<usize>().unwrap() + 1).collect())
        .collect();

    part1(&coords);
    part2(&coords);
}

fn part1(coords: &Vec<Vec<usize>>) {
    let mut field = vec![vec![vec![false; SIZE]; SIZE]; SIZE];

    let mut surface = 0;

    for cube in coords {
        let occupied_neighbors = get_occupied_neighbors(&field, cube);
        surface += 6 - (occupied_neighbors * 2);
        field[cube[2]][cube[1]][cube[0]] = true;
    }

    println!("{}", surface);
}

fn part2(coords: &Vec<Vec<usize>>) {
    let mut field = vec![vec![vec![false; SIZE]; SIZE]; SIZE];
    for cube in coords {
        field[cube[2]][cube[1]][cube[0]] = true;
    }

    println!("{}", measure_exterior(&field, 0, 0, 0));
}
