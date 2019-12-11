use std::io;
use int_code::{Computer,Unit,State};
use std::collections::HashMap;
use std::cmp::{max, min};

use Color::*;
use Rotation::*;
use Direction::*;

mod int_code {
    use Op::*;
    use State::*;
    use std::collections::VecDeque;
    use crate::int_code::AddressingMode::{Absolute, Immediate, Relative};

    pub type Unit = i128;

    fn debug(s: String) {
        if std::env::var("INTCODE_TRACE").is_ok() {
            eprint!("{}", s);
        }
    }

    #[derive(Debug)]
    enum Op {
        Add,
        Mul,
        Input,
        Output,
        JumpNZ,
        JumpZ,
        LessThan,
        Equals,
        ModRel,
        Halt,
    }

    impl Op {
        fn decode(opcode: Unit) -> Result<Op, String> {
            match opcode % 100 {
                1 => Ok(Add),
                2 => Ok(Mul),
                3 => Ok(Input),
                4 => Ok(Output),
                5 => Ok(JumpNZ),
                6 => Ok(JumpZ),
                7 => Ok(LessThan),
                8 => Ok(Equals),
                9 => Ok(ModRel),
                99 => Ok(Halt),
                x => Err(format!("Unknown opcode: {}", x))
            }
        }

        fn num_params(&self) -> (usize, bool) {
            match self {
                Add | Mul => (3, true),
                Input => (1, true),
                Output => (1, false),
                JumpNZ | JumpZ => (2, false),
                LessThan | Equals => (3, true),
                ModRel => (1, false),
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

    pub enum AddressingMode {
        Absolute,
        Immediate,
        Relative
    }

    pub struct Computer {
        memory: Vec<Unit>,
        input: VecDeque<Unit>,
        output: VecDeque<Unit>,
        pc: usize,
        state: State,
        relative_base: usize
    }

    pub fn get_addressing_mode(opcode: Unit, position: usize) -> AddressingMode {
        match (opcode / (10 as Unit).pow(position as u32 + 2)) % 10 {
            0 => Absolute,
            1 => Immediate,
            2 => Relative,
            n => panic!(format!("Unknown addressing mode: {}", n))
        }
    }

    impl Computer {
        pub fn new(memory: &Vec<Unit>, initial_input: Option<&Vec<Unit>>) -> Computer {
            let input = match initial_input {
                Some(input) => VecDeque::from(input.to_owned()),
                None => VecDeque::new(),
            };

            Computer {
                memory: memory.to_owned(),
                input,
                output: VecDeque::new(),
                pc: 0,
                state: NotStarted,
                relative_base: 0
            }
        }

        #[allow(dead_code)]
        pub fn push_input(&mut self, value: Unit) {
            self.input.push_back(value);
        }

        #[allow(dead_code)]
        pub fn pop_output(&mut self) -> Unit {
            self.output.pop_front().unwrap()
        }

        #[allow(dead_code)]
        pub fn get_input(&self) -> &VecDeque<Unit> {
            &self.input
        }

        #[allow(dead_code)]
        pub fn get_output(&self) -> &VecDeque<Unit> {
            &self.output
        }

        #[allow(dead_code)]
        pub fn get_memory(&self) -> &Vec<Unit> {
            &self.memory
        }

        #[allow(dead_code)]
        pub fn get_state(&mut self) -> &State {
            &self.state
        }

        fn ensure_adressable(&mut self, address: usize) {
            if address >= self.memory.len() {
                self.memory.resize(address + 1, 0);
            }
        }

        fn write_absolute(&mut self, address: usize, value: Unit) {
            self.ensure_adressable(address);
            self.memory[address] = value;
        }

        fn read_absolute(&mut self, address: usize) -> Unit {
            self.ensure_adressable(address);
            self.memory[address]
        }

        fn read_immediate(&mut self, offset: usize) -> Unit {
            self.read_absolute(self.pc + offset)
        }

        fn read_relative(&mut self, offset: isize) -> Unit {
            let address = self.relative_base as isize + offset;

            if address < 0 {
                panic!("[{}] Address less than zero: {}", self.pc, address);
            }

            self.read_absolute(address as usize)
        }

        fn get_param(&mut self, position: usize, opcode: Unit) -> Unit {
            let value_or_ref = self.read_immediate(position + 1);

            match get_addressing_mode(opcode, position) {
                Absolute => {
                    debug(format!(" [{}]", value_or_ref));
                    self.read_absolute(value_or_ref as usize)
                },
                Immediate => {
                    debug(format!(" {}", value_or_ref));
                    value_or_ref
                },
                Relative => {
                    debug(format!(" [{}+{}]", self.relative_base, value_or_ref));
                    self.read_relative(value_or_ref as isize)
                },
            }
        }

        fn get_dest(&mut self, position: usize, opcode: Unit) -> Unit {
            let value_or_ref = self.read_immediate(position + 1);

            match get_addressing_mode(opcode, position) {
                Absolute => {
                    debug(format!(" -> [{}]", value_or_ref));
                    value_or_ref
                },
                Relative => {
                    debug(format!(" -> [{}+{}]", self.relative_base, value_or_ref));
                    self.relative_base as Unit + value_or_ref
                },
                Immediate => { panic!("[{}] Destination cannot be immediate", self.pc)}
            }
        }

        fn get_params(&mut self, (num_params, has_dest): (usize, bool), opcode: Unit) -> Vec<Unit> {
            let num_input_params = match has_dest {
                true => num_params - 1,
                false => num_params
            };

            let mut params: Vec<Unit> = (0..num_input_params)
                .map(|position| self.get_param(position, opcode))
                .collect();

            if has_dest {
                let dest = self.get_dest(num_params - 1, opcode);
                params.push(dest);
            }

            params
        }

        fn execute_instruction(&mut self, op: &Op, opcode: Unit) {
            let (num_params, has_dest) = op.num_params();
            let params = self.get_params((num_params, has_dest), opcode);

            if params.len() != num_params {
                panic!("Incorrect amount of parameters");
            }

            match op {
                Add => { self.write_absolute(params[2] as usize, params[0] + params[1]); }
                Mul => { self.write_absolute(params[2] as usize, params[0] * params[1]); }
                Input => {
                    let value = self.input.pop_front().unwrap();
                    self.write_absolute(params[0] as usize, value);
                }
                Output => { self.output.push_back(params[0]); }
                JumpNZ => { self.pc = if params[0] != 0 { params[1] as usize } else { self.pc }; }
                JumpZ => { self.pc = if params[0] == 0 { params[1] as usize } else { self.pc }; }
                LessThan => { self.write_absolute(params[2] as usize, if params[0] < params[1] { 1 } else { 0 }); }
                Equals => { self.write_absolute(params[2] as usize, if params[0] == params[1] { 1 } else { 0 }); }
                ModRel => {
                    self.relative_base = (self.relative_base as isize + params[0] as isize) as usize;
                    debug(format!(" -> {}", self.relative_base));
                }
                Halt => panic!("Impossible")
            }
        }

        #[allow(dead_code)]
        pub fn run(&mut self) -> &State {
            if let Halted = self.state {
                panic!("Cannot start halted computer");
            }

            self.state = Running;

            loop {
                let opcode = self.memory[self.pc];
                let op = Op::decode(opcode).unwrap();

                let old_pc = self.pc;

                debug(format!("[{:5}] {:>8}", self.pc, format!("{:?}", op)));

                match op {
                    Halt => {
                        debug(String::from("\n"));
                        self.state = Halted;
                        break;
                    },
                    Input => match self.input.len() {
                        0 => {
                            debug(String::from("\n"));
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

                debug(String::from("\n"));
            }

            &self.state
        }
    }
}

#[derive(Clone)]
enum Direction {
    Up,
    Down,
    Left,
    Right
}

#[derive(Clone)]
enum Color {
    Black,
    White
}

#[derive(Clone)]
enum Rotation {
    CounterClockwise,
    Clockwise
}

impl Color {
    fn to_int(&self) -> int_code::Unit {
        match self {
            Color::Black => 0,
            Color::White => 1,
        }
    }

    fn from_int(int: int_code::Unit) -> Color {
        match int {
            0 => Black,
            1 => White,
            _ => panic!("Unknown color: {}", int),
        }
    }

    fn to_char(&self) -> char {
        match self {
            Color::Black => ' ',
            Color::White => '▓',
        }
    }
}

impl Rotation {
    fn from_int(int: int_code::Unit) -> Rotation {
        match int {
            0 => Rotation::CounterClockwise,
            1 => Rotation::Clockwise,
            _ => panic!("Unknown color: {}", int),
        }
    }
}

fn rotate(direction: &Direction, rotation: &Rotation) -> Direction {
    match rotation {
        CounterClockwise => match direction {
            Up => Left,
            Right => Up,
            Down => Right,
            Left => Down,
        },
        Clockwise => match direction {
            Up => Right,
            Right => Down,
            Down => Left,
            Left => Up,
        },
    }
}

fn moveit(position: &(i32, i32), direction: &Direction) -> (i32, i32) {
    match direction {
        Up => (position.0 - 1, position.1),
        Down => (position.0 + 1, position.1),
        Right => (position.0, position.1 + 1),
        Left => (position.0, position.1 - 1),
    }
}

fn main() {
    let mut memory = String::new();
    io::stdin().read_line(&mut memory)
        .expect("Failed to read input");

    let memory: Vec<Unit> = memory
        .split(',')
        .map(|s| s.trim())
        .map(|s| s.parse().unwrap())
        .collect();

    let mut position = (0, 0);
    let mut direction = Up;
    let mut canvas: HashMap<(i32, i32), Color> = HashMap::new();
    canvas.insert(position, White);

    let mut computer = Computer::new(&memory, None);

    loop {
        let current_color = match canvas.get(&position) {
            Some(c) => c.to_owned(),
            None => Black,
        };

        computer.push_input(current_color.to_int());

        let new_color = match computer.run() {
            State::NotStarted => panic!("Computer not started"),
            State::Running => panic!("Computer running"),
            State::Blocked => Color::from_int(computer.pop_output()),
            State::Halted => break
        };

        canvas.insert(position, new_color);

        let rotation = match computer.run() {
            State::NotStarted => panic!("Computer not started"),
            State::Running => panic!("Computer running"),
            State::Blocked => Rotation::from_int(computer.pop_output()),
            State::Halted => break
        };

        direction = rotate(&direction, &rotation);
        position = moveit(&position, &direction);
    }

    let (upper, lower, right, left) = canvas.keys()
        .fold((0, 0, 0, 0), |acc, value| {
            (
                max(acc.0, value.0),
                min(acc.1, value.0),
                max(acc.2, value.1),
                min(acc.3, value.1)
            )
        });

    (lower..upper+1).for_each(|x| {
        (left..right+1).for_each(|y| {
            let char = match canvas.get(&(x, y)) {
                Some(color) => color.to_char(),
                None => Black.to_char(),
            };
            print!("{}", char);
        });

        println!();
    });
}
