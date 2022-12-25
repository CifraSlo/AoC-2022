use std::fs::File;
use std::io::{prelude::*, BufReader};
use std::str;

fn main() {
    part1();
    part2();
}

fn parse_input_line(line: &Vec<u8>) -> (Sensor, i32, i32) {
    let mut comma1 = 0;
    let mut colon = 0;
    let mut comma2 = 0;
    for i in 0..line.len() {
        if line[i] == ',' as u8 {
            comma1 = i;
            break;
        }
    }
    for i in (comma1 + 1)..line.len() {
        if line[i] == ':' as u8 {
            colon = i;
            break;
        }
    }
    for i in (colon + 1)..line.len() {
        if line[i] == ',' as u8 {
            comma2 = i;
            break;
        }
    }

    let sensor_x: i32 = str::from_utf8(&line[12..comma1]).unwrap().parse().unwrap();
    let sensor_y: i32 = str::from_utf8(&line[(comma1 + 4)..colon]).unwrap().parse().unwrap();
    let beacon_x: i32 = str::from_utf8(&line[(colon + 25)..comma2]).unwrap().parse().unwrap();
    let beacon_y: i32 = str::from_utf8(&line[(comma2 + 4)..]).unwrap().parse().unwrap();

    let sensor = Sensor {
        x: sensor_x,
        y: sensor_y,
        range: (sensor_x - beacon_x).abs() + (sensor_y - beacon_y).abs(),
    };

    (sensor, beacon_x, beacon_y)
}

fn is_in_range_of_a_sensor(sensors: &Vec<Sensor>, x: i32, y: i32) -> bool {
    for sensor in sensors {
        if sensor.contains(x, y) {
            return true;
        }
    }
    false
}

struct Sensor {
    x: i32,
    y: i32,
    range: i32,
}

impl Sensor {
    fn contains(&self, x: i32, y: i32) -> bool {
        (self.x - x).abs() + (self.y - y).abs() <= self.range
    }
}

fn part1() {
    let file = File::open("input.txt").unwrap();
    let reader = BufReader::new(file);

    let lines: Vec<Vec<u8>> = reader.lines().map(|l| l.unwrap().into_bytes()).collect();
    let mut sensors: Vec<Sensor> = Vec::new();

    let mut sections_x: Vec<i32> = Vec::new();
    let mut beacon_x_on_target_y: Vec<i32> = Vec::new();

    let target_y = 2000000;

    for line in lines {
        let (sensor, beacon_x, beacon_y) = parse_input_line(&line);

        let min_x = sensor.x - sensor.range;
        let max_x = sensor.x + sensor.range;

        sections_x.push(max_x - (sensor.y - target_y).abs());
        sections_x.push(min_x + (sensor.y - target_y).abs());

        sensors.push(sensor);

        if beacon_y == target_y {
            beacon_x_on_target_y.push(beacon_x);
        }
    }
    sections_x.sort();
    sections_x.dedup();

    beacon_x_on_target_y.sort();
    beacon_x_on_target_y.dedup();

    let mut impossibles: i32 = -(beacon_x_on_target_y.len() as i32);

    let mut cur_left_x = sections_x[0];
    for i in 1..sections_x.len() {
        if is_in_range_of_a_sensor(&sensors, cur_left_x, target_y) {
            impossibles += (sections_x[i] - cur_left_x) + 1;
            cur_left_x = sections_x[i] + 1;
        } else {
            cur_left_x = sections_x[i];
        }
    }

    println!("{impossibles}");
}

fn intersection_point(positive_k_n: i64, negative_k_n: i64) -> (i64, i64) {
    let x = (negative_k_n - positive_k_n) / 2;
    let y = x + positive_k_n;

    (x, y)
}

const BOUND_MIN: i64 = 0;
const BOUND_MAX: i64 = 4000000;

fn part2() {
    let file = File::open("input.txt").unwrap();
    let reader = BufReader::new(file);

    let lines: Vec<Vec<u8>> = reader.lines().map(|l| l.unwrap().into_bytes()).collect();
    let mut sensors: Vec<Sensor> = Vec::new();

    let mut positive_k_line_n_s: Vec<i32> = Vec::new();
    let mut negative_k_line_n_s: Vec<i32> = Vec::new();

    for line in lines {
        let (sensor, _, _) = parse_input_line(&line);

        let min_x = sensor.x - sensor.range;
        let max_x = sensor.x + sensor.range;

        positive_k_line_n_s.push(sensor.y - max_x);
        positive_k_line_n_s.push(sensor.y - min_x);
        negative_k_line_n_s.push(sensor.y + max_x);
        negative_k_line_n_s.push(sensor.y + min_x);

        sensors.push(sensor);
    }
    positive_k_line_n_s.sort();
    positive_k_line_n_s.dedup();
    negative_k_line_n_s.sort();
    negative_k_line_n_s.dedup();

    let mut positive_k_matching_n_s = Vec::new();
    for i in 1..positive_k_line_n_s.len() {
        if positive_k_line_n_s[i] - positive_k_line_n_s[i - 1] == 2
            || (i < positive_k_line_n_s.len() - 2 && positive_k_line_n_s[i + 1] - positive_k_line_n_s[i - 1] == 2)
        {
            positive_k_matching_n_s.push(positive_k_line_n_s[i - 1] + 1);
        }
    }
    let mut negative_k_matching_n_s = Vec::new();
    for i in 1..negative_k_line_n_s.len() {
        if negative_k_line_n_s[i] - negative_k_line_n_s[i - 1] == 2
            || (i < negative_k_line_n_s.len() - 2 && negative_k_line_n_s[i + 1] - negative_k_line_n_s[i - 1] == 2)
        {
            negative_k_matching_n_s.push(negative_k_line_n_s[i - 1] + 1);
        }
    }

    for positive_k_matching_n in &positive_k_matching_n_s {
        for negative_k_matching_n in &negative_k_matching_n_s {
            let (x, y) = intersection_point(*positive_k_matching_n as i64, *negative_k_matching_n as i64);
            if x >= BOUND_MIN && x <= BOUND_MAX && y >= BOUND_MIN && y <= BOUND_MAX && !is_in_range_of_a_sensor(&sensors, x as i32, y as i32) {
                println!("{}", (x * 4000000) + y);
                break;
            }
        }
    }
}
