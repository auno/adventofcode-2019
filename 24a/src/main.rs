use std::{io, fmt};
use std::io::Read;
use std::collections::HashSet;

use crate::Tile::*;
use std::fmt::Display;

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

fn get_current(game: &Vec<Tile>, x: isize, y: isize) -> Tile {
    if x < 0 || x >= 5 || y < 0 || y >= 5 {
        return Nothing;
    }

    game[(y * 5 + x) as usize]
}

fn calculate_biodiversity(game: &Vec<Tile>) -> u32 {
    game
        .iter()
        .enumerate()
        .filter(|(_, t)| **t == Bug)
        .map(|(i, _)| 1 << i)
        .sum()
}

fn get_next(game: &Vec<Tile>, x: isize, y: isize) -> Tile {
    let neighbors: i32 = [
        get_current(&game, x - 1, y),
        get_current(&game, x + 1, y),
        get_current(&game, x, y - 1),
        get_current(&game, x, y + 1),
    ]
        .iter()
        .map(|t| match t {
            Bug => 1,
            Nothing => 0,
        })
        .sum();

    match (get_current(&game, x, y), neighbors) {
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

    let mut game: Vec<Tile> = input
        .iter()
        .map(|u| *u as char)
        .filter(|c| *c != '\n')
        .map(Tile::from)
        .collect();

    let mut seen: HashSet<u32> = HashSet::new();

    loop {
        let bd = calculate_biodiversity(&game);

        if seen.contains(&bd) {
            println!("{}", bd);
            return;
        }

        seen.insert(bd);
        let mut next_game: Vec<Tile> = game.clone();

        for y in 0..5 {
            for x in 0..5 {
                next_game[(y * 5 + x) as usize] = get_next(&game, x, y);
            }
        }

        game = next_game;
    }
}
