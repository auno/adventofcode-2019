use std::io;

mod int_code {
  use Op::*;

  enum Op {
    Add,
    Mul,
    Input,
    Output,
    JmpNonZero,
    JmpZero,
    LessThan,
    Equals,
    Halt
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

  pub struct Computer {
    memory: Vec<i32>,
    input: Vec<i32>,
    output: Vec<i32>,
    pc: usize
  }

  impl Computer {
    pub fn new(memory: &Vec<i32>, input: &Vec<i32>) -> Computer {
      let mut computer = Computer {
        memory: memory.to_vec(),
        input: input.to_vec(),
        output: Vec::new(),
        pc: 0,
      };

      computer.input.reverse();
      computer
    }

    pub fn get_output(&self) -> &Vec<i32> {
      &self.output
    }

    fn is_immediate(opcode: i32, position: u32) -> bool {
      let x = (opcode / 10_i32.pow(position + 2)) % 10 == 1;
      //println!("is_immediate {} {} -> {}", opcode, position, x);
      x
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

    fn execute_instruction(&mut self, op: Op, params: Vec<i32>) {
      if params.len() != op.num_params().0 as usize {
        panic!("Not correct amount of parameters: {} != {} {}", params.len(), op.num_params().0, op as i32);
      }
      let memory = &mut self.memory;

      match op {
        Add => { memory[params[2] as usize] = params[0] + params[1]; },
        Mul => { memory[params[2] as usize] = params[0] * params[1]; },
        Input => { memory[params[0] as usize] = self.input.pop().unwrap(); },
        Output => { self.output.push(params[0]); }
        JmpNonZero => { self.pc = if params[0] != 0 { params[1] as usize } else { self.pc }; },
        JmpZero => { self.pc = if params[0] == 0 { params[1] as usize } else { self.pc }; },
        LessThan => { memory[params[2] as usize] = if params[0] < params[1] { 1 } else { 0 }; },
        Equals => { memory[params[2] as usize] = if params[0] == params[1] { 1 } else { 0 }; },
        Halt => panic!("Impossible")
      }
    }

    pub fn run(&mut self) -> Vec<i32> {
      loop {
        let opcode = self.memory[self.pc];
        let op = Op::decode(opcode).unwrap();
        let (num_params, has_dest) = op.num_params();

        let old_pc = self.pc;

        match op {
          Halt => break,
          _ => {
            let params = self.get_params((num_params, has_dest), opcode);
            self.execute_instruction(op, params);
          }
        }

        if self.pc == old_pc {
          self.pc += num_params as usize + 1
        }
      }

      self.memory.to_vec()
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

  let input = vec![5];

  let mut computer = int_code::Computer::new(&memory, &input);
  let memout = computer.run();

  println!("{:?}", memout);
  println!("{:?}", computer.get_output());
}
