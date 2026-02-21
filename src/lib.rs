use crate::errors::*;
use crate::x86_decoder::*;
use crate::x86_definitions::*;

use std::fs::File;
use std::io::Write;

pub mod errors;
pub mod tools;
pub mod x86_decoder;
pub mod x86_definitions;

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

pub fn extract_instruction(
    machine_code: &mut std::vec::IntoIter<u8>,
) -> DecodeResult<Option<Instructions>> {
    let byte1 = match machine_code.next() {
        Some(value) => value,
        None => return Ok(None),
    };

    if byte1 & 0xFC == 0x88 {
        return extract_register_or_memory_to_or_from_register(machine_code, byte1);
    }
    if byte1 & 0xFE == 0xC6 {
        return extract_immediate_to_register_or_memory(machine_code, byte1);
    }
    if byte1 & 0xF0 == 0xB0 {
        return extract_immediate_to_register(machine_code, byte1);
    }
    if byte1 & 0xFC == 0xA0 {
        return extract_accumulator_to_memory_or_memory_to_accumulator(machine_code, byte1);
    }

    Err(FailedDecode {
        bytes: (byte1),
        message: "Byte matched no known opcodes!",
    })
}
