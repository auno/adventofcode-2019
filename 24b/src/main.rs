use std::{io, fmt};
use std::io::Read;
use std::collections::HashMap;

use crate::Tile::*;
use std::fmt::Display;

type Game = HashMap<isize, Vec<Tile>>;

#[derive(Copy, Clone, Eq, PartialEq)]
enum Tile {
    Bug,
    Nothing
}

impl From<char> for Tile {
    fn from(c: char) -> Tile {
        match c {
            '#' => Bug,
            '.' => Nothing,
            _ => panic!("Unknown tile: {}", c)
        }
    }
}

impl Display for Tile {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let c = match self {
            Bug => '#',
            Nothing => '.',
        };
        write!(f, "{}", c)
    }
}

fn get_neighbors((level, x, y): (isize, usize, usize)) -> Vec<(isize, usize, usize)> {
    match (level, x, y) {
        (l, 0, 0) =>
            vec![
                (l, 1, 0),
                (l, 0, 1),
                (l - 1, 2, 1),
                (l - 1, 1, 2),
            ],
        (l, 4, 0) =>
            vec![
                (l, 3, 0),
                (l, 4, 1),
                (l - 1, 2, 1),
                (l - 1, 3, 2),
            ],
        (l, 0, 4) =>
            vec![
                (l, 0, 3),
                (l, 1, 4),
                (l - 1, 1, 2),
                (l - 1, 2, 3),
            ],
        (l, 4, 4) =>
            vec![
                (l, 3, 4),
                (l, 4, 3),
                (l - 1, 3, 2),
                (l - 1, 2, 3),
            ],

        (l, 0, y) =>
            vec![
                (l, 0, y - 1),
                (l, 0, y + 1),
                (l, 1, y),
                (l - 1, 1, 2),
            ],
        (l, 4, y) =>
            vec![
                (l, 4, y - 1),
                (l, 4, y + 1),
                (l, 3, y),
                (l - 1, 3, 2),
            ],
        (l, x, 0) =>
            vec![
                (l, x - 1, 0),
                (l, x + 1, 0),
                (l, x, 1),
                (l - 1, 2, 1),
            ],
        (l, x, 4) =>
            vec![
                (l, x - 1, 4),
                (l, x + 1, 4),
                (l, x, 3),
                (l - 1, 2, 3),
            ],

        (l, 2, 1) =>
            vec![
                (l, x, y - 1),
                (l, x - 1, y),
                (l, x + 1, y),
                (l + 1, 0, 0),
                (l + 1, 1, 0),
                (l + 1, 2, 0),
                (l + 1, 3, 0),
                (l + 1, 4, 0),
            ],
        (l, 2, 3) =>
            vec![
                (l, x, y + 1),
                (l, x - 1, y),
                (l, x + 1, y),
                (l + 1, 0, 4),
                (l + 1, 1, 4),
                (l + 1, 2, 4),
                (l + 1, 3, 4),
                (l + 1, 4, 4),
            ],
        (l, 1, 2) =>
            vec![
                (l, x, y - 1),
                (l, x, y + 1),
                (l, x - 1, y),
                (l + 1, 0, 0),
                (l + 1, 0, 1),
                (l + 1, 0, 2),
                (l + 1, 0, 3),
                (l + 1, 0, 4),
            ],
        (l, 3, 2) =>
            vec![
                (l, x, y - 1),
                (l, x, y + 1),
                (l, x + 1, y),
                (l + 1, 4, 0),
                (l + 1, 4, 1),
                (l + 1, 4, 2),
                (l + 1, 4, 3),
                (l + 1, 4, 4),
            ],

        (l, x, y) =>
            vec![
                (l, x, y - 1),
                (l, x, y + 1),
                (l, x + 1, y),
                (l, x - 1, y),
            ]
    }
}

fn get_current(game: &Game, (level, x, y): (isize, usize, usize)) -> Tile {
    if !game.contains_key(&level) {
        return Nothing;
    }

    game[&level][(y * 5 + x) as usize]
}

fn get_next(game: &Game, level: isize, x: usize, y: usize) -> Tile {
    let neighbors: i32 = get_neighbors((level, x, y))
        .iter()
        .map(|p| get_current(&game, *p))
        .map(|t| match t {
            Bug => 1,
            Nothing => 0,
        })
        .sum();

    match (get_current(&game, (level, x, y)), neighbors) {
        (Bug, 1) => Bug,
        (Bug, _) => Nothing,
        (Nothing, 1) | (Nothing, 2) => Bug,
        (t, _) => t,
    }
}

fn main() {
    let mut input = Vec::new();

    if io::stdin().read_to_end(&mut input).is_err() {
        panic!("Failed to read input");
    }

    let input: Vec<Tile> = input
        .iter()
        .map(|u| *u as char)
        .filter(|c| *c != '\n')
        .map(Tile::from)
        .collect();

    let mut game: Game = HashMap::new();
    game.insert(0, input);

    let mut empty: Vec<Tile> = Vec::new();
    empty.resize(25, Nothing);

    for i in 0..200 {
        game.insert(-(i + 1), empty.clone());
        game.insert(i + 1, empty.clone());

        let mut next_game: Game = game.clone();

        for level in -(i + 1)..(i + 2) {
            let level_map = next_game.get_mut(&level).unwrap();

            for y in 0..5 {
                for x in 0..5 {
                    if x == 2 && y == 2 {
                        continue;
                    }

                    level_map[y * 5 + x] = get_next(&game, level, x, y);
                }
            }
        }

        game = next_game;
    }

    let count: u32 = game.values()
        .flat_map(|level| level.iter())
        .map(|t| match t {
            Bug => 1,
            Nothing => 0,
        })
        .sum();

    println!("{}", count);
}