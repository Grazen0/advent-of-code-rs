use std::fmt::{Display, Formatter};

pub type IntcodeResult<T> = Result<T, IntcodeError>;

#[derive(Debug, Clone)]
pub enum IntcodeError {
    IllegalOpcode(i64),
}

impl Display for IntcodeError {
    fn fmt(&self, f: &mut Formatter) -> Result<(), std::fmt::Error> {
        match self {
            Self::IllegalOpcode(op) => write!(f, "illegal opcode: {}", op),
        }
    }
}

impl std::error::Error for IntcodeError {}

#[derive(Debug, Clone)]
pub enum IntcodeState {
    Active,
    Halted,
}

impl Default for IntcodeState {
    fn default() -> Self {
        Self::Active
    }
}

#[derive(Debug, Clone, Default)]
pub struct IntcodeMachine {
    pc: usize,
    memory: Vec<i64>,
    state: IntcodeState,
}

impl IntcodeMachine {
    pub fn new() -> Self {
        Self {
            pc: 0,
            memory: Vec::new(),
            state: IntcodeState::Active,
        }
    }

    pub fn pc(&self) -> usize {
        self.pc
    }

    pub fn pc_mut(&mut self) -> &mut usize {
        &mut self.pc
    }

    pub fn memory(&self) -> &[i64] {
        &self.memory
    }

    pub fn memory_mut(&mut self) -> &mut [i64] {
        &mut self.memory
    }

    pub fn state(&self) -> &IntcodeState {
        &self.state
    }

    pub fn load_program(&mut self, program: &[i64]) {
        if program.len() > self.memory.len() {
            self.memory.resize(program.len(), 0);
        }

        self.memory[..program.len()].copy_from_slice(program);
    }

    pub fn run_until_halt(&mut self) -> IntcodeResult<()> {
        loop {
            self.step()?;

            if let IntcodeState::Halted = self.state {
                break;
            }
        }

        Ok(())
    }

    pub fn step(&mut self) -> IntcodeResult<()> {
        match self.state {
            IntcodeState::Active => {
                let opcode = self.fetch();

                match opcode {
                    1 => {
                        let src_a = self.fetch() as usize;
                        let src_b = self.fetch() as usize;
                        let dest = self.fetch() as usize;
                        self.memory[dest] = self.memory[src_a] + self.memory[src_b];
                    }
                    2 => {
                        let src_a = self.fetch() as usize;
                        let src_b = self.fetch() as usize;
                        let dest = self.fetch() as usize;
                        self.memory[dest] = self.memory[src_a] * self.memory[src_b];
                    }
                    99 => self.state = IntcodeState::Halted,
                    _ => return Err(IntcodeError::IllegalOpcode(opcode)),
                }
            }
            IntcodeState::Halted => {}
        }

        Ok(())
    }

    fn fetch(&mut self) -> i64 {
        let value = self.memory[self.pc];
        self.pc += 1;
        value
    }
}
