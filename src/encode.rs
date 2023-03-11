/// An error occured while encoding.
///
/// Currently the only error that can occur while encoding is an output size mismatch when encoding to a slice.
#[derive(Debug, Eq, PartialEq)]
pub struct Error;

impl core::fmt::Display for Error {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str("encode error: Wrong output size for input")
    }
}

// this is basically the only use for `std` as a crate feature here (likewise with [DecodeError](crate::DecodeError)).
#[cfg(feature = "std")]
impl std::error::Error for Error {}

mod fallback;

/// Encode `input` to a lowercase hex string.
///
/// # Examples
/// ```
/// assert_eq!(hector::encode([0xde, 0xca, 0xff]), "decaff");
/// ```
///
/// ```
/// assert_eq!(hector::encode("Hello, world!"), "48656c6c6f2c20776f726c6421");
/// ```
#[cfg(feature = "alloc")]
#[must_use]
pub fn encode<T: AsRef<[u8]>>(input: T) -> alloc::string::String {
    fallback::encode::<false>(input.as_ref())
}

/// Encode `input` to an uppercase hex string.
///
/// # Examples
/// ```
/// assert_eq!(hector::encode_upper([0xde, 0xca, 0xff]), "DECAFF");
/// ```
///
/// ```
/// assert_eq!(hector::encode_upper("Hello, world!"), "48656C6C6F2C20776F726C6421");
/// ```
#[cfg(feature = "alloc")]
#[must_use]
pub fn encode_upper<T: AsRef<[u8]>>(input: T) -> alloc::string::String {
    fallback::encode::<true>(input.as_ref())
}

/// Encode `input` to a lowercase hex string.
///
/// For convenience, this returns a [`&str`](str), backed by `output`.
///
/// # Examples
/// ```
//
/// let mut storage = [0; 6];
///
/// assert_eq!(hector::encode_to_slice([0xde, 0xca, 0xff], &mut storage), Ok("decaff"));
/// assert_eq!(&storage, b"decaff");
/// ```
///
/// ```
/// let mut storage = [0; 26];
/// let output = hector::encode_to_slice("Hello, world!", &mut storage);
///
/// assert_eq!(output, Ok("48656c6c6f2c20776f726c6421"));
/// assert_eq!(&storage, b"48656c6c6f2c20776f726c6421");
/// ```
///
/// # Errors
/// - [`EncodeError`] if the output is too big or too small.
pub fn encode_to_slice<T: AsRef<[u8]>>(input: T, output: &mut [u8]) -> Result<&str, Error> {
    fallback::encode_to_slice::<false>(input.as_ref(), output)
}

/// Encode `input` to an uppercase hex string.
///
/// For convenience, this returns a [`&str`](str), backed by `output`.
///
/// # Examples
/// ```
//
/// let mut storage = [0; 6];
///
/// assert_eq!(hector::encode_to_slice_upper([0xde, 0xca, 0xff], &mut storage), Ok("DECAFF"));
/// assert_eq!(&storage, b"DECAFF");
/// ```
///
/// ```
/// let mut storage = [0; 26];
/// let output = hector::encode_to_slice_upper("Hello, world!", &mut storage);
///
/// assert_eq!(output, Ok("48656C6C6F2C20776F726C6421"));
/// assert_eq!(&storage, b"48656C6C6F2C20776F726C6421");
/// ```
///
/// # Errors
/// - [`EncodeError`] if the output is too big or too small.
pub fn encode_to_slice_upper<T: AsRef<[u8]>>(input: T, output: &mut [u8]) -> Result<&str, Error> {
    fallback::encode_to_slice::<true>(input.as_ref(), output)
}

/// Encode `input` to a hex string.
///
/// For convenience, this returns a  [`&str`](str), backed by `output`.
///
/// # Examples
/// ```
//
/// let mut storage = [0; 6];
///
/// assert_eq!(hector::encode_to_array(&[0xde, 0xca, 0xff], &mut storage), "decaff");
/// assert_eq!(&storage, b"decaff");
/// ```
///
/// ```
/// let mut storage = [0; 26];
///
/// assert_eq!(hector::encode_to_array(b"Hello, world!", &mut storage), "48656c6c6f2c20776f726c6421");
/// assert_eq!(&storage, b"48656c6c6f2c20776f726c6421");
/// ```
///
/// # Panics
/// Due to limitations in const generics,
/// this function currently panics if `N != 2 * M`,
/// once `const_generic_exprs` is stable, `M` will be removed, and this will become a compiler error.
pub fn encode_to_array<'a, const N: usize, const M: usize>(
    input: &[u8; N],
    output: &'a mut [u8; M],
) -> &'a str {
    fallback::encode_array::<N, M, false>(input, output)
}

/// Encode `input` to an uppercase hex string.
///
/// For convenience, this returns a `&str`, backed by `output`.
///
/// # Examples
/// ```
//
/// let mut storage = [0; 6];
///
/// assert_eq!(hector::encode_to_array_upper(&[0xde, 0xca, 0xff], &mut storage), "DECAFF");
/// assert_eq!(&storage, b"DECAFF");
/// ```
///
/// ```
/// let mut storage = [0; 26];
///
/// assert_eq!(hector::encode_to_array_upper(b"Hello, world!", &mut storage), "48656C6C6F2C20776F726C6421");
/// assert_eq!(&storage, b"48656C6C6F2C20776F726C6421");
/// ```
///
/// # Panics
/// Due to limitations in const generics,
/// this function currently panics if `N != 2 * M`,
/// once `const_generic_exprs` is stable, `M` will be removed, and this will become a compiler error.
pub fn encode_to_array_upper<'a, const N: usize, const M: usize>(
    input: &[u8; N],
    output: &'a mut [u8; M],
) -> &'a str {
    fallback::encode_array::<N, M, true>(input, output)
}
