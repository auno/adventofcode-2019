use crate::int_code::{Computer, Unit};
use crate::int_code::State::Halted;
use std::io;
use std::fs;

mod int_code;

fn main() {
    let memory = fs::read_to_string("input.txt").expect("Failed to read file: input.txt");
    let memory: Vec<Unit> = memory
        .split(",")
        .map(str::trim)
        .map(|s| s.parse().unwrap())
        .collect();
//    let memory = int_code::read_memory().unwrap();
    let mut computer = Computer::new(&memory, None);

    while *computer.get_state() != Halted {
        computer.run();

        while let Some(c) = computer.pop_output() {
            print!("{}", (c as u8) as char);
        }

        let mut line: String = String::new();
        io::stdin().read_line(&mut line).expect("Failed to read input");

        for c in line.chars() {
            computer.push_input(c as Unit);
        }

//        computer.push_input(10);
    }
}
