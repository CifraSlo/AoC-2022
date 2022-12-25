use std::collections::VecDeque;
use std::fs::File;
use std::io::{prelude::*, BufReader};

#[derive(Clone)]
struct Monkey {
    items: VecDeque<i64>,
    op_is_sum: bool,
    op_val: Option<i64>,
    divisible: i64,
    destinations: [usize; 2],
    total_inspected: usize,
}

fn process_round(monkeys: &mut Vec<Monkey>, div_amount: i64, common_mul: i64) {
    for i in 0..monkeys.len() {
        monkeys[i].total_inspected += monkeys[i].items.len();
        for _ in 0..monkeys[i].items.len() {
            let mut item = monkeys[i].items.pop_front().unwrap();
            let other;
            if monkeys[i].op_val.is_none() {
                other = item;
            } else {
                other = monkeys[i].op_val.unwrap();
            }
            if monkeys[i].op_is_sum {
                item += other;
            } else {
                item *= other;
            }

            item /= div_amount;
            item %= common_mul;

            let dest;

            if item % monkeys[i].divisible == 0 {
                dest = monkeys[i].destinations[0];
            } else {
                dest = monkeys[i].destinations[1];
            }

            monkeys[dest].items.push_back(item);
        }
    }
}

fn main() {
    let file = File::open("input.txt").unwrap();
    let reader = BufReader::new(file);

    let mut monkeys = Vec::new();
    let lines: Vec<String> = reader.lines().map(|l| l.unwrap()).collect();

    for i in (0..lines.len()).step_by(7) {
        let items_line: Vec<&str> = lines[i + 1].split(", ").collect();
        let mut items: VecDeque<i64> = (&items_line[1..]).iter().map(|i| i.parse().unwrap()).collect();
        items.push_front(items_line[0][18..].parse().unwrap());
        let op_is_sum = lines[i + 2].as_bytes()[23] == '+' as u8;
        let op_is_val = !(lines[i + 2].as_bytes()[25] == 'o' as u8);
        let op_val;
        if op_is_val {
            op_val = Some(lines[i + 2][25..].parse().unwrap());
        } else {
            op_val = None;
        }
        let divisible = lines[i + 3][21..].parse().unwrap();
        let dest1 = lines[i + 4][29..].parse().unwrap();
        let dest2 = lines[i + 5][30..].parse().unwrap();

        monkeys.push(Monkey {
            items,
            op_is_sum,
            op_val,
            divisible,
            destinations: [dest1, dest2],
            total_inspected: 0,
        });
    }

    part1(&mut monkeys.clone());
    part2(&mut monkeys);
}

fn part1(monkeys: &mut Vec<Monkey>) {
    for _ in 0..20 {
        process_round(monkeys, 3, 1);
    }
    let mut inspected: Vec<usize> = monkeys.into_iter().map(|m| m.total_inspected).collect();
    inspected.sort();

    println!("{}", inspected[inspected.len() - 1] * inspected[inspected.len() - 2]);
}

fn part2(monkeys: &mut Vec<Monkey>) {
    let mut common_multiplier = 1;
    for i in 0..monkeys.len() {
        common_multiplier *= monkeys[i].divisible;
    }

    for _ in 0..10000 {
        process_round(monkeys, 1, common_multiplier);
    }
    let mut inspected: Vec<usize> = monkeys.into_iter().map(|m| m.total_inspected).collect();
    inspected.sort();

    println!("{}", inspected[inspected.len() - 1] * inspected[inspected.len() - 2]);
}
