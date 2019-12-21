mod maze;

use std::collections::{HashSet, VecDeque};

use maze::{Maze, Position};

type Distance = usize;
type Level = usize;

fn bfs(maze: &Maze, source: &Position, target: &Position) -> Option<Distance> {
    let mut seen: HashSet<(Level, Position)> = HashSet::new();
    let mut queue: VecDeque<(Level, Position, Distance)> = VecDeque::new();

    seen.insert((0, *source));
    queue.push_back((0, *source, 0));

    while !queue.is_empty() {
        let (level, position, distance) = queue.pop_front().unwrap();

        if (level, position) == (0, *target) {
            return Some(distance);
        }

        for (next_position, level_delta) in maze.get_neighbors(&position).iter() {
            if level == 0 && *level_delta < 0 {
                continue
            }

            let next_level = (level as i32 + *level_delta) as usize;

            if seen.contains(&(next_level, *next_position)) {
                continue;
            }

            seen.insert((next_level, *next_position));
            queue.push_back((next_level, *next_position, distance + 1));
        }
    }

    None
}

fn main() {
    let maze = Maze::read();
    let source = maze.labels.iter().find(|(n, _)| n == "AA").map(|(_, p)| p).unwrap();
    let target = maze.labels.iter().find(|(n, _)| n == "ZZ").map(|(_, p)| p).unwrap();
    let distance = bfs(&maze, &source, &target).unwrap();
    println!("{:?}", distance);
}
