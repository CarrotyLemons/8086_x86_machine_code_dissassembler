use crate::errors::*;

pub const BYTE_DATA_REGISTERS: [&str; 8] = ["al", "cl", "dl", "bl", "ah", "ch", "dh", "bh"];
pub const WORD_DATA_REGISTERS: [&str; 8] = ["ax", "cx", "dx", "bx", "sp", "bp", "si", "di"];

pub struct MoveInstruction<'a> {
    pub source: &'a str,
    pub destination: &'a str,
}

pub enum Instructions<'a> {
    Move(MoveInstruction<'a>),
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

        // Handle W bit
        let (reg_str, reg_or_mem_str) = if byte1 & 0x1 == 0x1 {
            // w = 1, operating on word data
            (
                WORD_DATA_REGISTERS[reg as usize],
                WORD_DATA_REGISTERS[reg_or_mem as usize],
            )
        } else {
            // w = 0, operating on byte data
            (
                BYTE_DATA_REGISTERS[reg as usize],
                BYTE_DATA_REGISTERS[reg_or_mem as usize],
            )
        };

        // Handle D bit
        let (source, destination) = if byte1 & 0x02 == 0x02 {
            // d is 1, destination is in reg field
            (reg_or_mem_str, reg_str)
        } else {
            // d is 0, source is in reg field
            (reg_str, reg_or_mem_str)
        };

        let decoded_instruction = Instructions::Move(MoveInstruction {
            source,
            destination,
        });

        // Handle Mode (currently just discarding those bits)
        let mode = byte2 >> 6;
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
