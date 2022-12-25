use std::fs::File;
use std::io::{prelude::*, BufReader};

const SIZE: usize = 200;
const OFFSET: usize = 50;

const DIRECTIONS: [[i32; 2]; 4] = [[-1, 0], [1, 0], [0, -1], [0, 1]];

fn get_proposed_move(field: &[[bool; SIZE]; SIZE], elf: &[i32; 2], index: usize) -> Option<[i32; 2]> {
    let mut paused = true;
    for y in (-1)..2 {
        for x in (-1)..2 {
            if y == 0 && x == 0 {
                continue;
            }
            if field[(elf[0] + y) as usize][(elf[1] + x) as usize] {
                paused = false;
                break;
            }
        }
    }
    if paused {
        return None;
    }

    for i in 0..4 {
        let dir_idx = (i + index) % 4;
        let direction = DIRECTIONS[dir_idx];
        let offset_idx = 1 - (dir_idx / 2);
        let mut available = true;
        for j in (-1)..2 {
            let mut dir = direction.clone();
            dir[offset_idx] += j;
            if field[(elf[0] + dir[0]) as usize][(elf[1] + dir[1]) as usize] {
                available = false;
                break;
            }
        }
        if available {
            return Some([elf[0] + direction[0], elf[1] + direction[1]]);
        }
    }

    None
}

fn print_part1(elves: &Vec<[i32; 2]>) {
    let mut min_y = i32::MAX;
    let mut max_y = i32::MIN;
    let mut min_x = i32::MAX;
    let mut max_x = i32::MIN;
    for elf in elves {
        if elf[0] < min_y {
            min_y = elf[0];
        }
        if elf[0] > max_y {
            max_y = elf[0];
        }
        if elf[1] < min_x {
            min_x = elf[1];
        }
        if elf[1] > max_x {
            max_x = elf[1];
        }
    }
    println!("{}", (((max_y - min_y) + 1) * ((max_x - min_x) + 1)) - elves.len() as i32);
}

fn main() {
    let file = File::open("input.txt").unwrap();
    let reader = BufReader::new(file);

    let lines: Vec<Vec<u8>> = reader.lines().map(|l| l.unwrap().into_bytes()).collect();
    let mut field = [[false; SIZE]; SIZE];
    let mut elves = Vec::new();
    for y in 0..lines.len() {
        for x in 0..lines[y].len() {
            if lines[y][x] == '#' as u8 {
                field[y + OFFSET][x + OFFSET] = true;
                elves.push([(y + OFFSET) as i32, (x + OFFSET) as i32]);
            }
        }
    }

    let mut proposes_field;
    for index in 0.. {
        proposes_field = [[None; SIZE]; SIZE];
        let mut proposes = Vec::new();
        for i in 0..elves.len() {
            let proposed = get_proposed_move(&field, &elves[i], index);
            if proposed.is_none() {
                continue;
            }
            if proposes_field[proposed.unwrap()[0] as usize][proposed.unwrap()[1] as usize].is_some() {
                proposes_field[proposed.unwrap()[0] as usize][proposed.unwrap()[1] as usize] = Some(-1);
            } else {
                proposes_field[proposed.unwrap()[0] as usize][proposed.unwrap()[1] as usize] = Some(i as i32);
                proposes.push(proposed.unwrap());
            }
        }

        let mut moved = false;
        for proposed in proposes {
            moved = true;
            if proposes_field[proposed[0] as usize][proposed[1] as usize].unwrap() == -1 {
                continue;
            }
            let elf_idx = proposes_field[proposed[0] as usize][proposed[1] as usize].unwrap() as usize;
            field[elves[elf_idx][0] as usize][elves[elf_idx][1] as usize] = false;
            field[proposed[0] as usize][proposed[1] as usize] = true;
            elves[elf_idx] = proposed;
        }
        if index == 9 {
            print_part1(&elves);
        }
        if !moved {
            println!("{}", index + 1);
            break;
        }
    }
}
