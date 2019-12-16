use std::io;
use std::iter;

static BASE_PATTERN: [i32; 4] = [0, 1, 0, -1];

fn fft(input: &Vec<i32>, iterations: i32) -> Vec<i32> {
    let mut value: Vec<i32> = input.clone();

    for _ in 0..iterations {
        value = (0..input.len())
            .map(|i| {
                let pattern: Vec<i32> = BASE_PATTERN.iter()
                    .flat_map(|n| iter::repeat(n).take(i + 1))
                    .map(|n| *n)
                    .collect();
                let pattern = iter::repeat(&pattern)
                    .flat_map(|p| p.iter())
                    .map(|n| *n)
                    .skip(1)
                    .take(input.len());

                let result: i32 = value.iter()
                    .zip(pattern)
                    .map(|(a, b)| a * b)
                    .sum();

                result.abs() % 10
            })
            .collect();
    }

    value
}

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input)
        .expect("Failed to read input");

    let digits: Vec<i32> = input.trim().chars()
        .map(|c| c.to_string().parse().unwrap())
        .collect();

    let output: String = fft(&digits, 100).iter()
        .take(8)
        .map(|n| n.to_string())
        .collect();

    println!("{}", output);
}
