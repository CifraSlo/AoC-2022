use std::fs::File;
use std::io::{prelude::*, BufReader};

fn is_visible(y: usize, x: usize, grid: &Vec<Vec<u8>>) -> bool {
    for neg_y in 1.. {
        if grid[y - neg_y][x] >= grid[y][x] {
            break;
        }
        if y - neg_y == 0 {
            return true;
        }
    }
    for pos_y in 1.. {
        if grid[y + pos_y][x] >= grid[y][x] {
            break;
        }
        if y + pos_y == grid.len() - 1 {
            return true;
        }
    }
    for neg_x in 1.. {
        if grid[y][x - neg_x] >= grid[y][x] {
            break;
        }
        if x - neg_x == 0 {
            return true;
        }
    }
    for pos_x in 1.. {
        if grid[y][x + pos_x] >= grid[y][x] {
            break;
        }
        if x + pos_x == grid[y].len() - 1 {
            return true;
        }
    }

    false
}

fn scenic_score(y: usize, x: usize, grid: &Vec<Vec<u8>>) -> usize {
    let mut score = 1;
    for neg_y in 0.. {
        if y - neg_y == 0 || (grid[y - neg_y][x] >= grid[y][x] && neg_y != 0) {
            score *= neg_y;
            break;
        }
    }
    for pos_y in 0.. {
        if y + pos_y == grid.len() - 1 || (grid[y + pos_y][x] >= grid[y][x] && pos_y != 0) {
            score *= pos_y;
            break;
        }
    }
    for neg_x in 0.. {
        if x - neg_x == 0 || (grid[y][x - neg_x] >= grid[y][x] && neg_x != 0) {
            score *= neg_x;
            break;
        }
    }
    for pos_x in 0.. {
        if x + pos_x == grid[y].len() - 1 || (grid[y][x + pos_x] >= grid[y][x] && pos_x != 0) {
            score *= pos_x;
            break;
        }
    }

    score
}

fn main() {
    let file = File::open("input.txt").unwrap();
    let reader = BufReader::new(file);

    let grid: Vec<Vec<u8>> = reader.lines().map(|l| l.unwrap().into_bytes()).collect();

    part1(&grid);
    part2(&grid);
}

fn part1(grid: &Vec<Vec<u8>>) {
    let mut visible_count = (grid.len() * 2) + ((grid[0].len() - 2) * 2);
    for y in 1..(grid.len() - 1) {
        for x in 1..(grid[y].len() - 1) {
            if is_visible(y, x, grid) {
                visible_count += 1;
            }
        }
    }
    println!("{visible_count}");
}

fn part2(grid: &Vec<Vec<u8>>) {
    let mut max_score = 0;
    for y in 1..(grid.len() - 1) {
        for x in 1..(grid[y].len() - 1) {
            let score = scenic_score(y, x, grid);
            if score > max_score {
                max_score = score;
            }
        }
    }
    println!("{max_score}");
}
