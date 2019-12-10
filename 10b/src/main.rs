use std::io;
use std::env;
use std::io::BufRead;
use std::collections::HashMap;

type Point = (i64, i64);

fn add_point(a: &Point, b: &Point) -> Point {
    (a.0 + b.0, a.1 + b.1)
}

fn sub_point(a: &Point, b: &Point) -> Point {
    (a.0 - b.0, a.1 - b.1)
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

fn direction(p: Point) -> Point {
    let d = gcd(p.0, p.1);
    (p.0 / d, p.1 / d)
}

fn distance(a: Point, b: Point) -> i64 {
    let xdiff: i64 = (a.0 - b.0).abs();
    let ydiff: i64 = (a.1 - b.1).abs();
    (((xdiff.pow(2) + ydiff.pow(2)) as f64).sqrt() * 1000000000000000_f64) as i64
}

fn point_to_sort_key((x, y): &Point) -> i64 {
    let angle_radians = -(*x as f64).atan2(*y as f64);
    let angle_degrees = angle_radians.to_degrees();
    ((angle_degrees * 1000000000000000_f64) as i64 + 180000000000000000) % 360000000000000000
}

fn main() {
    let lines: Vec<String> = io::stdin().lock().lines()
        .map(|s| s.unwrap())
        .collect();

    let origo: Vec<i64> = env::args()
        .skip(1)
        .take(2)
        .map(|arg| arg.parse().expect("Expected input parameters to be integers"))
        .collect();

    let origo = (origo[0], origo[1]);

    let target: usize = env::args()
        .nth(3)
        .unwrap()
        .parse()
        .unwrap();

    let asteroids: Vec<Point> = lines.iter()
        .enumerate()
        .flat_map(|(x, line)| {
            line.chars()
                .enumerate()
                .filter(|(_, c)| *c == '#')
                .map(move |(y,_)| {
                    (y as i64, x as i64)
                })
        })
        .collect();

    let mut asteroids_by_direction: HashMap<Point, Vec<Point>> = HashMap::new();

    asteroids.iter()
        .filter(|a| *a != &origo)
        .map(|other| sub_point(other, &origo))
        .map(|a| (direction(a), a))
        .for_each(|(d, a)| {
            if !asteroids_by_direction.contains_key(&d) {
                asteroids_by_direction.insert(d, Vec::new());
            }

            asteroids_by_direction.get_mut(&d).unwrap().push(a);
        });

    let mut keys_by_angle: Vec<(i64, i64)> = asteroids_by_direction.keys()
        .map(|key| key.to_owned())
        .collect();

    keys_by_angle.sort_by_key(point_to_sort_key);

    let longest_chain = keys_by_angle.iter()
        .map(|key| {
            let chain = asteroids_by_direction.get_mut(key).unwrap();
            chain.sort_by_key(|p| distance((0, 0), *p));
            chain.len()
        })
        .max()
        .unwrap();

    let mut counter = 1;

    (0..longest_chain)
        .for_each(|i| {
            keys_by_angle.iter()
                .for_each(|key| {
                    match asteroids_by_direction.get(&key.to_owned()).unwrap().get(i) {
                        Some(p) => {
                            let point = add_point(p, &origo);
                            if counter == target {
                                println!("{:3}: {:?} -> {}", counter, point, point.0 * 100 + point.1);
                            }
                            counter += 1;
                        },
                        None => ()
                    }
                })
        });
}
