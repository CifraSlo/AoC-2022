use priority_queue::PriorityQueue;
use std::cmp::Reverse;
use std::fs::File;
use std::io::{prelude::*, BufReader};

struct Blizzard {
    pos: [usize; 2],
    diff: [i32; 2],
}

impl Blizzard {
    pub fn new(pos: [usize; 2], dir: char) -> Self {
        let diff = match dir {
            '>' => [0, 1],
            '<' => [0, -1],
            'v' => [1, 0],
            '^' => [-1, 0],
            _ => panic!("how?"),
        };
        Blizzard { pos, diff }
    }
    fn do_move(&mut self, max_y: usize, max_x: usize) {
        self.pos[0] = (((self.pos[0] as i32 - 1) + self.diff[0]).rem_euclid(max_y as i32) + 1) as usize;
        self.pos[1] = (((self.pos[1] as i32 - 1) + self.diff[1]).rem_euclid(max_x as i32) + 1) as usize;
    }
}

struct Field {
    fields: Vec<Vec<Vec<bool>>>,
    blizzards: Vec<Blizzard>,
    base_field: Vec<Vec<bool>>,
}

impl Field {
    fn generate_next_moves(&mut self) {
        let mut new_field = self.base_field.clone();
        for blizzard in &mut self.blizzards {
            blizzard.do_move(self.base_field.len() - 2, self.base_field[0].len() - 2);
            new_field[blizzard.pos[0]][blizzard.pos[1]] = true;
        }
        self.fields.push(new_field);
    }

    fn is_state_valid(&mut self, state: &State) -> bool {
        if state.time == self.fields.len() {
            self.generate_next_moves();
        }
        if state.pos[0] < 0 || state.pos[0] > ((self.base_field.len() - 1) as i32) || self.fields[state.time][state.pos[0] as usize][state.pos[1] as usize] {
            return false;
        }
        true
    }
}

#[derive(PartialEq, Eq, Hash, Clone)]
struct State {
    pos: [i32; 2],
    time: usize,
}

impl State {
    fn get_score(&self, target: [i32; 2]) -> u32 {
        self.time as u32 + self.pos[0].abs_diff(target[0]) + self.pos[1].abs_diff(target[1])
    }
    fn get_sub_states(&self) -> Vec<State> {
        vec![
            State {
                pos: [self.pos[0], self.pos[1]],
                time: self.time + 1,
            },
            State {
                pos: [self.pos[0] - 1, self.pos[1]],
                time: self.time + 1,
            },
            State {
                pos: [self.pos[0] + 1, self.pos[1]],
                time: self.time + 1,
            },
            State {
                pos: [self.pos[0], self.pos[1] - 1],
                time: self.time + 1,
            },
            State {
                pos: [self.pos[0], self.pos[1] + 1],
                time: self.time + 1,
            },
        ]
    }
}

fn init_base_field(height: usize, width: usize) -> Vec<Vec<bool>> {
    let mut base_field = vec![vec![false; width]; height];
    for y in 0..height {
        base_field[y][0] = true;
        base_field[y][width - 1] = true;
    }
    for x in 1..(width - 1) {
        base_field[0][x] = true;
        base_field[height - 1][x] = true;
    }
    base_field[0][1] = false;
    base_field[height - 1][width - 2] = false;

    base_field
}

fn get_min_time(field: &mut Field, start: [i32; 2], target: [i32; 2], start_time: usize) -> u32 {
    let mut queue = PriorityQueue::new();
    queue.push(State { pos: start, time: start_time }, Reverse(1));

    let mut shortest = u32::MAX;

    while !queue.is_empty() {
        let (state, score) = queue.pop().unwrap();
        if score.0 >= shortest {
            break;
        }

        if state.pos[0] == target[0] {
            shortest = score.0;
            continue;
        }

        for sub_state in state.get_sub_states() {
            if field.is_state_valid(&sub_state) {
                let score = sub_state.get_score(target);
                queue.push(sub_state, Reverse(score));
            }
        }
    }
    shortest
}

fn main() {
    let file = File::open("input.txt").unwrap();
    let reader = BufReader::new(file);

    let lines: Vec<Vec<u8>> = reader.lines().map(|l| l.unwrap().into_bytes()).collect();
    let base_field = init_base_field(lines.len(), lines[0].len());
    let mut initial_field = base_field.clone();
    let mut blizzards = Vec::new();
    for y in 1..(lines.len() - 1) {
        for x in 1..(lines[y].len() - 1) {
            if lines[y][x] != '.' as u8 {
                blizzards.push(Blizzard::new([y, x], lines[y][x] as char));
                initial_field[y][x] = true;
            }
        }
    }
    let mut field = Field {
        fields: vec![initial_field],
        blizzards,
        base_field,
    };

    let start = [0, 1];
    let target = [(field.base_field.len() - 1) as i32, (field.base_field[0].len() - 2) as i32];

    let mut elapsed = get_min_time(&mut field, start, target, 0);
    println!("{elapsed}");
    elapsed = get_min_time(&mut field, target, start, elapsed as usize);
    elapsed = get_min_time(&mut field, start, target, elapsed as usize);
    println!("{elapsed}");
}
