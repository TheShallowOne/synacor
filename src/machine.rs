use alloc::{String, Vec};
use alloc::vec_deque::VecDeque;
use helper::u16_to_string;
use instruction::Instruction;
use operation::Operation;

pub const MEMORY_SIZE: usize = 1 << 15;
pub const REGISTER_COUNT: usize = 8;

pub struct Machine {
    memory: Vec<u16>,
    registers: Vec<u16>,
    stack: Vec<u16>,
    pc: usize,
    input: VecDeque<u8>,
    output: VecDeque<u8>,
    debug: bool,
}

impl Machine {
    pub fn new_u8(input: &[u8], debug: bool) -> Result<Machine, String> {
        // little endian
        let mut data = vec![0; input.len() / 2];

        for (i, byte) in input.iter().enumerate() {
            let idx = i / 2;
            let shift = (i % 2) * 8;

            let byte = (u16::from(*byte)) << shift;
            data[idx] |= byte;
        }

        Self::new(&data, debug)
    }

    pub fn new(input: &[u16], debug: bool) -> Result<Machine, String> {
        if MEMORY_SIZE < input.len() {
            return Err(String::from("Memory exceeds maximum size"));
        }

        let mut memory = vec![0; MEMORY_SIZE];
        let registers = vec![0; REGISTER_COUNT];

        memory[..input.len()].copy_from_slice(input);

        Ok(Machine {
            memory,
            registers,
            stack: Vec::new(),
            pc: 0,
            input: VecDeque::new(),
            output: VecDeque::new(),
            debug,
        })
    }

    pub fn execute(&mut self) -> Result<(), ::operation::ExecuteError> {
        let inst = Instruction::new(self.memory[self.pc])?;

        let arg_idx = self.pc + 1;
        let args = self.memory[arg_idx..(arg_idx + inst.argument_count())].to_vec();
        self.pc += 1 + inst.argument_count();

        inst.execute(&args, self)
    }

    pub fn value(&self, value: u16) -> Result<u16, String> {
        match value {
            0...0x7FFF => Ok(value),
            0x8000...0x8007 => Ok(self.registers[(value - 0x8000) as usize]),
            _ => Err(String::from("Invalid argument: ") + &u16_to_string(value)),
        }
    }

    pub fn set_value(&mut self, argument: u16, value: u16) -> Result<(), String> {
        if argument < 0x8000 || argument > 0x8007 {
            return Err(String::from("Invalid register: ") + &u16_to_string(argument));
        }

        let reg_num = (argument - 0x8000) as usize;
        self.registers[reg_num] = self.value(value)?;

        Ok(())
    }

    pub fn set_pc(&mut self, value: u16) -> Result<(), String> {
        let dst = self.value(value)?;
        self.pc = dst as usize;

        Ok(())
    }

    pub fn pc(&self) -> usize {
        self.pc
    }

    pub fn stack_push(&mut self, value: u16) {
        self.stack.push(value);
    }

    pub fn stack_pop(&mut self) -> Result<u16, String> {
        if let Some(val) = self.stack.pop() {
            Ok(val)
        } else {
            Err(String::from("Empty stack"))
        }
    }

    pub fn write_memory(&mut self, addr: u16, value: u16) -> Result<(), String> {
        let addr = self.value(addr)?;
        if addr as usize > MEMORY_SIZE {
            return Err(String::from("Invalid address: ") + &u16_to_string(addr));
        }

        let value = self.value(value)?;

        self.memory[addr as usize] = value;
        Ok(())
    }

    pub fn read_memory(&mut self, addr: u16) -> Result<u16, String> {
        let addr = self.get_address(addr)?;
        Ok(self.memory[addr as usize])
    }

    fn get_address(&self, addr: u16) -> Result<usize, String> {
        let addr = self.value(addr)? as usize;
        if addr <= MEMORY_SIZE {
            Ok(addr)
        } else {
            Err(String::from("Invalid address: ") + &u16_to_string(addr as u16))
        }
    }

    pub fn push_input(&mut self, val: u8) {
        self.input.push_back(val);
    }

    pub fn pop_input(&mut self) -> Option<u8> {
        self.input.pop_front()
    }

    pub fn push_output(&mut self, val: u8) {
        self.output.push_back(val);
    }

    pub fn pop_output(&mut self) -> Option<u8> {
        self.output.pop_front()
    }

    pub fn debug(&self) -> bool {
        self.debug
    }
}
