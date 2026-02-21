use std::{
    env::args,
    fs::{self},
};

mod errors;
mod tools;
mod x86_decoder;
mod x86_definitions;

fn main() -> () {
    // File handling
    let mut arguments = args();

    let source_file = arguments.nth(1).unwrap();
    let source_file = fs::read(source_file).unwrap();

    let destination_file = arguments.next().unwrap();
    let destination_file = fs::File::create_new(destination_file).unwrap();

    x86_decoder::decode_instructions(source_file, destination_file);
}
