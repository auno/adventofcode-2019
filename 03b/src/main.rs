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

fn calc_positions(line: Vec<(char, i32)>) -> Vec<(usize, (i32, i32))> {
    let mut position = (0, 0);
    let positions: Vec<(usize, (i32, i32))> = line.iter()
        .flat_map(|(d,v)| {
            let points: Vec<(i32, i32)> = (1..v+1)
                .map(|dist| mv(position, *d, dist))
                .collect();

            position = *points.last().unwrap();
            //eprintln!(" pos: {:?}", position);

            points
        })
        .enumerate()
        .collect();

    positions
}

fn get_dist(line: &Vec<(usize, (i32, i32))>, point: &(i32,i32)) -> usize {
    *line.iter()
        .filter(|(_,p_candidate)| p_candidate == point)
        .map(|(d,_)| d + 1)
        .collect::<Vec<usize>>()
        .first()
        .unwrap()
}

fn main() {
    let line1 = read_line();
    let line2 = read_line();

    let line1 = calc_positions(line1);
    //eprintln!("-----------");
    let line2 = calc_positions(line2);

    let up1: HashSet<(i32, i32)> = line1.iter().map(|(_,p)| *p).collect();
    let up2: HashSet<(i32, i32)> = line2.iter().map(|(_,p)| *p).collect();

    let common_points: Vec<&(i32, i32)> = up1.intersection(&up2).collect();

    let closest_intersection_manhattan = common_points.iter()
        //.inspect(|x| eprintln!(" {:?}", x))
        .map(|(x, y)| x.abs() + y.abs())
        //.inspect(|d| eprintln!(" dist {}", d))
        .min()
        .unwrap();

    let closest_intersection_wire_length = common_points.iter()
        .map(|p| {
            let d1 = get_dist(&line1, p);
            let d2 = get_dist(&line2, p);
            d1 + d2
        })
        .min()
        .unwrap();

    println!("a: {}", closest_intersection_manhattan);
    println!("b: {}", closest_intersection_wire_length);
}
