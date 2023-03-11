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

impl core::fmt::Display for Error {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Error::OddLength => f.write_str("input had an odd length"),
            Error::InvalidHex { offset, value } => write!(
                f,
                "character `{char_value}` ({value:#2x}) at `{offset}` is not a valid hex character",
                char_value = *value as char
            ),
            Error::MismatchedLength {
                source_len,
                dest_len,
            } => write!(
                f,
                "source / destination buffer length mismatch: `{source_len} != 2 * {dest_len}`"
            ),
        }
    }
}

#[cfg(feature = "std")]
impl std::error::Error for Error {}

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
    fn inner(input: &[u8]) -> Result<Vec<u8>, Error> {
        if input.len() % 2 != 0 {
            return Err(Error::OddLength);
        }

        validate_hex(input)?;

        let output = input
            .chunks_exact(2)
            .map(|byte| {
                let high = decode_trusted_char(byte[0]);
                let low = decode_trusted_char(byte[1]);

                high << 4 | low
            })
            .collect();

        Ok(output)
    }

    inner(input.as_ref())
}

/// Decodes 4-bits worth of data
///
/// this function assumes that the input is already a hex char.
fn decode_trusted_char(nibble: u8) -> u8 {
    if nibble > b'9' {
        // mask out the "lowercase" bit, subtract the start of the valid range (b'A'), then we still need to add 10.
        (nibble & !0x20) - b'A' + 10
    } else {
        nibble - b'0'
    }
    // todo: profile different versions such as:
    // (nibble & 0xf) + (nibble >> 6) + ((nibble >> 6) << 3)
}

// note: this function is effectively optimal for "errors are common, precise location of error is required"
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
