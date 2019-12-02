use std::io::{self, BufRead};

fn ensure_len(memory: &Vec<i32>, i: i32) {
    if i >= (memory.len() as i32) {
        panic!("out of memory");
    }
}

fn op(memory: &mut Vec<i32>, pc: usize) {
    let op = memory[pc];
    let a: usize = memory[pc+1] as usize;
    let b: usize = memory[pc+2] as usize;
    let dest: usize = memory[pc+3] as usize;

    eprint!("; {} {} {}", a, b, dest);

    match op {
        1 => { eprintln!("; {} + {} = {} -> {}", memory[a], memory[b], memory[a]+memory[b], dest); memory[dest] = memory[a] + memory[b] },
        2 => { eprintln!("; {} * {} = {} -> {}", memory[a], memory[b], memory[a]*memory[b], dest); memory[dest] = memory[a] * memory[b] },
        _ => panic!("Impossible")
    }
}

fn main() {
    let mut memory = String::new();
    io::stdin().read_line(&mut memory)
        .expect("Failed to read input");

    let mut memory: Vec<i32> = memory
        .split(',')
        .map(|s| s.trim())
        .map(|s| s.parse().unwrap())
        .collect();

    memory[1] = 12;
    memory[2] = 2;

    let mut pc = 0;
    loop {
        eprint!("pc: {}, op: {}", pc, memory[pc]);
        match memory[pc] {
            1 | 2 => op(&mut memory, pc),
            99 => { eprintln!(); break },
            _ => panic!("Don't know what to do")
        }

        pc += 4;
    }

    eprintln!("{}", memory.first().unwrap());
}
