use std::collections::HashMap;
use std::cmp::{max, Ordering};
use std::{thread, time};

mod int_code;

use crate::int_code::{Computer, Unit};
use crate::Tile::{Empty, Wall, Block, HorizontalPaddle, Ball};
use crate::int_code::State;

type Canvas = HashMap<(i32, i32), Tile>;

#[derive(PartialEq)]
enum Tile {
    Empty,
    Wall,
    Block,
    HorizontalPaddle,
    Ball
}

impl From<i32> for Tile {
    fn from(id: i32) -> Self {
        match id {
            0 => Empty,
            1 => Wall,
            2 => Block,
            3 => HorizontalPaddle,
            4 => Ball,
            _ => panic!("Unknown tile id: {}", id)
        }
    }
}

impl Tile {
    fn to_char(&self) -> char {
        match self {
            Empty => ' ',
            Wall => '▓',
            Block => '░',
            HorizontalPaddle => '▔',
            Ball => '•',
        }
    }
}

fn print_display(canvas: &Canvas, score: i32) {
    if std::env::var("AOC_TRACE").is_err() {
        return;
    }

    let (x_max, y_max) = canvas.keys()
        .fold((0, 0), |acc, value| {
            (max(acc.0, value.0), max(acc.1, value.1))
        });

    println!(" Score: {:3}", score);

    for y in 0..(y_max + 2) {
        for x in 0..(x_max + 2) {
            print!("{}", canvas.get(&(x, y)).unwrap_or(&Empty).to_char());
        }

        println!()
    }
}

fn find_tile_x_position(canvas: &Canvas, tile: Tile) -> &i32 {
    let ((x, _), _) = canvas.iter()
        .find(|((_, _), candidate)| **candidate == tile)
        .unwrap();

    x
}

fn move_paddle(canvas: &Canvas) -> Unit {
    let ball = find_tile_x_position(canvas, Ball);
    let paddle = find_tile_x_position(canvas, HorizontalPaddle);

    match paddle.cmp(ball) {
        Ordering::Less => 1,
        Ordering::Equal => 0,
        Ordering::Greater => -1,
    }
}

fn main() {
    let mut memory = int_code::read_memory().unwrap();
    memory[0] = 2;
    let mut computer = Computer::new(&memory, None);

    let mut canvas = Canvas::new();
    let mut score = 0;

    loop {
        computer.run();

        while !computer.get_output().is_empty() {
            let x = computer.pop_output() as i32;
            let y = computer.pop_output() as i32;
            let v = computer.pop_output() as i32;

            let tile = match (x, y) {
                (-1, 0) => { score = v; continue; },
                _ => { Tile::from(v) }
            };

            canvas.insert((x, y), tile);
        }

        print_display(&canvas, score);

        match computer.get_state() {
            State::Blocked => computer.push_input(move_paddle(&canvas)),
            State::Halted => break,
            state => panic!("Invalid computer state: {:?}", state),
        }

        if std::env::var("AOC_TRACE").is_ok() {
            thread::sleep(time::Duration::from_millis(100));
        }
    }

    eprintln!("{}", score);


}
