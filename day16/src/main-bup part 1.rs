use priority_queue::PriorityQueue;
use std::cmp::max;
use std::collections::VecDeque;
use std::fs::File;
use std::io::{prelude::*, BufReader};
use std::str;

fn main() {
    part1();
    part2();
}

struct Room {
    flow_rate: i32,
    connections: Vec<usize>, //room index, cost
    distances: Vec<i32>,
}

#[derive(PartialEq, Eq, Hash)]
struct State {
    cur_room_idx: usize,
    score: i32,
    state: u64,
    time_left: i32,
    flow_rate: i32,
    history: Vec<usize>,
}

impl State {
    fn calculate_theoretical_max(&self, rooms: &Vec<Room>) -> i32 {
        let mut final_score = self.score + (self.flow_rate * self.time_left);
        for i in 0..rooms.len() {
            if rooms[i].flow_rate == 0 || self.state & (1 << i) != 0 {
                continue;
            }
            let cur_time_left = max(0, (self.time_left - rooms[self.cur_room_idx].distances[i]) - 1);
            final_score += cur_time_left * rooms[i].flow_rate;
        }
        final_score
    }

    fn get_sub_states(&self, rooms: &Vec<Room>) -> Vec<State> {
        let mut sub_states = Vec::new();

        let mut new_history = self.history.clone();
        new_history.push(self.cur_room_idx);

        //staying here till the end
        sub_states.push(State {
            cur_room_idx: self.cur_room_idx,
            score: self.score + (self.flow_rate * self.time_left),
            state: self.state,
            time_left: 0,
            flow_rate: self.flow_rate,
            history: new_history.clone(),
        });
        /*
        if self.state & (1 << self.cur_room_idx) == 0 && rooms[self.cur_room_idx].flow_rate != 0 {
            sub_states.push(State {
                cur_room_idx: self.cur_room_idx,
                score: self.score + self.flow_rate,
                state: self.state | (1 << self.cur_room_idx),
                time_left: self.time_left - 1,
                flow_rate: self.flow_rate + rooms[self.cur_room_idx].flow_rate,
                history: new_history.clone(),
            });
        }

        for sub_room in &rooms[self.cur_room_idx].connections {
            let elapsed = 1;
            sub_states.push(State {
                cur_room_idx: *sub_room,
                score: self.score + (elapsed * self.flow_rate),
                state: self.state,
                time_left: self.time_left - elapsed,
                flow_rate: self.flow_rate,
                history: new_history.clone(),
            });
        }
        */

        for i in 0..rooms.len() {
            if self.state & (1 << i) == 0 && rooms[i].flow_rate != 0 {
                let elapsed = rooms[self.cur_room_idx].distances[i] + 1;
                if elapsed > self.time_left {
                    continue;
                }

                sub_states.push(State {
                    cur_room_idx: i,
                    score: self.score + (elapsed * self.flow_rate),
                    state: self.state | (1 << i),
                    time_left: self.time_left - elapsed,
                    flow_rate: self.flow_rate + rooms[i].flow_rate,
                    history: new_history.clone(),
                });
            }
        }

        sub_states
    }
}

fn parse_input(lines: &Vec<Vec<u8>>) -> (Vec<Room>, usize) {
    let mut rooms: Vec<Room> = Vec::new();
    let mut names: Vec<&[u8]> = Vec::new();
    let mut start_idx = 0;

    for line in lines {
        let mut semicolon = 0;
        for i in 0..line.len() {
            if line[i] == ';' as u8 {
                semicolon = i;
                break;
            }
        }

        let name = &line[6..8];
        let flow_rate = str::from_utf8(&line[23..semicolon]).unwrap().parse().unwrap();
        let mut connections: Vec<usize> = Vec::new();

        let mut con_offset = semicolon + 24;
        if line[con_offset] == ' ' as u8 {
            con_offset += 1;
        }

        for i in (con_offset..line.len()).step_by(4) {
            let connection_name = &line[i..(i + 2)];
            for j in 0..names.len() {
                if names[j] == connection_name {
                    connections.push(j);
                    rooms[j].connections.push(names.len());
                    break;
                }
            }
        }

        if name == ['A' as u8, 'A' as u8] {
            start_idx = names.len();
        }
        names.push(name);
        rooms.push(Room {
            flow_rate: flow_rate,
            connections: connections,
            distances: vec![i32::MAX; lines.len()],
        });
    }

    (rooms, start_idx)
}

fn calculate_all_connections(rooms: &mut Vec<Room>, room_idx: usize) {
    let mut distances = vec![i32::MAX; rooms.len()];
    distances[room_idx] = 0;

    let mut queue = VecDeque::new();
    queue.push_back(room_idx);

    while !queue.is_empty() {
        let current = queue.pop_front().unwrap();
        let current_distance = distances[current];
        for connection in &rooms[current].connections {
            if distances[*connection] == i32::MAX {
                distances[*connection] = current_distance + 1;
                queue.push_back(*connection);
            }
        }
    }

    for i in 0..distances.len() {
        if rooms[i].flow_rate == 0 {
            //distances[i] = -1;
        }
    }

    rooms[room_idx].distances = distances;
}

fn optimize_rooms(rooms: &mut Vec<Room>) {
    for i in 0..rooms.len() {
        calculate_all_connections(rooms, i);
    }
}

fn calculate_max(rooms: &Vec<Room>, start_idx: usize) -> i32 {
    let mut max_val = 0;
    let mut queue: PriorityQueue<State, i32> = PriorityQueue::new();

    let initial_state = State {
        cur_room_idx: start_idx,
        score: 0,
        state: 0,
        time_left: 30,
        flow_rate: 0,
        history: Vec::new(),
    };
    queue.push(initial_state, 1);

    while !queue.is_empty() {
        let (state, max) = queue.pop().unwrap();

        let mut enabled_valves = vec![false; rooms.len()];
        for i in 0..rooms.len() {
            if state.state & (1 << i) != 0 {
                enabled_valves[i] = true;
            }
        }

        if max < max_val {
            return max_val;
        }

        if state.time_left == 0 {
            if state.score > max_val {
                max_val = state.score;
            }
            continue;
        }

        let sub_states = state.get_sub_states(rooms);

        for sub_state in sub_states {
            let sub_max = sub_state.calculate_theoretical_max(rooms);
            queue.push(sub_state, sub_max);
        }
    }
    max_val
}

fn part1() {
    let file = File::open("input.txt").unwrap();
    let reader = BufReader::new(file);

    let lines: Vec<Vec<u8>> = reader.lines().map(|l| l.unwrap().into_bytes()).collect();

    let (mut rooms, start_idx) = parse_input(&lines);
    optimize_rooms(&mut rooms);

    let max = calculate_max(&rooms, start_idx);

    println!("{}", max);
}

fn part2() {}
