use crate::x86_definitions::*;
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
            Some(instruction) => match instruction {
                Instructions::Move(move_instruction) => {
                    writeln!(
                        destination_file,
                        "mov {}, {}",
                        read_location(move_instruction.destination),
                        read_location(move_instruction.source)
                    )
                    .unwrap()
                }
            },
            None => return,
        };
    }
}
