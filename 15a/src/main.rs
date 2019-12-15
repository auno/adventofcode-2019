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

fn main() {
    let memory = int_code::read_memory().unwrap();
    let computer = Computer::new(&memory, None);

    let mut seen: HashSet<Position> = HashSet::new();
    let mut queue: VecDeque<(Position, Distance, Computer)> = VecDeque::new();

    seen.insert((0, 0).into());
    queue.push_back(((0, 0).into(), 0, computer.clone()));

    while !queue.is_empty() {
        let (pos, dist, comp) = queue.pop_front().unwrap();

        for direction in vec![North, South, West, East] {
            let new_pos = pos.mv(direction);

            if seen.contains(&new_pos) {
                continue;
            }

            let mut comp_clone = comp.clone();
            comp_clone.push_input(direction.into());

            if let int_code::State::Halted = comp_clone.run() {
                panic!("Computer halted");
            }

            let status: Status = comp_clone.pop_output().try_into().unwrap();
            seen.insert(new_pos);

            match status {
                HitWall => (),
                Moved => { queue.push_back((new_pos, dist + 1, comp_clone)); }
                FoundOxygen => {
                    println!("{}", dist + 1);
                    return;
                },
            }
        }
    }
}
