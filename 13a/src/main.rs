use std::collections::HashMap;

use crate::int_code::Computer;
use crate::Tile::{Empty, Wall, Block, HorizontalPaddle, Ball};
use crate::int_code::State::Halted;

mod int_code;

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

fn main() {
    let memory = int_code::read_memory().unwrap();
    let mut computer = Computer::new(&memory, None);

    match computer.run() {
        Halted => (),
        state => panic!("Invalid computer state: {:?}", state)
    }

    let mut canvas: HashMap<(i32, i32), Tile> = HashMap::new();

    while !computer.get_output().is_empty() {
        let x = computer.pop_output() as i32;
        let y = computer.pop_output() as i32;
        let tile = Tile::from(computer.pop_output() as i32);

        canvas.insert((x, y), tile);
    }

    let num_blocks = canvas.values()
        .filter(|tile| **tile == Block)
        .count();
    println!("{}", num_blocks);
}
