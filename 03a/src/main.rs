use std::io;
use std::collections::HashSet;

fn read_line() -> Vec<(char, i32)> {
    let mut line = String::new();
    io::stdin().read_line(&mut line)
        .expect("Failed to read input");

    let line: Vec<(char,i32)> = line
        .split(',')
        .map(|s| s.trim())
        .map(|s| (s.chars().next().unwrap(), s.chars().skip(1).collect::<String>()))
        .map(|(d,v): (char, String)| (d, v.parse().unwrap()))
        //.inspect(|x| eprintln!(" foo {:?}", x))
        .collect();

    line
}

fn mv((x, y): (i32, i32), direction: char, distance: i32) -> (i32, i32) {
    match direction {
        'U' => (x, y + distance),
        'D' => (x, y - distance),
        'R' => (x + distance, y),
        'L' => (x - distance, y),
        _ => panic!("Unknown direction: {}", direction)
    }
}

fn calc_positions(line: Vec<(char, i32)>) -> HashSet<(i32, i32)> {
    let mut position = (0, 0);
    let positions: HashSet<(i32, i32)> = line.iter()
        .flat_map(|(d,v)| {
            let points: Vec<(i32, i32)> = (1..v+1)
                .map(|dist| mv(position, *d, dist))
                .collect();

            position = *points.last().unwrap();
            //eprintln!(" pos: {:?}", position);

            points
        })
        .collect();

    positions
}

fn main() {
    let line1 = read_line();
    let line2 = read_line();

    let positions1 = calc_positions(line1);
    //eprintln!("-----------");
    let positions2 = calc_positions(line2);
    eprintln!("1: {}, 2: {}", positions1.len(), positions2.len());

    let closest_intersection = positions1.intersection(&positions2)
        //.inspect(|x| eprintln!(" {:?}", x))
        .map(|(x, y)| x.abs() + y.abs())
        //.inspect(|d| eprintln!(" dist {}", d))
        .min()
        .unwrap();

    println!("{}", closest_intersection);
}
