use std::io::{self, BufRead};
use std::collections::{HashMap, VecDeque};
use std::collections::hash_map::RandomState;

fn find_path_to_root(parents: &HashMap<String, String>, start: &String) -> VecDeque<String> {
    match parents.get(start) {
        Some(parent) => {
            let mut path = find_path_to_root(parents, parent);
            path.push_back(parent.to_owned());
            path
        },
        None => VecDeque::new()
    }
}

fn main() {
    let parents: HashMap<String, String> =
    io::stdin().lock().lines()
        .map(|line| line.unwrap())
        .map(|line| line.split(')').map(|s| String::from(s)).collect())
        .map(|key_value: Vec<String>| {
            let parent = key_value[0].to_owned();
            let child = key_value[1].to_owned();
            (child, parent)
        })
        .collect();

    let mut path1 = find_path_to_root(&parents, &String::from("YOU"));
    let mut path2 = find_path_to_root(&parents, &String::from("SAN"));

    while path1.front() == path2.front() {
        path1.pop_front();
        path2.pop_front();
    }

    eprintln!("{:?}", path1);
    eprintln!("{:?}", path2);
    println!("{}", path1.len() + path2.len());
}
