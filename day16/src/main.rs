use priority_queue::PriorityQueue;
use std::cmp::max;
use std::cmp::min;
use std::collections::VecDeque;
use std::fs::File;
use std::io::{prelude::*, BufReader};
use std::str;

fn main() {
    let file = File::open("input.txt").unwrap();
    let reader = BufReader::new(file);

    let lines: Vec<Vec<u8>> = reader.lines().map(|l| l.unwrap().into_bytes()).collect();

    let (mut rooms, start_idx) = parse_input(&lines);
    optimize_rooms(&mut rooms);

    part1(&rooms, start_idx);
    part2(&rooms, start_idx);
}

struct Room {
    flow_rate: i32,
    connections: Vec<usize>, //room index, cost
    distances: Vec<i32>,
}

#[derive(PartialEq, Eq, Hash)]
struct State {
    players: Vec<Player>,
    score: i32,
    state: u64,
    time_left: i32,
    flow_rate: i32,
}

#[derive(PartialEq, Eq, Hash, Clone)]
struct Player {
    cur_room_idx: usize,
    moves_till_target: i32,
}

impl State {
    fn calculate_theoretical_max(&self, rooms: &Vec<Room>) -> i32 {
        let mut final_score = self.score + (self.flow_rate * self.time_left);

        for player_current in &self.players {
            final_score += rooms[player_current.cur_room_idx].flow_rate * max(0, self.time_left - player_current.moves_till_target);
        }

        for i in 0..rooms.len() {
            if rooms[i].flow_rate == 0 || self.state & (1 << i) != 0 {
                continue;
            }
            let min_dist = self
                .players
                .iter()
                .map(|p| rooms[p.cur_room_idx].distances[i] + p.moves_till_target)
                .min()
                .unwrap();
            let cur_time_left = max(0, (self.time_left - min_dist) - 1);
            final_score += cur_time_left * rooms[i].flow_rate;
        }
        final_score
    }

    fn get_player_sub_states(&self, rooms: &Vec<Room>, player_idx: usize) -> Vec<Player> {
        let mut player_states = Vec::new();
        if player_idx >= self.players.len() {
            return player_states;
        }
        if self.players[player_idx].moves_till_target > 0 {
            player_states.push(self.players[player_idx].clone());
            return player_states;
        }
        for room_idx in 0..rooms.len() {
            if self.state & (1 << room_idx) == 0 && rooms[room_idx].flow_rate != 0 {
                let elapsed = rooms[self.players[player_idx].cur_room_idx].distances[room_idx] + 1;
                if elapsed > self.time_left {
                    continue;
                }

                player_states.push(Player {
                    cur_room_idx: room_idx,
                    moves_till_target: elapsed,
                });
            }
        }

        if player_states.is_empty() {
            //stay here till the end
            player_states.push(Player {
                cur_room_idx: self.players[player_idx].cur_room_idx,
                moves_till_target: self.time_left,
            });
        }
        player_states
    }

    fn get_sub_states(&self, rooms: &Vec<Room>) -> Vec<State> {
        let mut sub_states = Vec::new();

        let mut new_flow_rate = self.flow_rate;
        for player_current in &self.players {
            if player_current.moves_till_target == 0 {
                new_flow_rate += rooms[player_current.cur_room_idx].flow_rate;
            }
        }

        let player0_new_states = self.get_player_sub_states(rooms, 0);
        let player1_new_states = self.get_player_sub_states(rooms, 1);

        for player0_new_state in &player0_new_states {
            for player1_new_state in &player1_new_states {
                if player0_new_state.cur_room_idx == player1_new_state.cur_room_idx
                    || (self.players[0].cur_room_idx == self.players[1].cur_room_idx && player1_new_state.cur_room_idx < player0_new_state.cur_room_idx)
                {
                    continue;
                }

                let min_steps = min(player0_new_state.moves_till_target, player1_new_state.moves_till_target);

                sub_states.push(State {
                    players: vec![
                        Player {
                            cur_room_idx: player0_new_state.cur_room_idx,
                            moves_till_target: player0_new_state.moves_till_target - min_steps,
                        },
                        Player {
                            cur_room_idx: player1_new_state.cur_room_idx,
                            moves_till_target: player1_new_state.moves_till_target - min_steps,
                        },
                    ],
                    score: self.score + (min_steps * new_flow_rate),
                    state: self.state | (1 << player0_new_state.cur_room_idx) | (1 << player1_new_state.cur_room_idx),
                    time_left: self.time_left - min_steps,
                    flow_rate: new_flow_rate,
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

    rooms[room_idx].distances = distances;
}

fn optimize_rooms(rooms: &mut Vec<Room>) {
    for i in 0..rooms.len() {
        calculate_all_connections(rooms, i);
    }
}

fn calculate_max(rooms: &Vec<Room>, start_idx: usize, total_time: i32, player_count: usize) -> i32 {
    if player_count > 2 {
        panic!("too many players!");
    }
    let mut max_val = 0;
    let mut queue: PriorityQueue<State, i32> = PriorityQueue::new();

    let mut players = Vec::new();
    for _ in 0..player_count {
        players.push(Player {
            cur_room_idx: start_idx,
            moves_till_target: -1,
        });
    }

    for _ in player_count..2 {
        let mut empty_room = 0;
        for i in 0..rooms.len() {
            if i != start_idx && rooms[i].flow_rate == 0 {
                empty_room = i;
                break;
            }
        }

        players.push(Player {
            cur_room_idx: empty_room,
            moves_till_target: 30,
        });
    }

    let initial_state = State {
        players: players,
        score: 0,
        state: 0,
        time_left: total_time,
        flow_rate: 0,
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

fn part1(rooms: &Vec<Room>, start_idx: usize) {
    println!("{}", calculate_max(&rooms, start_idx, 30, 1));
}

fn part2(rooms: &Vec<Room>, start_idx: usize) {
    println!("{}", calculate_max(&rooms, start_idx, 26, 2));
}
