use std::io::{self, BufRead};
use std::collections::{HashMap, VecDeque};
use std::collections::hash_map::RandomState;

fn main() {
    let mut neighbors: HashMap<String, Vec<String>, RandomState> = HashMap::new();
    io::stdin().lock().lines()
        .map(|line| line.unwrap())
        .map(|line| line.split(')').map(|s| String::from(s)).collect())
        .for_each(|key_value: Vec<String>| {
            let key = &key_value[0];
            let value = &key_value[1];

            if !neighbors.contains_key(key) {
                neighbors.insert(key.to_owned(), Vec::new());
            }

            let targets: &mut Vec<String> = neighbors.get_mut(key).unwrap();
            targets.push(String::from(value));
        });

    let mut distances = HashMap::new();
    distances.insert(String::from("COM"), 0u32);

    let mut queue = VecDeque::new();
    queue.push_back(String::from("COM"));

    loop {
        match queue.pop_front() {
            Some(cur_vertex) => {
                let cur_distance = *distances.get(&cur_vertex).unwrap();

                neighbors
                    .get(&cur_vertex).iter()
                    .for_each(|&cur_neighbors| {
                        cur_neighbors.iter()
                            .for_each(|neighbor| {
                                if !distances.contains_key(neighbor) || cur_distance < *distances.get(neighbor).unwrap() {
                                    distances.insert(String::from(neighbor), cur_distance + 1);
                                }

                                queue.push_back(neighbor.to_owned());
                            });
                    });
            },
            None => break
        }
    }

    let sum_distances: u32 = distances.values().sum();
    eprintln!("{:?}", sum_distances);
}
