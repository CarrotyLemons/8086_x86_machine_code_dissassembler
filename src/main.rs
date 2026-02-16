use std::{env::args, fs};

use crate::x86_definitions::{Instructions, read_location};

mod errors;
mod x86_definitions;

fn main() -> () {
    let path = args().into_iter().nth(1).unwrap();
    let mut assembled_program = fs::read(path).unwrap().into_iter();

    loop {
        let read_instruction= match x86_definitions::extract_instruction(&mut assembled_program) {
            Ok(value) => value,
            Err(e) => {
                eprintln!("{}", e);
                return;
            }
        };

        match read_instruction {
            Some(instruction) => match instruction {
                Instructions::Move(move_instruction) => {
                    println!(
                        "mov {}, {}",
                        read_location(move_instruction.destination),
                        read_location(move_instruction.source)
                    )
                }
            },
            None => return,
        };
    }
}
