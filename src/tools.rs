use crate::errors::*;

pub fn get_u8_displacement_from_iterator<'a>(
    iterator: &mut std::vec::IntoIter<u8>,
    byte1: u8,
    message: &'a str,
) -> Result<u8, FailedDecode<'a>> {
    match iterator.next() {
        Some(value) => return Ok(value),
        None => {
            return Err(FailedDecode {
                bytes: (byte1),
                message: message,
            });
        }
    };
}

pub fn get_u16_displacement_from_iterator<'a>(
    iterator: &mut std::vec::IntoIter<u8>,
    byte1: u8,
    message: &'a str,
) -> Result<u16, FailedDecode<'a>> {
    let address: u16 = get_u8_displacement_from_iterator(iterator, byte1, message)? as u16;
    let address = ((get_u8_displacement_from_iterator(iterator, byte1, message)? as u16) << 8) + address;
    return Ok(address);
}

#[cfg(test)]
mod tests {
    use crate::tools::get_u16_displacement_from_iterator;

    #[test]
    fn test_get_u16_from_iterator() {
        let input: Vec<u8> = vec![0x0C, 0x03];
        let mut input = input.into_iter();

        assert_eq!(get_u16_displacement_from_iterator(&mut input, 0x00, "").unwrap(), 0x030C as u16)
    }

    #[test]
    fn test_get_u16_from_iterator_2() {
        let input: Vec<u8> = vec![0x87, 0x13];
        let mut input = input.into_iter();

        assert_eq!(get_u16_displacement_from_iterator(&mut input, 0x00, "").unwrap(), 0x1387 as u16)
    }
}