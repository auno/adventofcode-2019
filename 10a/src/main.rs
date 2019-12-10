use std::io;
use std::io::BufRead;
use std::collections::HashSet;

type Point = (i32, i32);

fn sub_point(a: &Point, b: &Point) -> Point {
    (a.0 - b.0, a.1 - b.1)
}

fn gcd(a: i32, b: i32) -> i32 {
    let mut a = a.abs();
    let mut b = b.abs();

    while b != 0 {
        let t = b;
        b = a % b;
        a = t;
    }

    a
}

fn direction(p: Point) -> Point {
    let d = gcd(p.0, p.1);
    (p.0 / d, p.1 / d)
}

fn main() {
    let lines: Vec<String> = io::stdin().lock().lines()
        .map(|s| s.unwrap())
        .collect();

    let asteroids: Vec<Point> = lines.iter()
        .enumerate()
        .flat_map(|(x, line)| {
            line.chars()
                .enumerate()
                .filter(|(_, c)| *c == '#')
                .map(move |(y,_)| {
                    (y as i32, x as i32)
                })
        })
        .collect();

    let (asteroid, num_visible) = asteroids.iter()
        .map(|candidate| {
            let visible: HashSet<Point> = asteroids.iter()
                .filter(|a| a != &candidate)
                .map(|other| sub_point(other, candidate))
                .map(direction)
                .collect();

            (candidate, visible.len())
        })
        .max_by_key(|(_, num_visible)| *num_visible)
        .unwrap();

    println!("{:?}, {}", asteroid, num_visible);
}
