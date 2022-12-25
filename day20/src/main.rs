use std::fs::File;
use std::io::{prelude::*, BufReader};

struct LinkedList {
    nodes: Vec<Node>,
    zero_pos: usize,
    len: i64,
}

struct Node {
    value: i64,
    neighbor_idx: [usize; 2],
}

impl Node {
    fn get_value_and_direction(&self, full: i64) -> (usize, usize) {
        let half = full / 2;
        let value = (self.value + half).rem_euclid(full) - half;
        if value < 0 {
            return ((-value) as usize, 0);
        }
        (value as usize, 1)
    }
}

impl LinkedList {
    pub fn new(numbers: &Vec<i64>, decryption_key: i64) -> Self {
        let mut nodes = Vec::new();
        nodes.push(Node {
            value: numbers[0] * decryption_key,
            neighbor_idx: [numbers.len() - 1, 1],
        });
        for i in 1..(numbers.len() - 1) {
            nodes.push(Node {
                value: numbers[i] * decryption_key,
                neighbor_idx: [i - 1, i + 1],
            });
        }
        nodes.push(Node {
            value: numbers[numbers.len() - 1] * decryption_key,
            neighbor_idx: [numbers.len() - 2, 0],
        });
        Self {
            nodes,
            zero_pos: 0,
            len: numbers.len() as i64,
        }
    }

    fn move_nodes(&mut self) {
        let nodes = &mut self.nodes;
        for i in 0..self.len as usize {
            if nodes[i].value == 0 {
                self.zero_pos = i;
                continue;
            }
            let (val, dir) = nodes[i].get_value_and_direction(self.len - 1);
            let neighbor_idx = nodes[i].neighbor_idx;

            nodes[neighbor_idx[0] as usize].neighbor_idx[1] = neighbor_idx[1];
            nodes[neighbor_idx[1] as usize].neighbor_idx[0] = neighbor_idx[0];

            let mut left = neighbor_idx[0];
            for _ in 0..val {
                left = nodes[left].neighbor_idx[dir];
            }

            let right = nodes[left].neighbor_idx[1];
            nodes[i].neighbor_idx[0] = left;
            nodes[i].neighbor_idx[1] = right;
            nodes[left].neighbor_idx[1] = i;
            nodes[right].neighbor_idx[0] = i;
        }
    }

    fn get_result(&self) -> i64 {
        let mut sum = 0;
        let mut current = self.zero_pos;
        for i in 0..3001 {
            if i % 1000 == 0 {
                sum += self.nodes[current].value;
            }
            current = self.nodes[current].neighbor_idx[1];
        }
        sum
    }
}

fn main() {
    let file = File::open("input.txt").unwrap();
    let reader = BufReader::new(file);

    let numbers: Vec<i64> = reader.lines().map(|l| l.unwrap().parse().unwrap()).collect();

    part1(&numbers);
    part2(&numbers);
}

fn part1(numbers: &Vec<i64>) {
    let mut list = LinkedList::new(numbers, 1);
    list.move_nodes();
    println!("{}", list.get_result());
}

fn part2(numbers: &Vec<i64>) {
    let mut list = LinkedList::new(numbers, 811589153);
    for _ in 0..10 {
        list.move_nodes();
    }
    println!("{}", list.get_result());
}
