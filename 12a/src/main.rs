use std::io::{self, BufRead};
use std::cmp::Ordering;

#[derive(Clone, Copy)]
struct Point {
    x: i32,
    y: i32,
    z: i32
}

type Velocity = Point;

#[derive(Clone, Copy)]
struct Moon {
    position: Point,
    velocity: Velocity
}

fn calculate_velocity_single_dimension(current_velocity: i32, position: i32, other_position: i32) -> i32 {
    match position.cmp(&other_position) {
        Ordering::Less => current_velocity + 1,
        Ordering::Equal => current_velocity,
        Ordering::Greater => current_velocity - 1,
    }
}


impl Moon {
    fn new(position: Point) -> Moon {
        Moon {
            position,
            velocity: Velocity { x: 0, y: 0, z: 0 },
        }
    }

    fn adjust_velocity(&mut self, other: &Moon) {
        self.velocity.x = calculate_velocity_single_dimension(self.velocity.x, self.position.x, other.position.x);
        self.velocity.y = calculate_velocity_single_dimension(self.velocity.y, self.position.y, other.position.y);
        self.velocity.z = calculate_velocity_single_dimension(self.velocity.z, self.position.z, other.position.z);
    }

    fn adjust_position(&mut self) {
        self.position.x += self.velocity.x;
        self.position.y += self.velocity.y;
        self.position.z += self.velocity.z;
    }

    fn potential_energy(&self) -> i32 {
        self.position.x.abs() + self.position.y.abs() + self.position.z.abs()
    }

    fn kinetic_energy(&self) -> i32 {
        self.velocity.x.abs() + self.velocity.y.abs() + self.velocity.z.abs()
    }

    fn energy(&self) -> i32 {
        self.potential_energy() * self.kinetic_energy()
    }
}

fn debug_positions(i: i32, moons: &Vec<Moon>) {
    if std::env::var("AOC_TRACE").is_err() {
        return;
    }

    eprintln!("After {} steps:", i);

    for moon in moons {
        eprintln!(
            "pos=<x={:2}, y={:2}, z={:2}>, vel=<x={:2}, y={:2}, z={:2}>",
            moon.position.x,
            moon.position.y,
            moon.position.z,
            moon.velocity.x,
            moon.velocity.y,
            moon.velocity.z,
        )
    }

    eprintln!();
}

fn parse_line(line: String) -> Point {
    let dims: Vec<i32> = line.split(',')
        .map(|num| num.to_string())
        .map(|num| num.parse().unwrap())
        .collect();

    if dims.len() != 3 {
        panic!("Could not parse line: {}", line);
    }

    Point {
        x: dims[0],
        y: dims[1],
        z: dims[2],
    }
}

fn main() {
    let mut moons: Vec<Moon> = io::stdin().lock().lines()
        .map(|line| line.unwrap())
        .map(|line| line.chars().filter(|c| c.is_digit(10) || *c == '-' || *c == ',').collect::<String>())
        .map(parse_line)
        .map(Moon::new)
        .collect();

    debug_positions(0, &moons);
    for i in 0..1000 {
        for ai in 0..moons.len() {
            for bi in 0..moons.len() {
                if ai == bi {
                    continue;
                }

                let b = moons[bi].to_owned();
                let a = &mut moons[ai];

                a.adjust_velocity(&b);
            }
        }

        for moon in &mut moons {
            moon.adjust_position();
        }

        debug_positions(i + 1, &moons);
    }

    let total_energy: i32 = moons.iter()
        .map(|moon| {
            moon.energy()
        })
        .sum();

    println!("{}", total_energy);
}
