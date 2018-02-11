use alloc::String;
use instruction::Instruction;
use machine::MachineDetail;

pub trait Operation {
    fn execute(&self, args: &[u16], ma: &mut MachineDetail) -> Result<(), String>;
}

impl Operation for Instruction {
    fn execute(&self, args: &[u16], ma: &mut MachineDetail) -> Result<(), String> {
        //self.log(args);
        use self::Instruction::*;
        match *self {
            Halt | Noop => {}

            // In out
            In => unimplemented!(),
            Out => {
                let val = ma.value(args[0])? as u8;
                ::js::output(val);
            }

            // Jumps
            Jmp => ma.set_pc(args[0])?,
            Jt => if 0 != ma.value(args[0])? {
                ma.set_pc(args[1])?;
            },
            Jf => if 0 == ma.value(args[0])? {
                ma.set_pc(args[1])?;
            },

            // Reg/Memory
            Set => ma.set_value(args[0], args[1])?,
            Push => {
                let v = ma.value(args[0])?;
                ma.stack_push(v);
            }
            Pop => {
                let v = ma.stack_pop()?;
                ma.set_value(args[0], v)?;
            }
            Rmem => {
                let val = ma.read_memory(args[1])?;
                ma.set_value(args[0], val)?;
            }
            Wmem => ma.write_memory(args[0], args[1])?,

            // Maths
            Add => arithmetic(|a, b| (a + b) % 32768, args, ma)?,
            Mul => arithmetic(|a, b| ((a as u32 * b as u32) % 32768) as u16, args, ma)?,
            Mod => arithmetic(|a, b| a % b, args, ma)?,
            And => arithmetic(|a, b| a & b, args, ma)?,
            Or => arithmetic(|a, b| a | b, args, ma)?,
            Not => {
                let res = !ma.value(args[1])?;
                // 15 bit
                let res = res & ((1 << 15) - 1);
                ma.set_value(args[0], res)?;
            }

            // Comparison
            Eq => comparison(|a, b| a == b, args, ma)?,
            Gt => comparison(|a, b| a > b, args, ma)?,

            // Functions
            Call => {
                let ret_addr = ma.pc() as u16;
                ma.stack_push(ret_addr);
                ma.set_pc(args[0])?;
            }
            Ret => {
                let ret_addr = ma.stack_pop()?;
                ma.set_pc(ret_addr)?;
            }
        }

        Ok(())
    }
}

fn comparison<F>(f: F, args: &[u16], ma: &mut MachineDetail) -> Result<(), String>
where
    F: Fn(u16, u16) -> bool,
{
    let v1 = ma.value(args[1])?;
    let v2 = ma.value(args[2])?;
    let res = if f(v1, v2) { 1 } else { 0 };
    ma.set_value(args[0], res)?;

    Ok(())
}

fn arithmetic<F>(f: F, args: &[u16], ma: &mut MachineDetail) -> Result<(), String>
where
    F: Fn(u16, u16) -> u16,
{
    let v1 = ma.value(args[1])?;
    let v2 = ma.value(args[2])?;
    let res = f(v1, v2);
    ma.set_value(args[0], res)?;

    Ok(())
}
