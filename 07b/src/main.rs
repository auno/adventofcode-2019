use std::io;
use std::collections::VecDeque;
use int_code::{Computer,State};

struct Permutations {
    n: usize,
    c: VecDeque<usize>,
    a: VecDeque<usize>,
    i: usize,
    first: bool
}

impl Permutations {
    fn new(n: usize) -> Permutations {
        let mut c = VecDeque::new();
        c.resize(n, 0usize);
        let a: VecDeque<usize> = (0..n).collect();

        Permutations { n, c, a, i: 0, first: true }
    }
}

/* Heap's algorithm for permutation generation */
impl Iterator for Permutations {
    type Item = VecDeque<usize>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.first {
            self.first = false;
            return Some(self.a.to_owned());
        }

        while self.i < self.n {
            if self.c[self.i] < self.i {
                if self.i % 2 == 0 {
                    self.a.swap(0, self.i);
                } else {
                    self.a.swap(self.c[self.i], self.i);
                }

                self.c[self.i] += 1;
                self.i = 0;

                return Some(self.a.to_owned());
            } else {
                self.c[self.i] = 0;
                self.i += 1;
            }
        }

        None
    }
}

//fn main() {
//    generate(3);
//    eprintln!("---------------");
//
//    Permutations::new(3)
//        .into_iter()
//        .for_each(|x| eprintln!("{:?}", x))
//}

mod int_code {
    use Op::*;
    use State::*;
    use std::collections::VecDeque;

    #[derive(Debug)]
    enum Op {
        Add,
        Mul,
        Input,
        Output,
        JmpNonZero,
        JmpZero,
        LessThan,
        Equals,
        Halt,
    }

    impl Op {
        fn decode(opcode: i32) -> Result<Op, String> {
            match opcode % 100 {
                1 => Result::Ok(Add),
                2 => Result::Ok(Mul),
                3 => Result::Ok(Input),
                4 => Result::Ok(Output),
                5 => Result::Ok(JmpNonZero),
                6 => Result::Ok(JmpZero),
                7 => Result::Ok(LessThan),
                8 => Result::Ok(Equals),
                99 => Result::Ok(Halt),
                x => Err(format!("Unknown opcode: {}", x))
            }
        }

        fn num_params(&self) -> (u32, bool) {
            match self {
                Add | Mul => (3, true),
                Input => (1, true),
                Output => (1, false),
                JmpNonZero | JmpZero => (2, false),
                LessThan | Equals => (3, true),
                Halt => (0, false)
            }
        }
    }

    #[derive(Debug)]
    pub enum State {
        NotStarted,
        Running,
        Blocked,
        Halted
    }

    pub struct Computer {
        memory: Vec<i32>,
        input: VecDeque<i32>,
        output: VecDeque<i32>,
        pc: usize,
        state: State
    }

    impl Computer {
        pub fn new(memory: &Vec<i32>, initial_input: Option<&Vec<i32>>) -> Computer {
            let input = match initial_input {
                Some(input) => VecDeque::from(input.to_owned()),
                None => VecDeque::new(),
            };

            Computer {
                memory: memory.to_owned(),
                input,
                output: VecDeque::new(),
                pc: 0,
                state: NotStarted
            }
        }

        pub fn push_input(&mut self, value: i32) {
            self.input.push_back(value);
        }

        pub fn pop_output(&mut self) -> i32 {
            self.output.pop_front().unwrap()
        }

        pub fn get_state(&mut self) -> &State {
            &self.state
        }

        fn is_immediate(opcode: i32, position: u32) -> bool {
            (opcode / 10_i32.pow(position + 2)) % 10 == 1
        }

        fn read_absolute(&self, address: usize) -> i32 {
            self.memory[address]
        }

        fn read_relative(&self, offset: i32) -> i32 {
            self.read_absolute((self.pc as i32 + offset) as usize)
        }

        fn get_param(&self, position: u32, opcode: i32) -> i32 {
            let value_or_ref = self.read_relative(position as i32 + 1);

            match Computer::is_immediate(opcode, position) {
                true => value_or_ref,
                false => self.read_absolute(value_or_ref as usize)
            }
        }

        fn get_params(&self, (num_params, has_dest): (u32, bool), opcode: i32) -> Vec<i32> {
            let num_input_params = match has_dest {
                true => num_params - 1,
                false => num_params
            };

            let mut params: Vec<i32> = (0..num_input_params)
                .map(|position| self.get_param(position, opcode))
                .collect();

            if has_dest {
                params.push(self.read_relative(num_params as i32));
            }

            params
        }

        fn execute_instruction(&mut self, op: &Op, opcode: i32) {
            let (num_params, has_dest) = op.num_params();
            let params = self.get_params((num_params, has_dest), opcode);

            if params.len() != op.num_params().0 as usize {
                panic!("Not correct amount of parameters for {:?}: {} != {} ", op, params.len(), op.num_params().0);
            }
            let memory = &mut self.memory;

            match op {
                Add => { memory[params[2] as usize] = params[0] + params[1]; }
                Mul => { memory[params[2] as usize] = params[0] * params[1]; }
                Input => { memory[params[0] as usize] = self.input.pop_front().unwrap(); }
                Output => { self.output.push_back(params[0]); }
                JmpNonZero => { self.pc = if params[0] != 0 { params[1] as usize } else { self.pc }; }
                JmpZero => { self.pc = if params[0] == 0 { params[1] as usize } else { self.pc }; }
                LessThan => { memory[params[2] as usize] = if params[0] < params[1] { 1 } else { 0 }; }
                Equals => { memory[params[2] as usize] = if params[0] == params[1] { 1 } else { 0 }; }
                Halt => panic!("Impossible")
            }
        }

        pub fn run(&mut self) -> &State {
            if let Halted = self.state {
                panic!("Cannot start halted computer");
            }

            self.state = Running;

            loop {
                let opcode = self.memory[self.pc];
                let op = Op::decode(opcode).unwrap();

                let old_pc = self.pc;

                match op {
                    Halt => {
                        self.state = Halted;
                        break;
                    },
                    Input => match self.input.len() {
                        0 => {
                            self.state = Blocked;
                            break;
                        },
                        _ => self.execute_instruction(&op, opcode)
                    }
                    _ => {
                        self.execute_instruction(&op, opcode);
                    }
                }

                if self.pc == old_pc {
                    let (num_params, _) = op.num_params();
                    self.pc += num_params as usize + 1
                }
            }

            &self.state
        }
    }
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

    let max_signal = Permutations::new(5)
        .into_iter()
        .map(|permutation| {
            let mut computers: Vec<Computer> = permutation.iter()
                .map(|phase| Computer::new(&memory, Some(&vec![*phase as i32 + 5])))
                .collect();

            let mut signal = 0;

            loop {
                computers.iter_mut()
                    .for_each(|computer| {
                        computer.push_input(signal);
                        computer.run();
                        signal = computer.pop_output();
                    });

                match computers[4].get_state() {
                    State::Halted => break,
                    _ => ()
                }
            }

            signal
        })
        .max()
        .unwrap();

    println!("{}", max_signal);
}
