use alloc::{String, Vec};
use helper::u16_to_string;
use instruction::Instruction;
use operation::Operation;

pub const MEMORY_SIZE: usize = 1 << 15;
pub const REGISTER_COUNT: usize = 8;

pub struct MachineDetail {
    memory: Option<Vec<u16>>,
    registers: Option<Vec<u16>>,
    stack: Option<Vec<u16>>,
    pc: usize,
    initialized: bool,
}

impl MachineDetail {
    pub const fn new() -> MachineDetail {
        MachineDetail {
            memory: None,
            registers: None,
            stack: None,
            pc: 0,
            initialized: false,
        }
    }

    pub fn load(&mut self, input: &[u16]) -> Result<(), String> {
        if MEMORY_SIZE < input.len() {
            return Err(String::from("Memory exceeds maximum size"));
        }

        self.memory = Some(vec![0; MEMORY_SIZE]);
        self.registers = Some(vec![0; REGISTER_COUNT]);

        self.memory.as_mut().unwrap()[..input.len()].copy_from_slice(input);

        self.stack = Some(Vec::new());
        self.pc = 0;
        self.initialized = true;

        Ok(())
    }

    pub fn execute(&mut self) -> Result<bool, String> {
        if !self.initialized {
            return Err(String::from("No program loaded!"));
        }

        let inst = Instruction::new(self.memory.as_mut().unwrap()[self.pc])?;

        let arg_idx = self.pc + 1;
        let args =
            self.memory.as_mut().unwrap()[arg_idx..(arg_idx + inst.argument_count())].to_vec();
        self.pc += 1 + inst.argument_count();

        inst.execute(&args, self)?;

        Ok(!inst.end_program())
    }

    pub fn value(&self, value: u16) -> Result<u16, String> {
        match value {
            0...32767 => Ok(value),
            32768...32775 => Ok(self.registers.as_ref().unwrap()[(value - 32768) as usize]),
            _ => Err(String::from("Invalid argument: ") + &u16_to_string(value)),
        }
    }

    pub fn set_value(&mut self, argument: u16, value: u16) -> Result<(), String> {
        if argument < 32768 || argument > 32775 {
            return Err(String::from("Invalid register: ") + &u16_to_string(argument));
        }

        let reg_num = (argument - 32768) as usize;
        self.registers.as_mut().unwrap()[reg_num] = self.value(value)?;

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
        self.stack.as_mut().unwrap().push(value);
    }

    pub fn stack_pop(&mut self) -> Result<u16, String> {
        if let Some(val) = self.stack.as_mut().unwrap().pop() {
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

        self.memory.as_mut().unwrap()[addr as usize] = value;
        Ok(())
    }

    pub fn read_memory(&mut self, addr: u16) -> Result<u16, String> {
        let addr = self.get_address(addr)?;
        Ok(self.memory.as_mut().unwrap()[addr as usize])
    }

    fn get_address(&self, addr: u16) -> Result<usize, String> {
        let addr = self.value(addr)? as usize;
        if addr <= MEMORY_SIZE {
            Ok(addr)
        } else {
            Err(String::from("Invalid address: ") + &u16_to_string(addr as u16))
        }
    }
}
