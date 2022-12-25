use std::fs::File;
use std::io::{prelude::*, BufReader};

const DIRECTIONS: [[i32; 2]; 4] = [[0, 1], [1, 0], [0, -1], [-1, 0]];

#[derive(PartialEq, Eq, Hash, Clone)]
struct Connection {
    quadrant_idx: usize,
    rotations: i32,
}

#[derive(PartialEq, Eq, Hash, Clone)]
struct Quadrant {
    connections: [Option<Connection>; 4],
    pos: Coord,
}

#[derive(PartialEq, Eq, Hash, Clone)]
struct Coord {
    x: i32,
    y: i32,
}

impl Coord {
    fn added(&self, other: &Coord) -> Coord {
        Coord {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
    fn subtracted(&self, other: &Coord) -> Coord {
        Coord {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
    fn wrapped(&self, size: i32) -> Coord {
        Coord {
            x: self.x.rem_euclid(size),
            y: self.y.rem_euclid(size),
        }
    }
    fn multiplied(&self, mul: i32) -> Coord {
        Coord {
            x: self.x * mul,
            y: self.y * mul,
        }
    }
    fn divided(&self, div: i32) -> Coord {
        Coord {
            x: self.x / div,
            y: self.y / div,
        }
    }
    fn rotated_right(&self) -> Coord {
        Coord { x: -self.y, y: self.x }
    }
    fn translated_right(&self, size: i32) -> Coord {
        let center_times_two = Coord { x: size - 1, y: size - 1 };
        self.multiplied(2)
            .subtracted(&center_times_two)
            .rotated_right()
            .added(&center_times_two)
            .divided(2)
    }
    #[inline]
    fn x_idx(&self) -> usize {
        self.x as usize
    }
    #[inline]
    fn y_idx(&self) -> usize {
        self.y as usize
    }
}

#[derive(PartialEq, Eq, Hash, Clone)]
struct Bounds {
    min: i32,
    max: i32,
}

#[derive(PartialEq, Eq, Hash, Clone)]
struct Board {
    field: Vec<Vec<bool>>,
    bounds_x: Vec<Bounds>,
    bounds_y: Vec<Bounds>,
    quadrant_size: i32,
    quadrants: Vec<Quadrant>,
    quadrant_field: [[Option<usize>; 4]; 4],
}

impl Board {
    fn get_next_pos(&self, player_pos: &Coord, player_dir: &Coord) -> Option<Coord> {
        let mut next_pos = player_pos.added(player_dir);
        if player_dir.y < 0 && next_pos.y < self.bounds_y[next_pos.x_idx()].min {
            next_pos.y = self.bounds_y[next_pos.x_idx()].max;
        } else if player_dir.y > 0 && next_pos.y > self.bounds_y[next_pos.x_idx()].max {
            next_pos.y = self.bounds_y[next_pos.x_idx()].min;
        } else if player_dir.x < 0 && next_pos.x < self.bounds_x[next_pos.y_idx()].min {
            next_pos.x = self.bounds_x[next_pos.y_idx()].max;
        } else if next_pos.x > self.bounds_x[next_pos.y_idx()].max {
            next_pos.x = self.bounds_x[next_pos.y_idx()].min;
        }

        match self.field[next_pos.y_idx()][next_pos.x_idx()] {
            true => None,
            false => Some(next_pos),
        }
    }

    fn get_next_pos_part2(&self, player_pos: &Coord, player_dir: i32) -> Option<(Coord, i32)> {
        let mut next_pos = player_pos.clone();
        let diff = DIRECTIONS[player_dir as usize];
        next_pos.y += diff[0];
        next_pos.x += diff[1];

        let mut do_shit = false;
        if diff[0] < 0 && next_pos.y < self.bounds_y[next_pos.x_idx()].min {
            do_shit = true;
        } else if diff[0] > 0 && next_pos.y > self.bounds_y[next_pos.x_idx()].max {
            do_shit = true;
        } else if diff[1] < 0 && next_pos.x < self.bounds_x[next_pos.y_idx()].min {
            do_shit = true;
        } else if next_pos.x > self.bounds_x[next_pos.y_idx()].max {
            do_shit = true;
        }

        let mut next_dir = player_dir.clone();

        if do_shit {
            (next_pos, next_dir) = self.get_quadrant_offset(player_pos, &next_pos, player_dir);
        }

        match self.field[next_pos.y_idx()][next_pos.x_idx()] {
            true => None,
            false => Some((next_pos, next_dir.clone())),
        }
    }

    fn get_quadrant_offset(&self, player_pos: &Coord, new_pos: &Coord, player_dir: i32) -> (Coord, i32) {
        let quadrant_idx = Coord {
            x: player_pos.x / self.quadrant_size,
            y: player_pos.y / self.quadrant_size,
        };

        let mut new_dir = player_dir.clone();

        let mut relative_coords = Coord {
            x: new_pos.x.rem_euclid(self.quadrant_size),
            y: new_pos.y.rem_euclid(self.quadrant_size),
        };

        let quad_idx = self.quadrant_field[quadrant_idx.y as usize][quadrant_idx.x as usize].unwrap();
        let connection = self.quadrants[quad_idx].connections[player_dir as usize].as_ref().unwrap();
        let final_quadrant = &self.quadrants[connection.quadrant_idx].pos;

        new_dir = (new_dir + connection.rotations).rem_euclid(4);

        for _ in 0..connection.rotations {
            relative_coords = relative_coords.translated_right(self.quadrant_size);
        }

        let next_pos = relative_coords
            .wrapped(self.quadrant_size)
            .added(&final_quadrant.multiplied(self.quadrant_size));

        (next_pos, new_dir)
    }
}

fn establish_connections(quadrants: &mut Vec<Quadrant>, mut total_connections: i32) {
    let mut quadrant_idx = 0;
    while total_connections < 12 {
        for conn_idx in 0..4i32 {
            if quadrants[quadrant_idx].connections[conn_idx as usize].is_some() {
                continue;
            }
            if quadrants[quadrant_idx].connections[(conn_idx - 1).rem_euclid(4) as usize].is_some() {
                let left_conn = quadrants[quadrant_idx].connections[(conn_idx - 1).rem_euclid(4) as usize].as_ref().unwrap();
                let left_quad = left_conn.quadrant_idx;
                let left_quad_right_conn = &quadrants[left_quad].connections[(conn_idx + left_conn.rotations).rem_euclid(4) as usize];
                if left_quad_right_conn.is_some() {
                    let left_quad_right_conn1 = left_quad_right_conn.as_ref().unwrap();
                    let other_quad_idx = left_quad_right_conn1.quadrant_idx;
                    let new_connection = Connection {
                        quadrant_idx: other_quad_idx,
                        rotations: (left_conn.rotations + left_quad_right_conn1.rotations - 1).rem_euclid(4),
                    };

                    quadrants[other_quad_idx].connections[(conn_idx + new_connection.rotations + 2).rem_euclid(4) as usize] = Some(Connection {
                        quadrant_idx,
                        rotations: (4 - new_connection.rotations).rem_euclid(4),
                    });
                    quadrants[quadrant_idx].connections[conn_idx as usize] = Some(new_connection);
                    total_connections += 1;
                }
            }
        }
        quadrant_idx = (quadrant_idx + 1) % quadrants.len();
    }
}

fn init_quadrants(board: &mut Board) {
    let mut quadrant_field = [[None; 4]; 4];
    let mut quadrants = Vec::new();
    for q_y in 0..4 {
        for q_x in 0..4 {
            let y = q_y * board.quadrant_size;
            let x = q_x * board.quadrant_size;
            if board.bounds_x.len() > y as usize && x >= board.bounds_x[y as usize].min && x <= board.bounds_x[y as usize].max {
                quadrant_field[q_y as usize][q_x as usize] = Some(quadrants.len());
                quadrants.push(Quadrant {
                    connections: [None, None, None, None],
                    pos: Coord { x: q_x, y: q_y },
                })
            }
        }
    }

    let mut total_connections = 0;
    for q_y in 0..quadrant_field.len() {
        for q_x in 0..quadrant_field[q_y].len() {
            if quadrant_field[q_y][q_x].is_none() {
                continue;
            }
            let cur_idx = quadrant_field[q_y][q_x].unwrap();
            if q_y > 0 && quadrant_field[q_y - 1][q_x].is_some() {
                total_connections += 1;
                let other_idx = quadrant_field[q_y - 1][q_x].unwrap();
                quadrants[cur_idx].connections[3] = Some(Connection {
                    quadrant_idx: other_idx,
                    rotations: 0,
                });
                quadrants[other_idx].connections[1] = Some(Connection {
                    quadrant_idx: cur_idx,
                    rotations: 0,
                });
            }
            if q_x > 0 && quadrant_field[q_y][q_x - 1].is_some() {
                total_connections += 1;
                let other_idx = quadrant_field[q_y][q_x - 1].unwrap();
                quadrants[cur_idx].connections[2] = Some(Connection {
                    quadrant_idx: other_idx,
                    rotations: 0,
                });
                quadrants[other_idx].connections[0] = Some(Connection {
                    quadrant_idx: cur_idx,
                    rotations: 0,
                });
            }
        }
    }

    establish_connections(&mut quadrants, total_connections);

    board.quadrant_field = quadrant_field;
    board.quadrants = quadrants;
}

fn main() {
    let file = File::open("input.txt").unwrap();
    let reader = BufReader::new(file);

    let lines: Vec<Vec<u8>> = reader.lines().map(|l| l.unwrap().into_bytes()).collect();
    let mut field: Vec<Vec<bool>> = vec![Vec::new(); lines.len() - 2];
    let mut bounds_y = Vec::new();
    let mut bounds_x = vec![Bounds { min: i32::MAX, max: i32::MIN }; lines.len() - 2];
    for y in 0..lines.len() {
        if lines[y].is_empty() {
            break;
        }

        field[y] = vec![true; lines[y].len()];

        for x in 0..lines[y].len() {
            if bounds_y.len() < x + 1 {
                bounds_y.push(Bounds { min: i32::MAX, max: i32::MIN });
            }
            if lines[y][x] == ' ' as u8 {
                continue;
            }

            field[y][x] = lines[y][x] == '#' as u8;

            if (y as i32) < bounds_y[x].min {
                bounds_y[x].min = y as i32;
            }
            if (y as i32) > bounds_y[x].max {
                bounds_y[x].max = y as i32;
            }
            if (x as i32) < bounds_x[y].min {
                bounds_x[y].min = x as i32;
            }
            if (x as i32) > bounds_x[y].max {
                bounds_x[y].max = x as i32;
            }
        }
    }

    let mut quadrant_size = i32::MAX;
    for bound in &bounds_x {
        if (bound.max - bound.min) + 1 < quadrant_size {
            quadrant_size = (bound.max - bound.min) + 1;
        }
    }

    let commands = &lines[lines.len() - 1];

    let board = Board {
        field,
        bounds_x,
        bounds_y,
        quadrant_size,
        quadrants: Vec::new(),
        quadrant_field: [[None; 4]; 4],
    };

    part1(board.clone(), &commands);
    part2(board, &commands);
}

fn part1(board: Board, commands: &Vec<u8>) {
    let mut facing_score: i32 = 0;
    let mut player_dir = Coord { x: 1, y: 0 };
    let mut player_pos = Coord { x: 0, y: 0 };
    for x in board.bounds_x[0].min..(board.bounds_x[0].max + 1) {
        if !board.field[0][x as usize] {
            player_pos = Coord { x, y: 0 };
            break;
        }
    }

    let mut num = 0;
    for cmd in commands {
        if *cmd >= '0' as u8 && *cmd <= '9' as u8 {
            num *= 10;
            num += (*cmd - '0' as u8) as i32;
        } else {
            for _ in 0..num {
                let next_pos = board.get_next_pos(&player_pos, &player_dir);
                if next_pos.is_none() {
                    break;
                }
                player_pos = next_pos.unwrap();
            }
            num = 0;

            if *cmd == 'L' as u8 {
                facing_score -= 1;
                player_dir = Coord {
                    x: player_dir.y,
                    y: -player_dir.x,
                }
            } else {
                facing_score += 1;
                player_dir = player_dir.rotated_right();
            }
        }
    }

    for _ in 0..num {
        let next_pos = board.get_next_pos(&player_pos, &player_dir);
        if next_pos.is_none() {
            break;
        }
        player_pos = next_pos.unwrap();
    }

    facing_score = facing_score.rem_euclid(4);
    println!("{}", (1000 * (player_pos.y + 1)) + (4 * (player_pos.x + 1)) + facing_score);
}

fn part2(mut board: Board, commands: &Vec<u8>) {
    let mut player_dir: i32 = 0;
    let mut player_pos = Coord { x: 0, y: 0 };
    for x in board.bounds_x[0].min..(board.bounds_x[0].max + 1) {
        if !board.field[0][x as usize] {
            player_pos = Coord { x, y: 0 };
            break;
        }
    }

    init_quadrants(&mut board);

    let mut num = 0;
    for cmd in commands {
        if *cmd >= '0' as u8 && *cmd <= '9' as u8 {
            num *= 10;
            num += (*cmd - '0' as u8) as i32;
        } else {
            for _ in 0..num {
                let next = board.get_next_pos_part2(&player_pos, player_dir);
                if next.is_none() {
                    break;
                }

                player_pos = next.clone().unwrap().0;
                player_dir = next.unwrap().1;
            }
            num = 0;

            if *cmd == 'L' as u8 {
                player_dir = (player_dir - 1).rem_euclid(4);
            } else {
                player_dir = (player_dir + 1).rem_euclid(4);
            }
        }
    }

    for _ in 0..num {
        let next = board.get_next_pos_part2(&player_pos, player_dir);
        if next.is_none() {
            break;
        }
        player_pos = next.clone().unwrap().0;
        player_dir = next.unwrap().1;
    }

    println!("{}", (1000 * (player_pos.y + 1)) + (4 * (player_pos.x + 1)) + player_dir);
}
