use std::io::{self, BufRead};

fn fuel(mass: i32) -> i32 {
    match mass / 3 - 2 {
        x if x <= 0 => 0,
        x => x + fuel(x)
    }
}

fn main() {
    let sum: i32 = io::stdin().lock().lines()
        .map(|line| line.unwrap())
        .map(|line| line.parse().unwrap())
        .map(fuel)
        .sum();

    println!("{}", sum);
}
