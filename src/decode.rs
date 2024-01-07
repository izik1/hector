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

// This is basically the only use for `std` as a crate feature here (likewise with [EncodeError](crate::EncodeError)).
#[cfg(feature = "std")]
impl std::error::Error for Error {}

mod fallback;

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
/// // It works with binary strings.
/// assert_eq!(hector::decode(b"decaff"), Ok(vec![0xde, 0xca, 0xff]));
///
/// // It works with normal strings too.
/// assert_eq!(hector::decode("decaff"), Ok(vec![0xde, 0xca, 0xff]));
///
/// // ... and uppercase
/// assert_eq!(hector::decode("DECAFF"), Ok(vec![0xde, 0xca, 0xff]));
///
/// // Mixed case works just fine as well.
/// assert_eq!(hector::decode("C0Ffee"), Ok(vec![0xc0, 0xff, 0xee]))
/// ```
/// ```
/// use hector::DecodeError;
/// assert_eq!(hector::decode(b"abc"), Err(DecodeError::OddLength));
/// ```
#[cfg(feature = "alloc")]
pub fn decode<T: AsRef<[u8]>>(input: T) -> Result<Vec<u8>, Error> {
    fallback::decode(input.as_ref())
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
/// // It works with binary strings.
/// let mut storage = [0; 3];
/// assert_eq!(hector::decode_to_slice(b"decaff", &mut storage), Ok([0xde, 0xca, 0xff].as_slice()));
///
/// // It works with normal strings too.
/// let mut storage = [0; 3];
/// assert_eq!(hector::decode_to_slice("decaff", &mut storage), Ok([0xde, 0xca, 0xff].as_slice()));
///
/// // ... and uppercase
/// assert_eq!(hector::decode_to_slice("DECAFF", &mut storage), Ok([0xde, 0xca, 0xff].as_slice()));
///
/// // Mixed case works just fine as well.
/// let mut storage = [0; 3];
/// assert_eq!(hector::decode_to_slice("C0Ffee", &mut storage), Ok([0xc0, 0xff, 0xee].as_slice()));
/// ```
///
/// ```
/// let mut storage = [0; 13];
/// let output = hector::decode_to_slice("48656c6c6f2c20776f726c6421", &mut storage);
///
/// assert_eq!(output, Ok(b"Hello, world!".as_slice()));
/// assert_eq!(&storage, b"Hello, world!");
/// ```
///
/// ```
/// use hector::DecodeError;
/// let mut storage = [0; 2];
///
/// let output = hector::decode_to_slice(b"abcde", &mut storage);
/// assert_eq!(output, Err(DecodeError::MismatchedLength { source_len: 5, dest_len: 2 }));
/// ```
pub fn decode_to_slice<T: AsRef<[u8]>>(input: T, output: &mut [u8]) -> Result<&[u8], Error> {
    fallback::decode_to_slice(input.as_ref(), output)
}
