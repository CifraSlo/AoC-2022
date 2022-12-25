use std::fs::File;
use std::io::{prelude::*, BufReader};

const SIZE: usize = 15000;
const WIDTH: usize = 7;

const MOVES_PART_1: usize = 2022;
const MOVES_PART_2: usize = 1000000000000;

struct GameState {
    rocks: Vec<Vec<(usize, usize)>>,
    rock_idx: usize,
    moves: Vec<bool>,
    move_idx: usize,
    max_height: usize,
    field: [[bool; WIDTH + 2]; SIZE],
}

impl GameState {
    pub fn new(rocks: Vec<Vec<(usize, usize)>>, moves: Vec<bool>) -> Self {
        let mut field = [[false; WIDTH + 2]; SIZE];
        for y in 1..SIZE {
            field[y][0] = true;
            field[y][WIDTH + 1] = true;
        }
        for x in 0..(WIDTH + 2) {
            field[0][x] = true;
        }

        GameState {
            rocks,
            rock_idx: 0,
            moves,
            move_idx: 0,
            max_height: 0,
            field,
        }
    }

    fn get_next_rock(&mut self) -> Vec<(usize, usize)> {
        let rock = &self.rocks[self.rock_idx];
        self.rock_idx = (self.rock_idx + 1) % self.rocks.len();
        rock.clone()
    }

    fn get_next_move(&mut self) -> bool {
        let move_left = self.moves[self.move_idx];
        self.move_idx = (self.move_idx + 1) % self.moves.len();
        move_left
    }

    fn check_if_valid(&self, rock: &Vec<(usize, usize)>, offset_y: usize, offset_x: usize) -> bool {
        for coord in rock {
            if self.field[coord.0 + offset_y][coord.1 + offset_x] {
                return false;
            }
        }
        true
    }

    fn finalize_position(&mut self, rock: &Vec<(usize, usize)>, offset_y: usize, offset_x: usize) {
        for coords in rock {
            self.field[coords.0 + offset_y][coords.1 + offset_x] = true;
            if coords.0 + offset_y > self.max_height {
                self.max_height = coords.0 + offset_y;
            }
        }
    }

    fn process_next_rock(&mut self) {
        let mut offset_y = self.max_height + 4;
        let mut offset_x = 3;
        let rock = self.get_next_rock();

        loop {
            let move_left = self.get_next_move();
            if move_left && self.check_if_valid(&rock, offset_y, offset_x - 1) {
                offset_x -= 1;
            }
            if !move_left && self.check_if_valid(&rock, offset_y, offset_x + 1) {
                offset_x += 1;
            }
            if self.check_if_valid(&rock, offset_y - 1, offset_x) {
                offset_y -= 1;
            } else {
                self.finalize_position(&rock, offset_y, offset_x);
                break;
            }
        }
    }
}

fn read_moves_input() -> Vec<bool> {
    let file = File::open("input.txt").unwrap();
    let reader = BufReader::new(file);

    reader.lines().next().unwrap().unwrap().into_bytes().iter().map(|m| *m == '<' as u8).collect()
}

fn read_shapes_input() -> Vec<Vec<(usize, usize)>> {
    let file = File::open("shapes.txt").unwrap();
    let reader = BufReader::new(file);

    let mut rocks = Vec::new();
    let mut current: Vec<(usize, usize)> = Vec::new();
    for line in reader.lines().map(|l| l.unwrap().into_bytes()) {
        if line.is_empty() {
            rocks.push(current);
            current = Vec::new();
            continue;
        }
        for i in 0..current.len() {
            current[i].0 += 1;
        }

        for x in 0..line.len() {
            if line[x] == '#' as u8 {
                current.push((0, x));
            }
        }
    }
    rocks.push(current);

    rocks
}

fn main() {
    let moves = read_moves_input();
    let rocks = read_shapes_input();

    part1(rocks.clone(), moves.clone());
    part2(rocks, moves);
}

fn part1(rocks: Vec<Vec<(usize, usize)>>, moves: Vec<bool>) {
    let mut game = GameState::new(rocks, moves);
    for _ in 0..MOVES_PART_1 {
        game.process_next_rock();
    }
    println!("{}", game.max_height);
}

fn part2(rocks: Vec<Vec<(usize, usize)>>, moves: Vec<bool>) {
    let num_moves = moves.len();
    let mut game = GameState::new(rocks, moves);

    let mut already_seen_move = vec![false; num_moves];
    let mut rock_num = 0;

    for i in 0usize.. {
        if game.rock_idx == 0 {
            if already_seen_move[game.move_idx] {
                rock_num = i;
                break;
            } else {
                already_seen_move[game.move_idx] = true;
            }
        }
        game.process_next_rock();
    }

    let common_move_idx = game.move_idx;

    let mut rock_num_1 = None;
    let mut height_1 = None;
    let mut rock_num_delta = 0;
    let mut height_delta = 0;

    for i in rock_num.. {
        if game.rock_idx == 0 && game.move_idx == common_move_idx {
            if rock_num_1.is_none() {
                rock_num_1 = Some(i);
                height_1 = Some(game.max_height);
            } else {
                rock_num_delta = i - rock_num_1.unwrap();
                height_delta = game.max_height - height_1.unwrap();
                rock_num = i;
                break;
            }
        }
        game.process_next_rock();
    }

    let rocks_left = MOVES_PART_2 - rock_num;
    let repeats = rocks_left / rock_num_delta;
    let remains = rocks_left % rock_num_delta;

    for _ in 0..remains {
        game.process_next_rock();
    }

    let result = game.max_height + (repeats * height_delta);

    println!("{}", result);
}
