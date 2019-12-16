use std::io;
use std::iter;

use std::collections::VecDeque;

fn fft(input: &Vec<i32>, iterations: i32) -> Vec<i32> {
    let mut value: VecDeque<i32> = VecDeque::from(input.clone());

    for _ in 0..iterations {
        let mut new_value: VecDeque<i32> = VecDeque::new();

        for i in 0..input.len() {
            new_value.push_front((new_value.front().unwrap_or(&0) + value[input.len() - 1 - i]) % 10);
        }
        value = new_value;
    }

    Vec::from(value)
}

fn main() {
    let num_repeats = 10000;
    let mut input = String::new();
    io::stdin().read_line(&mut input)
        .expect("Failed to read input");

    let offset: String = input.chars()
        .take(7)
        .collect();
    let offset= offset.parse().unwrap();

    let digits: Vec<i32> = input.trim().chars()
        .map(|c| c.to_string().parse().unwrap())
        .collect();

    let digits: Vec<i32> = iter::repeat(&digits)
        .flat_map(|d| d.iter())
        .take(num_repeats * digits.len())
        .map(|n| *n)
        .skip(offset)
        .collect();

    let output = fft(&digits, 100);

    let output: String = output.iter()
        .take(8)
        .map(|n| n.to_string())
        .collect();

    println!("{}", output);
}
