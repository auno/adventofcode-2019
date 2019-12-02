use std::io;

fn op(memory: &mut Vec<i32>, pc: usize) {
    let op = memory[pc];
    let a: usize = memory[pc+1] as usize;
    let b: usize = memory[pc+2] as usize;
    let dest: usize = memory[pc+3] as usize;

    //eprint!("; {} {} {}", a, b, dest);

    match op {
        1 => { /*eprintln!("; {} + {} = {} -> {}", memory[a], memory[b], memory[a]+memory[b], dest);*/ memory[dest] = memory[a] + memory[b] },
        2 => { /*eprintln!("; {} * {} = {} -> {}", memory[a], memory[b], memory[a]*memory[b], dest);*/ memory[dest] = memory[a] * memory[b] },
        _ => panic!("Impossible")
    }
}

fn compute(mut memory: Vec<i32>) -> Vec<i32> {
    let mut pc = 0;
    loop {
        //eprint!("pc: {}, op: {}", pc, memory[pc]);
        match memory[pc] {
            1 | 2 => op(&mut memory, pc),
            99 => { /*eprintln!();*/ break },
            _ => panic!("Don't know what to do")
        }

        pc += 4;
    }

    memory.to_vec()
}

fn main() {
    let mut memory = String::new();
    io::stdin().read_line(&mut memory)
        .expect("Failed to read input");

    let memory: Vec<i32> = memory
        .split(',')
        .map(|s| s.trim())
        .map(|s| s.parse().unwrap())
        .collect();

    for noun in 0..100 {
        for verb in 0..100 {
            //eprintln!("Testing {}, {}", noun, verb);
            let mut copy = memory.to_vec();

            copy[1] = noun;
            copy[2] = verb;

            let copy = compute(copy);

            if *(copy.first().unwrap()) == 19690720 {
                println!("{}, {} -> {}", noun, verb, 100 * noun + verb);
                return;
            }
        }
    }
}
