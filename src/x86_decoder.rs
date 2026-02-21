use crate::errors::*;
use crate::tools::*;
use crate::x86_definitions::*;

use Register as Reg;

pub fn get_standard_register_encoding(bits: u8, is_word_encoding: bool) -> Reference {
    // The sequence of bits that correspond to which register is hard coded for when referring directly to registers
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

pub fn get_expression_register_encoding(bits: u8, offset: i16) -> Reference {
    // When referring to expressions for addressing memory, this table describes which bits correspond to the addition of which registers
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

pub fn extract_register_or_memory_to_or_from_register(
    machine_code: &mut std::vec::IntoIter<u8>,
    byte1: u8,
) -> DecodeResult<Option<Instructions>> {
    let byte2 = get_u8_displacement_from_iterator(machine_code, byte1, "missing second byte!")?;

    let reg_bits = byte2 >> 3 & 0x07;
    let reg_or_mem_bits = byte2 & 0x07;
    let is_word_encoding = byte1 & 0x1 == 0x1;
    let mode = byte2 >> 6;

    let reg;
    let reg_or_mem;
    // Register mode
    if mode == 0x3 {
        reg = get_standard_register_encoding(reg_bits, is_word_encoding);
        reg_or_mem = get_standard_register_encoding(reg_or_mem_bits, is_word_encoding);
    // Addressing memory from offset
    } else if mode == 0x0 && reg_or_mem_bits == 0x6 {
        let address = get_u16_displacement_from_iterator(
            machine_code,
            byte1,
            "Missing bytes for direct address",
        )? as i16;

        reg = get_standard_register_encoding(reg_bits, is_word_encoding);
        reg_or_mem = Reference::Mem(Memory {
            registers: [None, None],
            offset: address,
        });
    // Addressing memory from register and no offset
    } else if mode == 0x0 {
        reg = get_standard_register_encoding(reg_bits, is_word_encoding);
        reg_or_mem = get_expression_register_encoding(reg_or_mem_bits, 0);
    // Addressing memory from register and byte offset
    } else if mode == 0x1 {
        let offset =
            get_u8_displacement_from_iterator(machine_code, byte1, "Missing byte for u8 offset!")?
                as i8;

        reg = get_standard_register_encoding(reg_bits, is_word_encoding);
        reg_or_mem = get_expression_register_encoding(reg_or_mem_bits, offset.into());
    // Addressing memory from register with two byte offset
    } else {
        let offset = get_u16_displacement_from_iterator(
            machine_code,
            byte1,
            "Missing bytes for u16 offset!",
        )? as i16;

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

pub fn extract_immediate_to_register_or_memory(
    machine_code: &mut std::vec::IntoIter<u8>,
    byte1: u8,
) -> DecodeResult<Option<Instructions>> {
    let byte2 = get_u8_displacement_from_iterator(machine_code, byte1, "missing second byte!")?;
    let reg_or_mem_bits = byte2 & 0x07;
    let is_word_encoding = byte1 & 0x1 == 0x1;
    let mode = byte2 >> 6;

    let reg_or_mem;

    // Register mode
    if mode == 0x3 {
        reg_or_mem = get_standard_register_encoding(reg_or_mem_bits, is_word_encoding);
    // Addressing memory from offset
    } else if mode == 0x0 && reg_or_mem_bits == 0x6 {
        let address = get_u16_displacement_from_iterator(
            machine_code,
            byte1,
            "Missing bytes for direct address",
        )? as i16;

        reg_or_mem = Reference::Mem(Memory {
            registers: [None, None],
            offset: address,
        });
    // Addressing memory from register and no offset
    } else if mode == 0x0 {
        reg_or_mem = get_expression_register_encoding(reg_or_mem_bits, 0);
    // Addressing memory from register and byte offset
    } else if mode == 0x1 {
        let offset =
            get_u8_displacement_from_iterator(machine_code, byte1, "Missing byte for u8 offset!")?
                as i8;

        reg_or_mem = get_expression_register_encoding(reg_or_mem_bits, offset.into());
    // Addressing memory from register with two byte offset
    } else {
        let offset = get_u16_displacement_from_iterator(
            machine_code,
            byte1,
            "Missing bytes for u16 offset!",
        )? as i16;

        reg_or_mem = get_expression_register_encoding(reg_or_mem_bits, offset);
    }

    // Getting the immediate
    let immediate;
    if is_word_encoding {
        immediate = get_u16_displacement_from_iterator(
            machine_code,
            byte1,
            "Missing bytes for u16 immediate!",
        )?;
    } else {
        immediate = get_u8_displacement_from_iterator(
            machine_code,
            byte1,
            "Missing bytes for u16 immediate!",
        )? as u16;
    }
    let immediate = Reference::Imm(Immediate {
        value: immediate,
        is_word_encoding,
    });

    return Ok(Some(Instructions::Move(MoveInstruction {
        source: immediate,
        destination: reg_or_mem,
    })));
}

pub fn extract_immediate_to_register(
    machine_code: &mut std::vec::IntoIter<u8>,
    byte1: u8,
) -> DecodeResult<Option<Instructions>> {
    let reg_bits = byte1 & 0x07;
    let is_word_encoding = byte1 & 0x8 == 0x8;

    let reg = get_standard_register_encoding(reg_bits, is_word_encoding);

    // Getting the immediate
    let immediate;
    if is_word_encoding {
        immediate = get_u16_displacement_from_iterator(
            machine_code,
            byte1,
            "Missing bytes for u16 immediate!",
        )?;
    } else {
        immediate = get_u8_displacement_from_iterator(
            machine_code,
            byte1,
            "Missing bytes for u16 immediate!",
        )? as u16;
    }
    let immediate = Reference::Imm(Immediate {
        value: immediate,
        is_word_encoding,
    });

    return Ok(Some(Instructions::Move(MoveInstruction {
        source: immediate,
        destination: reg,
    })));
}

pub fn extract_accumulator_to_memory_or_memory_to_accumulator(
    machine_code: &mut std::vec::IntoIter<u8>,
    byte1: u8,
) -> DecodeResult<Option<Instructions>> {
    let address = get_u16_displacement_from_iterator(
        machine_code,
        byte1,
        "Missing bytes for direct address",
    )? as i16;

    let address = Reference::Mem(Memory {
        registers: [None, None],
        offset: address,
    });

    let is_word_encoding = byte1 & 0x1 == 0x1;
    let destination = if is_word_encoding {
        Reference::Reg(Register::AX)
    } else {
        Reference::Reg(Register::AL)
    };

    let is_accumulator_to_memory = byte1 & 0x02 == 0x02;

    let (source, destination) = if is_accumulator_to_memory {
        (destination, address)
    } else {
        (address, destination)
    };

    return Ok(Some(Instructions::Move(MoveInstruction {
        source,
        destination,
    })));
}
