use std::io::{self, BufRead};
use std::cmp::Ordering;

#[derive(Clone, Copy, Debug)]
struct Point {
    x: i32,
    y: i32,
    z: i32
}

type Velocity = Point;

#[derive(Clone, Copy, Debug)]
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

fn equal_single_dimension(a: &Vec<Moon>, b: &Vec<Moon>, accessor: fn (Point) -> i32) -> bool {
    if a.len() != b.len() {
        return false;
    }

    for i in 0..a.len() {
        if accessor(a[i].position) != accessor(b[i].position) || accessor(a[i].velocity) != accessor(b[i].velocity) {
            return false;
        }
    }

    true
}

fn find_period_single_dimension(moons: &Vec<Moon>, accessor: fn (Point) -> i32) -> i32 {
    let mut moons = moons.to_owned();
    let start_state = moons.to_owned();
    let mut period = 0;

    debug_positions(0, &moons);

    loop {
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

        debug_positions(period + 1, &moons);
        period += 1;

        if equal_single_dimension(&moons, &start_state, accessor) {
            break;
        }
    }

    period
}

fn gcd(a: i64, b: i64) -> i64 {
    let mut a = a.abs();
    let mut b = b.abs();

    while b != 0 {
        let t = b;
        b = a % b;
        a = t;
    }

    a
}

fn lcm(a: i64, b: i64) -> i64 {
    (a * b).abs() / gcd(a, b)
}

fn lcm3(a: i64, b: i64, c: i64) -> i64 {
    lcm(lcm(a, b), c)
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

    let periods = (
        find_period_single_dimension(&mut moons, |p| p.x),
        find_period_single_dimension(&mut moons, |p| p.y),
        find_period_single_dimension(&mut moons, |p| p.z),
    );

    println!("lcm{:?} = {}", periods, lcm3(periods.0 as i64, periods.1 as i64, periods.2 as i64));
}
