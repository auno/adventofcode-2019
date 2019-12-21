use crate::maze::Direction::*;
use crate::maze::Tile::*;
use std::collections::HashMap;
use std::io;
use std::io::BufRead;
use std::cmp::max;

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
    East,
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
pub struct Position {
    x: i32,
    y: i32,
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

pub type Label = (String, Position);

pub struct Maze {
    tiles: HashMap<Position, Tile>,
    pub labels: Vec<(String, Position)>,
    pub portals: HashMap<Position, Position>,
    height: usize,
    width: usize,
}

impl Maze {
    pub fn read() -> Maze {
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
                }
                _ => {
                    tiles.insert(position, c.into());
                    position = Position::at(position.x + 1, position.y);
                }
            }
        }

        let labels = find_labels(&tiles);
        let portals = find_portals(&labels);

        let max = tiles.keys()
            .fold(Position::at(0, 0), |acc, value| Position::at(max(acc.x, value.x), max(acc.y, value.y)));
        let (height, width) = (max.y as usize + 1, max.x as usize + 1);


        Maze {
            tiles,
            labels,
            portals,
            height,
            width
        }
    }

    #[allow(unused)]
    pub fn eprint(&self) {
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

        eprintln!("Labels:");
        self.labels.iter()
            .for_each(|label| eprintln!("  {:?}", label));

        eprintln!("Portals:");
        self.portals.iter()
            .for_each(|portal| eprintln!("  {:?}", portal));

        eprintln!("Height: {}", self.height);
        eprintln!("Width: {}", self.width);
    }

    pub fn get_neighbors(&self, position: &Position) -> Vec<(Position, i32)> {
        let mut neighbors: Vec<(Position, i32)> = [North, South, West, East].iter()
            .filter_map(|direction| {
                let candidate_position = position.mv1(*direction);
                match self.tiles.get(&candidate_position) {
                    Some(Corridor) => Some((candidate_position, 0)),
                    _ => None,
                }
            })
            .collect();

        if let Some(portal_destination) = self.portals.get(position) {
            let delta = match self.is_outer_position(position) {
                true => -1,
                false => 1
            };
            neighbors.push((*portal_destination, delta));
        }

        neighbors
    }

    pub fn is_outer_position(&self, position: &Position) -> bool {
        position.x == 2 || position.y == 2 || position.x == (self.width - 3) as i32 || position.y == (self.height - 3) as i32
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