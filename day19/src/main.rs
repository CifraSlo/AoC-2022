use priority_queue::PriorityQueue;
use std::cmp::min;
use std::fs::File;
use std::io::{prelude::*, BufReader};

const ELEMENTS_COUNT: usize = 4;
const GEODE_IDX: usize = 3;

#[derive(PartialEq, Eq, Hash, Clone)]
struct Elements {
    elements: [i32; ELEMENTS_COUNT],
}

impl Elements {
    fn added(&self, other: &Elements) -> Elements {
        let mut new = self.clone();
        for i in 0..ELEMENTS_COUNT {
            new.elements[i] += other.elements[i];
        }
        new
    }
    fn add(&mut self, other: &Elements) {
        for i in 0..ELEMENTS_COUNT {
            self.elements[i] += other.elements[i];
        }
    }
    fn add_multiple(&mut self, other: &Elements, factor: i32) {
        for i in 0..ELEMENTS_COUNT {
            self.elements[i] += other.elements[i] * factor;
        }
    }
    fn subtracted(&self, other: &Elements) -> Elements {
        let mut new = self.clone();
        for i in 0..ELEMENTS_COUNT {
            new.elements[i] -= other.elements[i];
        }
        new
    }
    fn contains(&self, other: &Elements) -> bool {
        for i in 0..ELEMENTS_COUNT {
            if self.elements[i] < other.elements[i] {
                return false;
            }
        }
        true
    }
    fn add_one(&mut self, index: usize) {
        self.elements[index] += 1;
    }
    fn added_one(&self, index: usize) -> Elements {
        let mut new = self.clone();
        new.elements[index] += 1;
        new
    }
    fn time_until_available(&self, cost: &Elements, production: &Elements) -> i32 {
        let mut balance = self.clone();
        for i in 1..25 {
            if balance.contains(cost) {
                return i;
            }
            balance.add(production);
        }
        30
    }
}

#[derive(PartialEq, Eq, Hash, Clone)]
struct State {
    blueprint: [Elements; ELEMENTS_COUNT],
    balance: Elements,
    production: Elements,
    time_left: i32,
}

impl State {
    fn get_max_balance(&self, element_idx: usize) -> Vec<i32> {
        if element_idx == 0 {
            return vec![99999; self.time_left as usize];
        }
        let mut state = self.clone();

        let mut result = Vec::new();
        let prev_max_balance = self.get_max_balance(element_idx - 1);
        let mut spent = 0;
        for i in 0..self.time_left as usize {
            result.push(state.balance.elements[element_idx]);
            state.balance.add(&state.production);
            if prev_max_balance[i] - spent >= self.blueprint[element_idx].elements[element_idx - 1] {
                state.production.add_one(element_idx);
                spent += self.blueprint[element_idx].elements[element_idx - 1];
            }
        }
        result.push(state.balance.elements[element_idx]);
        result
    }

    fn get_sub_states(&self) -> Vec<State> {
        let mut result = Vec::new();

        if self.time_left == 0 {
            return result;
        }

        for i in 0..ELEMENTS_COUNT {
            let time_until_available = self.balance.time_until_available(&self.blueprint[i], &self.production);
            if time_until_available > self.time_left - 1 {
                continue;
            }
            let mut balance = self.balance.subtracted(&self.blueprint[i]);
            balance.add_multiple(&self.production, time_until_available);
            result.push(State {
                blueprint: self.blueprint.clone(),
                balance,
                production: self.production.added_one(i),
                time_left: self.time_left - time_until_available,
            })
        }

        if result.is_empty() {
            let mut final_balance = self.balance.clone();
            final_balance.add_multiple(&self.production, self.time_left);
            result.push(State {
                blueprint: self.blueprint.clone(),
                balance: final_balance,
                production: self.production.clone(),
                time_left: 0,
            });
        }

        result
    }

    fn get_sub_states_old(&self) -> Vec<State> {
        let mut result = Vec::new();

        if self.time_left == 0 {
            return result;
        }

        let new_balance = self.balance.added(&self.production);

        for i in 0..ELEMENTS_COUNT {
            if self.balance.contains(&self.blueprint[i]) {
                result.push(State {
                    blueprint: self.blueprint.clone(),
                    balance: new_balance.subtracted(&self.blueprint[i]),
                    production: self.production.added_one(i),
                    time_left: self.time_left - 1,
                })
            }
        }

        result.push(State {
            blueprint: self.blueprint.clone(),
            balance: new_balance,
            production: self.production.clone(),
            time_left: self.time_left - 1,
        });

        result
    }
}

fn main() {
    let file = File::open("input.txt").unwrap();
    let reader = BufReader::new(file);

    let lines = reader.lines().map(|l| l.unwrap());

    let mut blueprints = Vec::new();

    for line in lines {
        let split: Vec<&str> = line.split(' ').collect();
        blueprints.push([
            Elements {
                elements: [split[6].parse().unwrap(), 0, 0, 0],
            },
            Elements {
                elements: [split[12].parse().unwrap(), 0, 0, 0],
            },
            Elements {
                elements: [split[18].parse().unwrap(), split[21].parse().unwrap(), 0, 0],
            },
            Elements {
                elements: [split[27].parse().unwrap(), 0, split[30].parse().unwrap(), 0],
            },
        ]);
    }

    part1(blueprints.clone());
    part2(blueprints);
}

fn calculate_max(blueprint: [Elements; ELEMENTS_COUNT], total_time: i32) -> i32 {
    let mut queue: PriorityQueue<State, i32> = PriorityQueue::new();

    queue.push(
        State {
            blueprint,
            balance: Elements { elements: [0, 0, 0, 0] },
            production: Elements { elements: [1, 0, 0, 0] },
            time_left: total_time,
        },
        1,
    );

    let mut max_val = 0;

    while !queue.is_empty() {
        let (current, score) = queue.pop().unwrap();

        if score <= max_val {
            return max_val;
        }

        if current.balance.elements[GEODE_IDX] > max_val {
            max_val = current.balance.elements[GEODE_IDX];
        }

        let sub_states = current.get_sub_states();
        for sub_state in sub_states {
            let score = *sub_state.get_max_balance(GEODE_IDX).last().unwrap();
            if score <= max_val {
                continue;
            }
            queue.push(sub_state, score);
        }
    }
    max_val
}

fn part1(blueprints: Vec<[Elements; ELEMENTS_COUNT]>) {
    let mut score = 0;
    for i in 0..blueprints.len() {
        let result = calculate_max(blueprints[i].clone(), 24);
        score += (i as i32 + 1) * result;
    }
    println!("{score}");
}

fn part2(blueprints: Vec<[Elements; ELEMENTS_COUNT]>) {
    let mut score = 1;
    for i in 0..min(blueprints.len(), 3) {
        let result = calculate_max(blueprints[i].clone(), 32);
        score *= result;
    }
    println!("{score}");
}
