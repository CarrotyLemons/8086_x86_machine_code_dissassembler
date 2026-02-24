pub enum Instructions {
    Move,
    Add,
    Subtract,
    Compare,
}

impl std::fmt::Display for Instructions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let representation = match self {
            Instructions::Move => "mov",
            Instructions::Add => "add",
            Instructions::Subtract => "sub",
            Instructions::Compare => "cmp",
        };

        f.write_str(representation)?;

        Ok(())
    }
}

pub struct Instruction {
    pub instruction: Instructions,
    pub source: Reference,
    pub destination: Reference,
    pub sizing: Option<InstructionSizing>,
}

impl std::fmt::Display for Instruction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.instruction.to_string().as_str())?;
        f.write_str(" ")?;
        match &self.sizing {
            None => {}
            Some(sizing) => {
                if !sizing.is_source {
                    f.write_fmt(format_args!("{} ", sizing))?
                }
            }
        };
        f.write_str(self.destination.to_string().as_str())?;
        f.write_str(", ")?;
        match &self.sizing {
            None => {}
            Some(sizing) => {
                if sizing.is_source {
                    f.write_fmt(format_args!("{} ", sizing))?
                }
            }
        };
        // if self.sizing.is_source {f.write_fmt(format_args!("{} "), self.sizing.is_source)};
        f.write_str(self.source.to_string().as_str())?;

        Ok(())
    }
}

pub struct InstructionSizing {
    pub is_source: bool,
    pub size: InstructionSizingOptions,
}

pub enum InstructionSizingOptions {
    Word,
    Byte,
}

impl std::fmt::Display for InstructionSizing {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let text = match self.size {
            InstructionSizingOptions::Byte => "byte",
            InstructionSizingOptions::Word => "word",
        };

        f.write_str(text)
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
    pub offset: i16,
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
        let representation = match self {
            Register::AL => "al",
            Register::CL => "cl",
            Register::DL => "dl",
            Register::BL => "bl",
            Register::AH => "ah",
            Register::CH => "ch",
            Register::DH => "dh",
            Register::BH => "bh",
            Register::AX => "ax",
            Register::CX => "cx",
            Register::DX => "dx",
            Register::BX => "bx",
            Register::SP => "sp",
            Register::BP => "bp",
            Register::SI => "si",
            Register::DI => "di",
        };

        f.write_str(representation)?;

        Ok(())
    }
}

pub struct Immediate {
    pub value: u16,
}

impl std::fmt::Display for Immediate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.value.to_string().as_str())?;
        Ok(())
    }
}
