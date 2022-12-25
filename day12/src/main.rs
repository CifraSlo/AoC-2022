use std::collections::VecDeque;
use std::fs::File;
use std::io::{prelude::*, BufReader};

fn main() {
    part1();
    part2();
}

struct Point {
    y: usize,
    x: usize,
}

fn part1() {
    let file = File::open("input.txt").unwrap();
    let reader = BufReader::new(file);

    let mut lines: Vec<Vec<u8>> = reader.lines().map(|l| l.unwrap().into_bytes()).collect();
    let height = lines.len();
    let width = lines.first().unwrap().len();

    let mut pending_processing = vec![vec![false; width]; height];
    let mut energy_cost = vec![vec![i32::MAX; width]; height];
    let mut process_queue: VecDeque<Point> = VecDeque::new();
    let mut cost_limit = i32::MAX;

    let mut end: Point = Point { y: 0, x: 0 };
    for y in 0..height {
        for x in 0..width {
            if lines[y][x] == 'S' as u8 {
                process_queue.push_back(Point { y: y, x: x });
                pending_processing[y][x] = true;
                lines[y][x] = 'a' as u8;
                energy_cost[y][x] = 0;
            } else if lines[y][x] == 'E' as u8 {
                end = Point { y: y, x: x };
                lines[y][x] = 'z' as u8;
            }
        }
    }

    while !process_queue.is_empty() {
        let cur = process_queue.pop_front().unwrap();
        pending_processing[cur.y][cur.x] = false;
        let cost = energy_cost[cur.y][cur.x] + 1;

        if cost >= cost_limit {
            continue;
        }
        if cur.y == end.y && cur.x == end.x {
            cost_limit = cost - 1;
            continue;
        }

        let mut next_points: Vec<Point> = Vec::new();
        if cur.x > 0 {
            next_points.push(Point { y: cur.y, x: cur.x - 1 });
        }
        if cur.x < width - 1 {
            next_points.push(Point { y: cur.y, x: cur.x + 1 });
        }
        if cur.y > 0 {
            next_points.push(Point { y: cur.y - 1, x: cur.x });
        }
        if cur.y < height - 1 {
            next_points.push(Point { y: cur.y + 1, x: cur.x });
        }

        for point in next_points {
            if lines[cur.y][cur.x] + 1 < lines[point.y][point.x] {
                continue;
            }
            if energy_cost[point.y][point.x] <= cost {
                continue;
            }
            if pending_processing[point.y][point.x] {
                continue;
            }

            energy_cost[point.y][point.x] = cost;
            pending_processing[point.y][point.x] = true;
            process_queue.push_back(point);
        }
    }

    println!("{}", cost_limit);
}

fn part2() {
    let file = File::open("input.txt").unwrap();
    let reader = BufReader::new(file);

    let mut lines: Vec<Vec<u8>> = reader.lines().map(|l| l.unwrap().into_bytes()).collect();
    let height = lines.len();
    let width = lines.first().unwrap().len();

    let mut pending_processing = vec![vec![false; width]; height];
    let mut energy_cost = vec![vec![i32::MAX; width]; height];
    let mut process_queue: VecDeque<Point> = VecDeque::new();
    let mut cost_limit = i32::MAX;

    for y in 0..height {
        for x in 0..width {
            if lines[y][x] == 'E' as u8 {
                process_queue.push_back(Point { y: y, x: x });
                pending_processing[y][x] = true;
                lines[y][x] = 'z' as u8;
                energy_cost[y][x] = 0;
            } else if lines[y][x] == 'S' as u8 {
                lines[y][x] = 'a' as u8;
            }
        }
    }

    while !process_queue.is_empty() {
        let cur = process_queue.pop_front().unwrap();
        pending_processing[cur.y][cur.x] = false;
        let cost = energy_cost[cur.y][cur.x] + 1;

        if cost >= cost_limit {
            continue;
        }
        if lines[cur.y][cur.x] == 'a' as u8 {
            cost_limit = cost - 1;
            continue;
        }

        let mut next_points: Vec<Point> = Vec::new();
        if cur.x > 0 {
            next_points.push(Point { y: cur.y, x: cur.x - 1 });
        }
        if cur.x < width - 1 {
            next_points.push(Point { y: cur.y, x: cur.x + 1 });
        }
        if cur.y > 0 {
            next_points.push(Point { y: cur.y - 1, x: cur.x });
        }
        if cur.y < height - 1 {
            next_points.push(Point { y: cur.y + 1, x: cur.x });
        }

        for point in next_points {
            if lines[cur.y][cur.x] - 1 > lines[point.y][point.x] {
                continue;
            }
            if energy_cost[point.y][point.x] <= cost {
                continue;
            }
            if pending_processing[point.y][point.x] {
                continue;
            }

            energy_cost[point.y][point.x] = cost;
            pending_processing[point.y][point.x] = true;
            process_queue.push_back(point);
        }
    }

    println!("{cost_limit}");
}
