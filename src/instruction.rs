use alloc::String;
use helper::u16_to_string;

#[derive(PartialEq, Clone, Copy)]
pub enum Instruction {
    Halt, // 0
    Set,  // 1
    Push, // 2
    Pop,  // 3
    Eq,   // 4
    Gt,   // 5
    Jmp,  // 6
    Jt,   // 7
    Jf,   // 8
    Add,  // 9
    Mul,  // 10
    Mod,  // 11
    And,  // 12
    Or,   // 13
    Not,  // 14
    Rmem, // 15
    Wmem, // 16
    Call, // 17
    Ret,  // 18
    Out,  // 19
    In,   // 20
    Noop, // 21
}

impl Instruction {
    pub fn log_instruction(&self, pc: usize, args: &[u16]) {
        // pc was already incremented, revert for output
        let mut s = u16_to_string((pc - args.len() - 1) as u16);
        s += ": ";
        use self::Instruction::*;
        s += &match *self {
            Halt => String::from("halt"),
            Set => String::from("set"),
            Push => String::from("push"),
            Pop => String::from("pop"),
            Eq => String::from("eq"),
            Gt => String::from("gt"),
            Jmp => String::from("jmp"),
            Jt => String::from("jt"),
            Jf => String::from("jf"),
            Add => String::from("add"),
            Mul => String::from("mul"),
            Mod => String::from("mod"),
            And => String::from("and"),
            Or => String::from("or"),
            Not => String::from("not"),
            Rmem => String::from("rmem"),
            Wmem => String::from("wmem"),
            Call => String::from("call"),
            Ret => String::from("ret"),
            Out => String::from("out"),
            In => String::from("in"),
            Noop => String::from("noop"),
        };

        for a in args {
            s += " ";
            s += &u16_to_string(*a);
        }

        ::js::log(&s);
    }

    pub fn new(instr: u16) -> Result<Instruction, String> {
        use self::Instruction::*;
        match instr {
            0 => Ok(Halt),
            1 => Ok(Set),
            2 => Ok(Push),
            3 => Ok(Pop),
            4 => Ok(Eq),
            5 => Ok(Gt),
            6 => Ok(Jmp),
            7 => Ok(Jt),
            8 => Ok(Jf),
            9 => Ok(Add),
            10 => Ok(Mul),
            11 => Ok(Mod),
            12 => Ok(And),
            13 => Ok(Or),
            14 => Ok(Not),
            15 => Ok(Rmem),
            16 => Ok(Wmem),
            17 => Ok(Call),
            18 => Ok(Ret),
            19 => Ok(Out),
            20 => Ok(In),
            21 => Ok(Noop),
            _ => {
                let mut err = String::from("Unknown instruction: ");
                err += &u16_to_string(instr);
                Err(err)
            }
        }
    }

    pub fn argument_count(&self) -> usize {
        use self::Instruction::*;
        match *self {
            Halt | Noop | Ret => 0,
            Call | In | Jmp | Out | Pop | Push => 1,
            Jf | Jt | Not | Rmem | Set | Wmem => 2,
            Add | And | Eq | Gt | Mod | Mul | Or => 3,
        }
    }
}
