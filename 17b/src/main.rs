use crate::int_code::{Computer, Unit};
use std::collections::HashMap;
use std::cmp::max;

mod int_code;

use Direction::*;
use Turn::*;
use std::convert::TryFrom;
use crate::Error::ParseDirectionError;

#[derive(Debug)]
enum Error {
    ParseDirectionError(String)
}

#[derive(PartialOrd, PartialEq, Eq, Hash, Copy, Clone, Debug)]
struct Position {
    x: i32,
    y: i32
}

impl Position {
    fn mv1(&self, direction: Direction) -> Position {
        self.mv(direction, 1)
    }

    fn mv(&self, direction: Direction, distance: usize) -> Position {
        let distance = distance as i32;
        match direction {
            North => (self.x, self.y - distance),
            South => (self.x, self.y + distance),
            West => (self.x - distance, self.y),
            East => (self.x + distance, self.y),
        }.into()
    }
}

impl From<(i32, i32)> for Position {
    fn from((x, y): (i32, i32)) -> Self {
        Position { x, y }
    }
}

#[derive(Copy, Clone, Debug, PartialOrd, PartialEq)]
enum Turn {
    Left,
    Right,
}

impl Into<char> for Turn {
    fn into(self) -> char {
        match self {
            Left => 'L',
            Right => 'R',
        }
    }
}

#[derive(Copy, Clone, Debug, PartialOrd, PartialEq)]
enum Direction {
    North,
    South,
    West,
    East
}

impl Direction {
    fn turn(&self, turn: &Turn) -> Direction {
        match (self, turn) {
            (North, Left) => West,
            (West, Left) => South,
            (South, Left) => East,
            (East, Left) => North,
            (North, Right) => East,
            (West, Right) => North,
            (South, Right) => West,
            (East, Right) => South,
        }
    }
}

#[derive(Copy, Clone, Debug, PartialOrd, PartialEq)]
struct PathElement {
    turn: Turn,
    distance: usize
}

impl From<(Turn, usize)> for PathElement {
    fn from((turn, distance): (Turn, usize)) -> PathElement {
        PathElement {
            turn,
            distance
        }
    }
}

impl ToString for PathElement {
    fn to_string(&self) -> String {
        let tc: char = self.turn.into();
        format!("{},{}", tc, self.distance)
    }
}

type Path = Vec<PathElement>;

impl TryFrom<char> for Direction {
    type Error = Error;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '^' => Ok(North),
            'v' => Ok(South),
            '<' => Ok(West),
            '>' => Ok(East),
            _ => Err(ParseDirectionError(format!("Unknown direction: {}", value)))
        }
    }
}

fn build_solution_input(main_routine: Vec<char>, functions: HashMap<char, Vec<PathElement>>) -> String {
    let mut solution_input = String::new();

    solution_input.push_str(main_routine.iter().map(char::to_string).collect::<Vec<String>>().join(",").as_str());
    solution_input.push('\n');

    for name in &['A', 'B', 'C'] {
        solution_input.push_str(functions.get(name).unwrap().iter().map(PathElement::to_string).collect::<Vec<String>>().join(",").as_str());
        solution_input.push('\n');
    }

    solution_input.push_str("n\n");

    solution_input
}

fn find_solution(path: &Vec<PathElement>) -> Option<(Vec<char>, HashMap<char, Path>)> {
    let a = 0usize;
    for alen in (1..6usize).rev() {
        for b in (a + alen)..(path.len() - 1) {
            for blen in (1..6usize).rev() {
                if b + blen + 1 > path.len() {
                    continue;
                }

                for c in (b + blen)..path.len() + 1 {
                    for clen in (1..6usize).rev() {
                        if c + clen > path.len() {
                            continue;
                        }

                        if let Some((mr, functions)) = build_main_routine(&path, a, alen, b, blen, c, clen) {
                            if mr.len() <= 10 {
                                return Some((mr, functions));
                            }
                        }
                    }
                }
            }
        }
    }

    None
}

fn read_map(computer: &mut Computer) -> HashMap<Position, char> {
    let mut position: Position = (0, 0).into();
    let mut map: HashMap<Position, char> = HashMap::new();
    let mut max_pos = position;

    let mut last_c = '.';
    for c in computer.get_output() {
        let c: char = (*c as u8) as char;

        max_pos = (max(position.x, max_pos.x), max(position.y, max_pos.y)).into();

        match (c, last_c) {
            ('\n', '\n') => break,
            ('\n', _) => {
                position = (0, position.y + 1).into();
            },
            _ => {
                map.insert(position, c);
                position = (position.x + 1, position.y).into();
            }
        }

        last_c = c;
    }

    map
}

fn build_path(map: &HashMap<Position, char>) -> Vec<PathElement> {
    let (mut position, mut direction): (Position, Direction) = map.iter()
        .find(|(_, d)| Direction::try_from(**d).is_ok())
        .map(|(p, d)| (*p, Direction::try_from(*d).unwrap()))
        .unwrap();
    let mut path: Vec<PathElement> = Vec::new();

    loop {
        let next_turn = vec![Left, Right].iter()
            .map(|t| *t)
            .find(|candidate| *map.get(&position.mv1(direction.turn(candidate))).unwrap_or(&'.') == '#');

        if next_turn.is_none() {
            break;
        }

        let next_turn = next_turn.unwrap();
        let next_direction = direction.turn(&next_turn);

        let next_distance = (0usize..)
            .find(|candidate| *map.get(&position.mv(next_direction, candidate + 1)).unwrap_or(&'.') == '.')
            .unwrap();

        path.push((next_turn, next_distance).into());
        position = position.mv(next_direction, next_distance);
        direction = next_direction;
    }

    path
}

fn build_main_routine(path: &Vec<PathElement>, a: usize, alen: usize, b: usize, blen: usize, c: usize, clen: usize) -> Option<(Vec<char>, HashMap<char, Path>)> {
    let mut functions: HashMap<char, Path> = HashMap::new();
    functions.insert('A', path.iter().skip(a).take(alen).map(|r| *r).collect());
    functions.insert('B', path.iter().skip(b).take(blen).map(|r| *r).collect());
    functions.insert('C', path.iter().skip(c).take(clen).map(|r| *r).collect());
    let functions = functions;

    let mut main_routine: Vec<char> = Vec::new();
    let mut position = 0usize;

    'outer: while position < path.len() {
        for (name, body) in functions.iter() {
            if position + body.len() > path.len() {
                continue;
            }

            if body.as_slice() == &path.as_slice()[position..(position + body.len())] {
                position += body.len();
                main_routine.push(*name);
                continue 'outer;
            }
        }

        return None;
    }

    Some((main_routine, functions))
}

fn main() {
    let mut memory = int_code::read_memory().unwrap();
    memory[0] = 2;
    let memory = memory;
    let mut computer = Computer::new(&memory, None);
    computer.run();

    let map = read_map(&mut computer);
    let path = build_path(&map);
    let solution = find_solution(&path);
    let (main_routine, functions) = solution.unwrap();
    let solution_input = build_solution_input(main_routine, functions);

    solution_input.chars()
        .for_each(|c| computer.push_input(c as Unit));

    computer.run();

    println!("{}", computer.get_output().back().unwrap());
}