use Register as Reg;

pub enum Instructions {
    Move(MoveInstruction),
}

impl std::fmt::Display for Instructions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Instructions::Move(instruction) => f.write_str(instruction.to_string().as_str()),
        }
    }
}

pub struct MoveInstruction {
    pub source: Reference,
    pub destination: Reference,
}

impl std::fmt::Display for MoveInstruction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!(
            "mov {}, {}",
            self.destination.to_string(),
            self.source.to_string()
        ))
    }
}

pub enum Reference {
    Reg(Register),
    Mem(Memory),
    Imm(Immediate),
}

impl std::fmt::Display for Reference {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Reference::Reg(register) => f.write_str(register.to_string().as_str()),
            Reference::Mem(memory) => f.write_str(memory.to_string().as_str()),
            Reference::Imm(immediate) => f.write_str(immediate.to_string().as_str()),
        }
    }
}

pub struct Memory {
    pub registers: [Option<Register>; 2],
    pub offset: u16,
}

impl std::fmt::Display for Memory {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("[")?;

        match self.registers[0] {
            None => (),
            Some(register) => {
                f.write_str(register.to_string().as_str())?;
                f.write_str(" + ")?;
            }
        };
        match self.registers[1] {
            None => (),
            Some(register) => {
                f.write_str(register.to_string().as_str())?;
                f.write_str(" + ")?;
            }
        };

        f.write_str(self.offset.to_string().as_str())?;

        f.write_str("]")?;

        Ok(())
    }
}

#[derive(Clone, Copy)]
pub enum Register {
    AL,
    CL,
    DL,
    BL,
    AH,
    CH,
    DH,
    BH,
    AX,
    CX,
    DX,
    BX,
    SP,
    BP,
    SI,
    DI,
}

impl std::fmt::Display for Register {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let representation = match *self {
            Reg::AL => "al",
            Reg::CL => "cl",
            Reg::DL => "dl",
            Reg::BL => "bl",
            Reg::AH => "ah",
            Reg::CH => "ch",
            Reg::DH => "dh",
            Reg::BH => "bh",
            Reg::AX => "ax",
            Reg::CX => "cx",
            Reg::DX => "dx",
            Reg::BX => "bx",
            Reg::SP => "sp",
            Reg::BP => "bp",
            Reg::SI => "si",
            Reg::DI => "di",
        };

        f.write_str(representation)?;

        Ok(())
    }
}

pub struct Immediate {
    pub value: u16,
    pub is_word_encoding: bool,
}

impl std::fmt::Display for Immediate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.is_word_encoding {
            f.write_str("word ")?;
        } else {
            f.write_str("byte ")?;
        }
        f.write_str(self.value.to_string().as_str())?;
        Ok(())
    }
}
