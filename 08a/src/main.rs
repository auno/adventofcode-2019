use std::io;

fn main() {
    let width = 25;
    let height = 6;
    let image_size = width * height;

    let mut input = String::new();
    io::stdin().read_line(&mut input)
        .expect("Failed to read input");

    let digits: Vec<i32> = input.trim().chars()
        .map(|c| c.to_string().parse().unwrap())
        .collect();

    let fewest_zeros_layer = digits.chunks(image_size)
        .min_by_key(|layer| {
            layer.iter()
                .filter(|candidate| **candidate == 0)
                .count()
        })
        .unwrap();

    let num_ones = fewest_zeros_layer.iter()
        .filter(|candidate| **candidate == 1)
        .count();

    let num_twos = fewest_zeros_layer.iter()
        .filter(|candidate| **candidate == 2)
        .count();

    println!("{}", num_ones * num_twos);
}
