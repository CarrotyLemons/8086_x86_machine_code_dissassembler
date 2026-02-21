use crate::x86_definitions::*;
use Register as Reg;

use crate::errors::*;
use crate::tools::*;

use std::fs::File;
use std::io::Write;

// Distinct from x86_definitions content, as this interacts with io but is still x86 specific.
pub fn decode_instructions(source_file: Vec<u8>, mut destination_file: File) {
    let mut source_file = source_file.into_iter();

    writeln!(destination_file, "bits 16").unwrap();

    loop {
        let read_instruction = match extract_instruction(&mut source_file) {
            Ok(value) => value,
            Err(e) => {
                eprintln!("{}", e);
                return;
            }
        };

        match read_instruction {
            None => return,
            Some(instruction) => {
                writeln!(destination_file, "{}", instruction).unwrap();
            }
        }
    }
}

fn get_standard_register_encoding(bits: u8, is_word_encoding: bool) -> Reference {
    const BYTE_DATA_REGISTERS: [Reg; 8] = [
        Reg::AL,
        Reg::CL,
        Reg::DL,
        Reg::BL,
        Reg::AH,
        Reg::CH,
        Reg::DH,
        Reg::BH,
    ];
    const WORD_DATA_REGISTERS: [Reg; 8] = [
        Reg::AX,
        Reg::CX,
        Reg::DX,
        Reg::BX,
        Reg::SP,
        Reg::BP,
        Reg::SI,
        Reg::DI,
    ];

    let register = if is_word_encoding {
        WORD_DATA_REGISTERS[bits as usize]
    } else {
        BYTE_DATA_REGISTERS[bits as usize]
    };

    return Reference::Reg(register);
}

fn get_expression_register_encoding(bits: u8, offset: u16) -> Reference {
    const DATA_REGISTERS: [[Option<Reg>; 2]; 8] = [
        [Some(Reg::BX), Some(Reg::SI)],
        [Some(Reg::BX), Some(Reg::DI)],
        [Some(Reg::BP), Some(Reg::SI)],
        [Some(Reg::BP), Some(Reg::DI)],
        [Some(Reg::SI), None],
        [Some(Reg::DI), None],
        [Some(Reg::BP), None],
        [Some(Reg::BX), None],
    ];

    return Reference::Mem(Memory {
        registers: DATA_REGISTERS[bits as usize],
        offset,
    });
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
        let byte2 = get_u8_displacement_from_iterator(machine_code, byte1, "missing second byte!")?;

        let reg_bits = byte2 >> 3 & 0x07;
        let reg_or_mem_bits = byte2 & 0x07;
        let is_word_encoding = byte1 & 0x1 == 0x1;
        let mode = byte2 >> 6;

        let reg;
        let reg_or_mem;
        // Traditional encoding register to register move
        if mode == 0x3 {
            reg = get_standard_register_encoding(reg_bits, is_word_encoding);
            reg_or_mem = get_standard_register_encoding(reg_or_mem_bits, is_word_encoding);
        // Direct addressing
        } else if mode == 0x0 && reg_or_mem_bits == 0x6 {
            let address: u16 =
                get_u16_displacement_from_iterator(machine_code, byte1, "Missing bytes for direct address")?;

            reg = get_standard_register_encoding(reg_bits, is_word_encoding);
            reg_or_mem = Reference::Mem(Memory {
                registers: [None, None],
                offset: address,
            });
        // Expression encoding for reg_or_mem with no constant
        } else if mode == 0x0 {
            reg = get_standard_register_encoding(reg_bits, is_word_encoding);
            reg_or_mem = get_expression_register_encoding(reg_or_mem_bits, 0);
        // Expression encoding for reg_or_mem with byte constant added
        } else if mode == 0x1 {
            let offset =
                get_u8_displacement_from_iterator(machine_code, byte1, "Missing byte for u8 offset!")? as u16;

            reg = get_standard_register_encoding(reg_bits, is_word_encoding);
            reg_or_mem = get_expression_register_encoding(reg_or_mem_bits, offset);
        // Expression encoding for reg_or_mem with two byte constant added
        } else {
            let offset =
                get_u16_displacement_from_iterator(machine_code, byte1, "Missing bytes for u16 offset!")? as u16;

            reg = get_standard_register_encoding(reg_bits, is_word_encoding);
            reg_or_mem = get_expression_register_encoding(reg_or_mem_bits, offset);
        }
        // Handle D bit
        let (source, destination) = if byte1 & 0x02 == 0x02 {
            // d is 1, destination is in reg field
            (reg_or_mem, reg)
        } else {
            // d is 0, source is in reg field
            (reg, reg_or_mem)
        };

        return Ok(Some(Instructions::Move(MoveInstruction {
            source,
            destination,
        })));
    }

    Err(FailedDecode {
        bytes: (byte1),
        message: "Byte matched no known opcodes!",
    })
}
