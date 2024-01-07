use super::Error;

/// Decodes 4-bits worth of data
///
/// this function assumes that the input is already a hex char.
fn decode_trusted_char(nibble: u8) -> u8 {
    if nibble > b'9' {
        // Mask out the "lowercase" bit, subtract the start of the valid range (b'A'), then we still need to add 10.
        (nibble & !0x20) - b'A' + 10
    } else {
        nibble - b'0'
    }
    // todo: Profile different versions such as:
    // (nibble & 0xf) + (nibble >> 6) + ((nibble >> 6) << 3)
}

// note: This function is effectively optimal for "errors are common, precise location of error is required"
// better functions could easily exist for "errors are rare" and/or "location of error is not required"
// notably, if errors are rare you could iterate through the list in chunks looking for an error,
// and look again for where that error is, if one is found.
// if the location of the error is not required
fn validate_hex(input: &[u8]) -> Result<(), Error> {
    input.iter().enumerate().try_for_each(|(offset, &value)| {
        if value.is_ascii_hexdigit() {
            Ok(())
        } else {
            Err(Error::InvalidHex { offset, value })
        }
    })
}

// todo: Faster impl
#[cfg(feature = "alloc")]
pub(super) fn decode(input: &[u8]) -> Result<alloc::vec::Vec<u8>, Error> {
    if input.len() % 2 != 0 {
        return Err(Error::OddLength);
    }

    validate_hex(input)?;

    let output = input
        .chunks_exact(2)
        .map(|bytes| decode_trusted_nibbles([bytes[0], bytes[1]]))
        .collect();

    Ok(output)
}

#[inline(always)]
fn decode_trusted_nibbles(nibbles: [u8; 2]) -> u8 {
    let high = decode_trusted_char(nibbles[0]);
    let low = decode_trusted_char(nibbles[1]);

    high << 4 | low
}

pub(super) fn decode_to_slice<'a>(input: &[u8], output: &'a mut [u8]) -> Result<&'a [u8], Error> {
    if input.len() != output.len() * 2 {
        return Err(Error::MismatchedLength {
            source_len: input.len(),
            dest_len: output.len(),
        });
    }

    validate_hex(input)?;

    for (out, nibbles) in output.iter_mut().zip(input.chunks_exact(2)) {
        *out = decode_trusted_nibbles([nibbles[0], nibbles[1]]);
    }

    Ok(output)
}
