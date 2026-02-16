use crate::errors::*;

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

pub enum Location {
    Register(Register),
    Location(u8),
}

pub const BYTE_DATA_REGISTERS: [Register; 8] = [
    Register::AL,
    Register::CL,
    Register::DL,
    Register::BL,
    Register::AH,
    Register::CH,
    Register::DH,
    Register::BH,
];
pub const WORD_DATA_REGISTERS: [Register; 8] = [
    Register::AX,
    Register::CX,
    Register::DX,
    Register::BX,
    Register::SP,
    Register::BP,
    Register::SI,
    Register::DI,
];

pub struct MoveInstruction {
    pub source: Location,
    pub destination: Location,
}

pub enum Instructions {
    Move(MoveInstruction),
}

pub fn read_location(provided_location: Location) -> String {
    match provided_location {
        Location::Location(memory_location) => format!("{:X}", memory_location),
        Location::Register(referenced_register) => match referenced_register {
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
        }
        .to_string(),
    }
}

pub fn extract_instruction(
    machine_code: &mut std::vec::IntoIter<u8>,
) -> DecodeResult<Option<Instructions>> {
    let byte1 = match machine_code.next() {
        Some(value) => value,
        None => return Ok(None),
    };

    ///////////////////////////////////
    // Register/memory to/from register
    if byte1 & 0xFC == 0x88 {
        let byte2 = match machine_code.next() {
            Some(value) => value,
            None => {
                return Err(FailedDecode {
                    bytes: (byte1 as u64),
                });
            }
        };

        let reg = byte2 >> 3 & 0x07;
        let reg_or_mem = byte2 & 0x07;

        ///////////////
        // Handle W bit
        let relevant_register_array = {
            // w = 1, operating on word data
            if byte1 & 0x1 == 0x1 {
                WORD_DATA_REGISTERS
            // w = 0, operating on byte data
            } else {
                BYTE_DATA_REGISTERS
            }
        };

        ///////////////
        // handle D bit
        // d is 1, destination is in reg field
        let decoded_instruction = if byte1 & 0x02 == 0x02 {
            Instructions::Move(MoveInstruction {
                source: Location::Register(relevant_register_array[reg_or_mem as usize]),
                destination: Location::Register(relevant_register_array[reg as usize]),
            })
        // d is 0, source is in reg field
        } else {
            Instructions::Move(MoveInstruction {
                source: Location::Register(relevant_register_array[reg as usize]),
                destination: Location::Register(relevant_register_array[reg_or_mem as usize]),
            })
        };

        /////////////////////////////////////////////////////
        // Handle Mode (currently just discarding those bits)
        let mode = byte2 >> 6;
        // Logic of padding bytes is described in 8086 manual
        if mode == 0x2 {
            machine_code.nth(0);
        } else if (reg_or_mem == 0x6 && mode == 0x0) || (mode == 0x2) {
            machine_code.nth(1);
        }

        return Ok(Some(decoded_instruction));
    }

    Err(FailedDecode {
        bytes: (byte1 as u64),
    })
}
