use core::panic;
use std::collections::HashMap;
use std::fs::File;
use std::io::{prelude::*, BufReader};

#[derive(PartialEq, Eq, Hash, Clone)]
enum Operation {
    ADD,
    SUBTRACT,
    MULTIPLY,
    DIVIDE,
}

#[derive(PartialEq, Eq, Hash, Clone)]
struct Monkey {
    value: Option<i64>,
    operator1: String,
    operator2: String,
    operation: Operation,
}

fn get_value(monkey_name: String, monkeys: &HashMap<String, Monkey>) -> i64 {
    let monkey = monkeys.get(&monkey_name).unwrap();
    if monkey.value.is_some() {
        return monkey.value.unwrap();
    }
    let operation = monkey.operation.clone();
    let operator1_name = monkey.operator1.clone();
    let operator2_name = monkey.operator2.clone();
    let operator1 = get_value(operator1_name, monkeys);
    let operator2 = get_value(operator2_name, monkeys);
    match operation {
        Operation::ADD => operator1 + operator2,
        Operation::SUBTRACT => operator1 - operator2,
        Operation::MULTIPLY => operator1 * operator2,
        Operation::DIVIDE => operator1 / operator2,
    }
}

fn solve_part_2(monkeys: &mut HashMap<String, Monkey>) -> i64 {
    get_value_part2("root".to_string(), monkeys);
    let root_monkey = monkeys.get(&"root".to_string()).unwrap();
    let operator1_name = root_monkey.operator1.clone();
    let operator2_name = root_monkey.operator2.clone();
    let operator1 = monkeys.get(&operator1_name).unwrap().value;
    let operator2 = monkeys.get(&operator2_name).unwrap().value;
    if operator1.is_none() {
        return get_required_value(operator1_name, monkeys, operator2.unwrap());
    } else {
        return get_required_value(operator2_name, monkeys, operator1.unwrap());
    }
}

fn get_value_part2(monkey_name: String, monkeys: &mut HashMap<String, Monkey>) -> Option<i64> {
    let monkey = monkeys.get(&monkey_name).unwrap();
    if monkey_name == "humn" {
        monkeys.get_mut(&monkey_name).unwrap().value = None;
        return None;
    }
    if monkey.value.is_some() {
        return Some(monkey.value.unwrap());
    }
    let operation = monkey.operation.clone();
    let operator1_name = monkey.operator1.clone();
    let operator2_name = monkey.operator2.clone();
    let operator1 = get_value_part2(operator1_name, monkeys);
    let operator2 = get_value_part2(operator2_name, monkeys);
    if operator1.is_none() || operator2.is_none() {
        return None;
    }
    let result = match operation {
        Operation::ADD => operator1.unwrap() + operator2.unwrap(),
        Operation::SUBTRACT => operator1.unwrap() - operator2.unwrap(),
        Operation::MULTIPLY => operator1.unwrap() * operator2.unwrap(),
        Operation::DIVIDE => operator1.unwrap() / operator2.unwrap(),
    };
    monkeys.get_mut(&monkey_name).unwrap().value = Some(result);
    Some(result)
}

fn get_required_value(monkey_name: String, monkeys: &HashMap<String, Monkey>, required_value: i64) -> i64 {
    if monkey_name == "humn" {
        return required_value;
    }

    let monkey = monkeys.get(&monkey_name).unwrap();
    let operation = monkey.operation.clone();
    let operator1_name = monkey.operator1.clone();
    let operator2_name = monkey.operator2.clone();
    let operator1 = monkeys.get(&operator1_name).unwrap().value;
    let operator2 = monkeys.get(&operator2_name).unwrap().value;

    if operator1.is_none() {
        let required_subvalue = match operation {
            Operation::ADD => required_value - operator2.unwrap(),
            Operation::SUBTRACT => required_value + operator2.unwrap(),
            Operation::MULTIPLY => required_value / operator2.unwrap(),
            Operation::DIVIDE => required_value * operator2.unwrap(),
        };
        return get_required_value(operator1_name, monkeys, required_subvalue);
    } else {
        let required_subvalue = match operation {
            Operation::ADD => required_value - operator1.unwrap(),
            Operation::SUBTRACT => operator1.unwrap() - required_value,
            Operation::MULTIPLY => required_value / operator1.unwrap(),
            Operation::DIVIDE => operator1.unwrap() / required_value,
        };
        return get_required_value(operator2_name, monkeys, required_subvalue);
    }
}

fn main() {
    let file = File::open("input.txt").unwrap();
    let reader = BufReader::new(file);

    let lines = reader.lines().map(|l| l.unwrap());
    let mut monkeys = HashMap::new();
    for line in lines {
        let name = line[0..4].to_string();
        if line.as_bytes()[6] >= '0' as u8 && line.as_bytes()[6] <= '9' as u8 {
            let value = line[6..].parse().unwrap();
            monkeys.insert(
                name,
                Monkey {
                    value: Some(value),
                    operator1: String::new(),
                    operator2: String::new(),
                    operation: Operation::ADD,
                },
            );
        } else {
            let operation = match line.as_bytes()[11] as char {
                '+' => Operation::ADD,
                '-' => Operation::SUBTRACT,
                '*' => Operation::MULTIPLY,
                '/' => Operation::DIVIDE,
                _ => panic!("invalid"),
            };
            monkeys.insert(
                name,
                Monkey {
                    value: None,
                    operator1: line[6..10].to_string(),
                    operator2: line[13..17].to_string(),
                    operation,
                },
            );
        }
    }

    part1(&monkeys);
    part2(&mut monkeys);
}

fn part1(monkeys: &HashMap<String, Monkey>) {
    println!("{}", get_value("root".to_string(), monkeys));
}

fn part2(monkeys: &mut HashMap<String, Monkey>) {
    println!("{}", solve_part_2(monkeys));
}
