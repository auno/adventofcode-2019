use std::io::{self, BufRead};
use std::collections::{HashMap, HashSet, VecDeque};
use std::cmp::max;

use crate::Direction::*;
use crate::Tile::*;

#[derive(Copy, Clone, Debug, PartialOrd, PartialEq)]
enum Tile {
    Wall,
    Corridor,
    Nothing,
    Label(char),
}

impl From<char> for Tile {
    fn from(c: char) -> Tile {
        match c {
            '#' => Wall,
            '.' => Corridor,
            ' ' => Nothing,
            'A'..='Z' => Label(c),
            _ => panic!("Unknown tile: {}", c)
        }
    }
}

impl Into<char> for Tile {
    fn into(self) -> char {
        match self {
            Wall => '#',
            Corridor => '.',
            Nothing => ' ',
            Label(c) => c,
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
    fn opposite(&self) -> Direction {
        match self {
            North => South,
            South => North,
            West => East,
            East => West,
        }
    }
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

    fn at(x: i32, y: i32) -> Self {
        Position { x, y }
    }
}

impl From<(i32, i32)> for Position {
    fn from((x, y): (i32, i32)) -> Self {
        Position::at(x, y)
    }
}

type Label = (String, Position);

struct Maze {
    tiles: HashMap<Position, Tile>,
    labels: Vec<(String, Position)>,
    portals: HashMap<Position, Position>,
}

impl Maze {
    fn read() -> Maze {
        let mut position = Position::at(0, 0);
        let mut tiles: HashMap<Position, Tile> = HashMap::new();

        let input: String = io::stdin().lock().lines()
            .map(Result::unwrap)
            .collect::<Vec<String>>()
            .join("\n");

        for c in input.chars() {
            match c {
                '\n' => {
                    position = Position::at(0, position.y + 1);
                },
                _ => {
                    tiles.insert(position, c.into());
                    position = Position::at(position.x + 1, position.y);
                }
            }
        }

        let labels = find_labels(&tiles);
        let portals = find_portals(&labels);

        Maze {
            tiles,
            labels,
            portals
        }
    }

    #[allow(unused)]
    fn eprint(&self) {
        let max = self.tiles.keys()
            .fold(Position::at(0, 0), |acc, value| Position::at(max(acc.x, value.x), max(acc.y, value.y)));

        for y in 0..(max.y + 1) {
            for x in 0..(max.x + 1) {
                let tile = *self.tiles.get(&Position::at(x, y)).unwrap_or(&Nothing);
                let c: char = tile.into();
                eprint!("{}", c);
            }
            eprintln!();
        }

        println!("Labels:");
        self.labels.iter()
            .for_each(|label| eprintln!("  {:?}", label));

        println!("Portals:");
        self.portals.iter()
            .for_each(|portal| eprintln!("  {:?}", portal));
    }

    fn get_neighbors(&self, position: &Position) -> Vec<Position> {
        let mut neighbors: Vec<Position> = [North, South, West, East].iter()
            .filter_map(|direction| {
                let candidate_position = position.mv1(*direction);
                match self.tiles.get(&candidate_position) {
                    Some(Corridor) => Some(candidate_position),
                    _ => None,
                }
            })
            .collect();

        if let Some(portal_destination) = self.portals.get(position) {
            neighbors.push(*portal_destination);
        }

        neighbors
    }
}

fn resolve_label_direction(tiles: &HashMap<Position, Tile>, position: &Position, d1: Direction) -> Option<Label> {
    let d1 = match d1 {
        South => North,
        East => West,
        d => d,
    };

    let d2 = d1.opposite();


    let neighbors = (
        tiles.get(&position.mv1(d1)).unwrap_or(&Nothing),
        tiles.get(&position).unwrap_or(&Nothing),
        tiles.get(&position.mv1(d2)).unwrap_or(&Nothing),
    );

    match neighbors {
        (Label(c1), Label(c2), Corridor) => Some((format!("{}{}", c1, c2), position.mv1(d2))),
        (Corridor, Label(c1), Label(c2)) => Some((format!("{}{}", c1, c2), position.mv1(d1))),
        _ => None
    }
}

fn resolve_label(tiles: &HashMap<Position, Tile>, position: &Position) -> Option<Label> {
    if let Some(label) = resolve_label_direction(tiles, position, North) {
        return Some(label);
    }

    if let Some(label) = resolve_label_direction(tiles, position, West) {
        return Some(label);
    }

    None
}

fn find_labels(tiles: &HashMap<Position, Tile>) -> Vec<(String, Position)> {
    tiles.iter()
        .filter_map(|(p, _)| {
            resolve_label(tiles, p)
        })
        .collect()
}

fn find_portals(labels: &Vec<Label>) -> HashMap<Position, Position> {
    labels.iter()
        .filter(|(name, _)| name != "AA" && name != "ZZ")
        .map(|(name, position)| {
            let other = labels.iter()
                .find(|(name2, position2)| name2 == name && position2 != position)
                .map(|(_, p)| p)
                .expect("Labels must always come in pairs");

            (*position, *other)
        })
        .collect()
}

type Distance = usize;

fn bfs(maze: &Maze, source: &Position, target: &Position) -> Option<Distance> {
    let mut seen: HashSet<Position> = HashSet::new();
    let mut queue: VecDeque<(Position, Distance)> = VecDeque::new();

    seen.insert(*source);
    queue.push_back((*source, 0));

    while !queue.is_empty() {
        let (position, distance) = queue.pop_front().unwrap();

        if position == *target {
            return Some(distance);
        }

        for neighbor in maze.get_neighbors(&position).iter() {
            if seen.contains(&neighbor) {
                continue;
            }

            seen.insert(*neighbor);
            queue.push_back((*neighbor, distance + 1));
        }
    }

    None
}

fn main() {
    let maze = Maze::read();
    let source = maze.labels.iter().find(|(n, _)| n == "AA").map(|(_, p)| p).unwrap();
    let target = maze.labels.iter().find(|(n, _)| n == "ZZ").map(|(_, p)| p).unwrap();
    let distance = bfs(&maze, &source, &target).unwrap();

    println!("{}", distance);
}
