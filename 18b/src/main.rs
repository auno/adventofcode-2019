use std::io::{self, BufRead};
use std::collections::{VecDeque, HashSet, HashMap};

use Direction::*;
use Tile::*;

#[derive(Copy, Clone, Debug, PartialOrd, PartialEq)]
enum Tile {
    Wall,
    Corridor,
    Lock(char),
    Key(char),
}

impl From<char> for Tile {
    fn from(c: char) -> Tile {
        match c {
            '#' => Wall,
            '.' | '@' => Corridor,
            'A'..='Z' => Lock(c.to_ascii_lowercase()),
            'a'..='z' => Key(c),
            _ => panic!("Unknown tile: {}", c)
        }
    }
}

impl Into<char> for Tile {
    fn into(self) -> char {
        match self {
            Wall => '#',
            Corridor => '.',
            Lock(c) => c,
            Key(c) => c,
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

type Distance = usize;
type Positions = [Position; 4];

fn read_map() -> (HashMap<Position, Tile>, Positions) {
    let mut position: Position = (0, 0).into();
    let mut starting_positions: Vec<Position> = Vec::new();
    let mut map: HashMap<Position, Tile> = HashMap::new();

    let input: String = io::stdin().lock().lines()
        .map(Result::unwrap)
        .collect::<Vec<String>>()
        .join("\n");

    for c in input.chars() {
        if c == '@' {
            starting_positions.push(position.clone());
        }

        match c {
            '\n' => {
                position = (0, position.y + 1).into();
            },
            _ => {
                map.insert(position, c.into());
                position = (position.x + 1, position.y).into();
            }
        }
    }

    if starting_positions.len() != 4 {
        panic!("Not exactly four starting positions: {:?}", starting_positions);
    }

    let starting_positions = [
        starting_positions[0],
        starting_positions[1],
        starting_positions[2],
        starting_positions[3],
    ];

    (map, starting_positions)
}

fn bfs(map: &HashMap<Position, Tile>, source: Position, keys: &Vec<char>) -> Vec<(Position, Distance, char)> {
    let mut seen: HashSet<Position> = HashSet::new();
    let mut queue: VecDeque<(Position, Distance)> = VecDeque::new();
    let mut results: Vec<(Position, Distance, char)> = Vec::new();
    let keys: HashSet<char> = keys.iter().map(|c| *c).collect();

    seen.insert(source);
    queue.push_back((source, 0));

    while !queue.is_empty() {
        let (position, distance) = queue.pop_front().unwrap();

        for direction in &[North, South, West, East] {
            let new_pos = position.mv1(*direction);

            if seen.contains(&new_pos) {
                continue;
            }

            seen.insert(new_pos);

            match map.get(&new_pos) {
                Some(Wall) | None => (),
                Some(Corridor) => {
                    queue.push_back((new_pos, distance + 1));
                },
                Some(Key(c)) => {
                    if !keys.contains(c) {
                        results.push((new_pos, distance + 1, *c));
                    }
                    queue.push_back((new_pos, distance + 1));
                },
                Some(Lock(c)) => {
                    if keys.contains(c) {
                        queue.push_back((new_pos, distance + 1));
                    }
                },
            }
        }
    }

    results
}

type BfsCache = HashMap<(Position, Vec<char>), Vec<(Position, Distance, char)>>;

fn bfsc(cache: &mut BfsCache, map: &HashMap<Position, Tile>, source: Position, keys: &Vec<char>) -> Vec<(Position, Distance, char)> {
    if let Some(result) = cache.get(&(source, keys.clone())) {
        return result.clone();
    }

    let result = bfs(map, source, keys);

    cache.insert((source, keys.clone()), result.clone());

    result
}

fn bfsa(cache: &mut BfsCache, map: &HashMap<Position, Tile>, sources: Positions, keys: &Vec<char>) -> [Vec<(Position, Distance, char)>; 4] {
    let mut results: [Vec<(Position, Distance, char)>; 4] = [Vec::new(), Vec::new(), Vec::new(), Vec::new()];

    sources.iter().enumerate()
        .for_each(|(i, source)| results[i] = bfsc(cache, map, *source, keys));

    results
}

fn main() {
    let (map, starting_positions) = read_map();

    let mut cache: BfsCache = HashMap::new();
    let mut queue: VecDeque<(Positions, Distance, Vec<char>)> = VecDeque::new();
    let mut distances: HashMap<(Positions, Vec<char>), Distance> = HashMap::new();
    let mut all_keys: Vec<char> = map.values()
        .filter_map(|t| match t {
            Key(c) => Some(*c),
            _ => None
        })
        .collect();
    all_keys.sort();
    let all_keys = all_keys;

    queue.push_back((starting_positions, 0, Vec::new()));

    while !queue.is_empty() {
        let (positions, distance, keys) = queue.pop_front().unwrap();

        if distances.get(&(positions, keys.to_owned())).unwrap_or(&std::usize::MAX) <= &distance {
            continue;
        }

        distances.insert((positions, keys.to_owned()), distance);

        if keys.len() >= all_keys.len() {
            continue;
        }

        let reachable_keyss = bfsa(&mut cache, &map, positions, &keys);

        for (i, reachable_keys) in reachable_keyss.iter().enumerate() {
            for (next_position, next_distance, next_key) in reachable_keys {
                let mut next_positions = positions;
                next_positions[i] = *next_position;
                let next_positions = next_positions;

                let mut next_keys = keys.clone();
                next_keys.push(*next_key);
                next_keys.sort();
                let next_keys = next_keys;

                queue.push_back((next_positions, distance + *next_distance, next_keys));
            }
        }
    }

    let min_distance = distances.iter()
        .filter(|((_, keys), _)| keys == &all_keys)
        .map(|(_, d)| d)
        .min()
        .unwrap();

    println!("{}", min_distance);
}
