use std::io;

fn get_first_non_transparent_pixel(pixels: Vec<i32>) -> i32 {
    *pixels.iter()
        .find(|pixel| **pixel != 2)
        .unwrap()
}

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

    let layers: Vec<Vec<i32>> = digits.chunks(image_size)
        .map(|layer| layer.to_vec())
        .collect();

    let image: Vec<i32> = (0..image_size)
        .map(|i| get_first_non_transparent_pixel(layers.iter().map(|layer| layer[i]).collect()))
        .collect();

    image.chunks(width)
        .for_each(|line| {
            let line: String = line.iter().map(|digit| format!("{}", digit)).collect();
            println!("{}", line.replace("0", " ").replace("1", "▓"));
        });

//    println!("{:?}", image);
}
