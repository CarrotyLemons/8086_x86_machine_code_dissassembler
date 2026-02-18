use instruction_decoding_on_the_8086::x86_decoder;
use std::fs::{self};
use std::io::{self, ErrorKind, Read};
use std::process::Command;
use tempfile::NamedTempFile;

#[cfg(test)]
fn tool_exists(name: &str) -> Result<bool, io::Error> {
    match Command::new(name).output() {
        Ok(_) => return Ok(true),
        Err(e) => {
            if e.kind() == ErrorKind::NotFound {
                return Ok(false);
            } else {
                return Err(e);
            }
        }
    };
}

fn compare_decompilation(source_binary_name: &str) {
    let source_asm_name = format!("{}.asm", source_binary_name);

    let destination_asm = NamedTempFile::new().unwrap();
    let destination_asm_name = destination_asm.path().to_str().unwrap();

    let destination_binary_name = NamedTempFile::new().unwrap();
    let destination_binary_name = destination_binary_name.path().to_str().unwrap();

    // Create assembled file
    Command::new("nasm").arg(&source_asm_name).output().unwrap();
    let source_file = fs::read(&source_binary_name).unwrap();

    // Run decoding
    x86_decoder::decode_instructions(source_file, destination_asm.reopen().unwrap());

    // Encode output
    Command::new("nasm")
        .args([destination_asm_name, "-o", destination_binary_name])
        .output()
        .unwrap();

    // Compare encoded source and encoded output
    let diff_result = Command::new("diff")
        .args([&source_binary_name, &destination_binary_name])
        .output()
        .unwrap();
    let diff_success = diff_result.status.success();

    if !diff_success {
        let mut buf = String::new();
        destination_asm
            .into_file()
            .read_to_string(&mut buf)
            .unwrap();
        println!("{}", buf);
    }

    // Delete created files
    Command::new("trash")
        .arg(&source_binary_name)
        .output()
        .unwrap();

    assert!(diff_success);
}

mod tests {
    use super::*;

    #[test]
    fn check_tooling() -> io::Result<()> {
        assert!(tool_exists("diff")?);
        assert!(tool_exists("nasm")?);
        assert!(tool_exists("trash")?);
        Ok(())
    }

    #[test]
    fn listing_0037() {
        compare_decompilation("tests/listing_0037_single_register_mov")
    }

    #[test]
    fn listing_0038() {
        compare_decompilation("tests/listing_0038_many_register_mov")
    }
}
