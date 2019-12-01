use std::io::{self, BufRead};

fn main() {
    let sum: i32 = io::stdin().lock().lines()
        .map(|line| line.unwrap())
        .map(|line| line.parse().unwrap())
        .map(|mass: i32| mass / 3 - 2)
        .sum();

    println!("{}", sum);
}
