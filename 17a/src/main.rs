use crate::int_code::Computer;
use std::collections::HashMap;
use std::cmp::max;

mod int_code;

#[derive(PartialOrd, PartialEq, Eq, Hash, Copy, Clone, Debug)]
struct Position {
    x: i32,
    y: i32
}

impl From<(i32, i32)> for Position {
    fn from((x, y): (i32, i32)) -> Self {
        Position { x, y }
    }
}

fn is_intersection(map: &HashMap<Position, char>, position: &Position) -> bool {
    *map.get(&position).unwrap_or(&' ') == '#' &&
        *map.get(&(position.x, position.y - 1).into()).unwrap_or(&' ') == '#' &&
        *map.get(&(position.x, position.y + 1).into()).unwrap_or(&' ') == '#' &&
        *map.get(&(position.x - 1, position.y).into()).unwrap_or(&' ') == '#' &&
        *map.get(&(position.x + 1, position.y).into()).unwrap_or(&' ') == '#'
}

fn main() {
    let memory = int_code::read_memory().unwrap();
    let mut computer = Computer::new(&memory, None);
    computer.run();

    let mut position: Position = (0, 0).into();
    let mut map: HashMap<Position, char> = HashMap::new();
    let mut max_pos = position;

    for c in computer.get_output() {
        let c: char = (*c as u8) as char;

        max_pos = (max(position.x, max_pos.x), max(position.y, max_pos.y)).into();

        if c == '\n' {
            position = (0, position.y + 1).into();
        } else {
            map.insert(position, c);
            position = (position.x + 1, position.y).into();
        }
    }

    let mut sum = 0;

    for y in 0..max_pos.y {
        for x in 0..max_pos.x {
            let position = (x, y).into();
            let c = map.get(&position).unwrap();

            if (is_intersection(&map, &position)) {
                eprint!("O");
                sum += position.x * position.y;
            } else {
                eprint!("{}", c);
            }
        }

        eprintln!();
    }

    eprintln!("{:?}", sum);
}
