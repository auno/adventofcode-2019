mod int_code;

use std::io::{self, BufRead};

use int_code::Computer;
use crate::RowEvaluation::*;
use crate::int_code::Unit;
use std::collections::{HashMap, HashSet};

enum RowEvaluation {
    TooLow,
    TooHigh,
    JustRight(usize)
}

fn find_start_of_row<F>(y: usize, is_pulled: F) -> usize
        where F: Fn(usize, usize) -> bool, F: Copy {
    for x in 0..(y + 10) {
        if is_pulled(x, y) {
            return x;
        }
    }

    unreachable!("Should have found y");
}

fn check_row<F>(y: usize, is_pulled: F) -> RowEvaluation
        where F: Fn (usize, usize) -> bool, F: Copy {
    match check_row1(y, is_pulled) {
        JustRight(x) => match check_row1(y - 1, is_pulled) {
                TooLow => JustRight(x),
                _ => TooHigh,
            },
        x => x
    }
}

fn check_row1<F>(y: usize, is_pulled: F) -> RowEvaluation where
        F: Fn(usize, usize) -> bool, F: Copy {
    let size = 100;
    let x = find_start_of_row(y + size - 1, is_pulled);
    if is_pulled(x + size - 1, y) {
        if is_pulled(x + size, y) {
            return TooHigh;
        } else {
            return JustRight(x)
        }
    }

    TooLow
}

fn read_input() -> HashSet<(usize, usize)> {
    let mut map: HashSet <(usize, usize)> = HashSet::new();

    let mut position= (0, 0);

    let input: String = io::stdin().lock().lines()
        .map(Result::unwrap)
        .collect::<Vec<String>>()
        .join("\n");

    for c in input.chars() {
        match c {
            '#' => {
                map.insert(position);
                position = (position.0 + 1, position.1).into();
            }
            '\n' => {
                position = (0, position.1 + 1).into();
            },
            _ => {
                position = (position.0 + 1, position.1).into();
            }
        }
    }

    map
}

fn main() {
    let memory = int_code::read_memory().unwrap();

    let is_pulled = |x, y| {
        let mut computer = Computer::new(&memory, Some(&vec![x as Unit, y as Unit]));
        computer.run();

        return computer.pop_output() == 1;
    };

//    let map = read_input();
//    let is_pulled = |x, y| {
//        map.contains(&(x, y))
//    };

    let mut start = 500;
    let mut end = 3000;

    let mut answer = None;

    while start <= end {
        let mid = (start + end) / 2;

        match check_row(mid, is_pulled) {
            TooLow => {
                start = mid + 1;
            },
            TooHigh => {
                end = mid - 1;
            },
            JustRight(x) => {
                answer = Some((x, mid));
                break;
            },
        }
    }

    if let Some(answer) = answer {
        println!("{}", answer.0 * 10000 + answer.1);
    } else {
        println!("No solution");
    }
}
