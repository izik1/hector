#[cfg(feature = "alloc")]
use alloc::vec::Vec;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Error {
    /// The source buffer had an odd number of characters.
    OddLength,

    /// The byte at `offset` was not a valid hex character.
    InvalidHex {
        /// The offset into the source buffer that the error occurred at.
        offset: usize,
        /// The character in question.
        value: u8,
    },

    /// The destination buffer size was incorrect for the provided source buffer.
    ///
    /// This happens when decoding into a fixed sized buffer and `source_len != dest_len * 2`
    MismatchedLength {
        /// The length of the source buffer.
        source_len: usize,

        /// The length of the destination buffer.
        dest_len: usize,
    },
}

/// Decode the hex encoded `input`.
///
/// This function does _not_ enforce a specific casing convention.
///
/// # Errors
/// - [`Error::OddLength`] if `input.len()` is not even.
/// - [`Error::InvalidHex`] if any character isn't a valid hex character.
///
/// # Examples
/// ```
/// // it works with binary strings
/// assert_eq!(hector::decode(b"decaff"), Ok(vec![0xde, 0xca, 0xff]));
///
/// // it works with normal strings too
/// assert_eq!(hector::decode("decaff"), Ok(vec![0xde, 0xca, 0xff]));
///
/// // ... and uppercase
/// assert_eq!(hector::decode("DECAFF"), Ok(vec![0xde, 0xca, 0xff]));
///
/// // mixed case works just fine as well
/// assert_eq!(hector::decode("C0Ffee"), Ok(vec![0xc0, 0xff, 0xee]))
/// ```
/// ```
/// use hector::DecodeError;
/// assert_eq!(hector::decode(b"abc"), Err(DecodeError::OddLength));
/// ```
#[cfg(feature = "alloc")]
pub fn decode<T: AsRef<[u8]>>(input: T) -> Result<Vec<u8>, Error> {
    // todo: copy the structure of the encode module.
    // todo: faster impl

    let input = input.as_ref();

    if input.len() % 2 != 0 {
        return Err(Error::OddLength);
    }

    validate_hex(input)?;

    let output = input
        .chunks_exact(2)
        .into_iter()
        .map(|byte| {
            let high = decode_trusted_char(byte[0]);
            let low = decode_trusted_char(byte[1]);

            high << 4 | low
        })
        .collect();

    Ok(output)
}

/// Decodes 1 nibble's worth of data
///
/// this function assumes that the input is already a hex char.
fn decode_trusted_char(nibble: u8) -> u8 {
    if nibble > b'9' {
        // mask out the "lowercase" bit, subtract the start of the valid range (b'A'), then we still need to add 10.
        (nibble & !0x20) - b'A' + 10
    } else {
        nibble - b'0'
    }
}

fn validate_hex(input: &[u8]) -> Result<(), Error> {
    input
        .iter()
        .copied()
        .enumerate()
        .map(|(offset, value)| {
            if value.is_ascii_hexdigit() {
                Ok(())
            } else {
                Err(Error::InvalidHex { offset, value })
            }
        })
        .collect()
}
