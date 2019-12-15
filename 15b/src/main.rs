mod int_code;

use std::convert::{TryFrom, TryInto};
use std::collections::{VecDeque, HashSet};

use crate::int_code::Computer;
use crate::Direction::*;
use crate::Status::*;
use crate::Error::ParseStatusError;

#[derive(Debug)]
enum Error {
    ParseStatusError(String)
}

#[derive(Copy, Clone)]
enum Direction {
    North,
    South,
    West,
    East
}

impl Into<int_code::Unit> for Direction {
    fn into(self) -> int_code::Unit {
        match self {
            North => 1,
            South => 2,
            West => 3,
            East => 4,
        }
    }
}

#[derive(Copy, Clone, PartialEq)]
enum Status {
    HitWall,
    Moved,
    FoundOxygen,
}

impl TryFrom<int_code::Unit> for Status {
    type Error = Error;

    fn try_from(value: int_code::Unit) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(HitWall),
            1 => Ok(Moved),
            2 => Ok(FoundOxygen),
            _ => Err(ParseStatusError(format!("Unknown status: {}", value)))
        }
    }
}

#[derive(PartialOrd, PartialEq, Eq, Hash, Copy, Clone)]
struct Position {
    x: i32,
    y: i32
}

impl Position {
    fn mv(&self, direction: Direction) -> Position {
        match direction {
            North => (self.x, self.y - 1),
            South => (self.x, self.y + 1),
            West => (self.x - 1, self.y),
            East => (self.x + 1, self.y),
        }.into()
    }
}

impl From<(i32, i32)> for Position {
    fn from((x, y): (i32, i32)) -> Self {
        Position { x, y }
    }
}

type Distance = i32;

fn bfs(initial_computer: Computer) -> Vec<(Distance, Status, Computer)> {
    let mut seen: HashSet<Position> = HashSet::new();
    let mut queue: VecDeque<(Position, Distance, Computer)> = VecDeque::new();
    let mut result: Vec<(Distance, Status, Computer)> = Vec::new();

    seen.insert((0, 0).into());
    queue.push_back(((0, 0).into(), 0, initial_computer.clone()));

    while !queue.is_empty() {
        let (position, distance, computer) = queue.pop_front().unwrap();

        for direction in vec![North, South, West, East] {
            let new_pos = position.mv(direction);

            if seen.contains(&new_pos) {
                continue;
            }

            let mut new_computer = computer.clone();
            new_computer.push_input(direction.into());

            if let int_code::State::Halted = new_computer.run() {
                panic!("Computer halted");
            }

            let status: Status = new_computer.pop_output().try_into().unwrap();
            seen.insert(new_pos);
            let new_distance = distance + 1;
            result.push((new_distance, status, new_computer.clone()));

            match status {
                HitWall => (),
                _ => { queue.push_back((new_pos, new_distance, new_computer)); }
            }
         }
    }

    result
}

fn main() {
    let memory = int_code::read_memory().unwrap();
    let computer = Computer::new(&memory, None);

    let result_from_start = bfs(computer);
    let (distance_to_oxygen, _, computer) = result_from_start.iter()
        .find(|(_, status, _)| *status == FoundOxygen)
        .unwrap();

    println!("{}", distance_to_oxygen);

    let result_from_oxygen = bfs(computer.to_owned());
    let max_distance_from_oxygen = result_from_oxygen.iter()
        .map(|(distance, _, _)| distance)
        .max()
        .unwrap();

    println!("{}", max_distance_from_oxygen);
}
