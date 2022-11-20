#[derive(Debug)]
pub struct Error;

mod fallback;

// note: conversion from a `String` to a `Vec<u8>` is free,
// and we only ever output the characters `0..=9, a..=f, A..=F`,
// so we can (unsafely, yet soundly) assume that the string is valid.
/// Encode `input` to a hex string.
///
/// # Examples
/// ```
/// assert_eq!(hector::encode([0xde, 0xca, 0xff]), "decaff");
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
#[cfg(feature = "alloc")]
#[must_use]
pub fn encode_upper<T: AsRef<[u8]>>(input: T) -> alloc::string::String {
    fallback::encode::<true>(input.as_ref())
}

/// Encode `input` to a hex string.
///
/// # Errors
/// - [`EncodeError`] if the output is too big or too small.
pub fn encode_to_slice<T: AsRef<[u8]>>(input: T, output: &mut [u8]) -> Result<&mut str, Error> {
    fallback::encode_to_slice::<false>(input.as_ref(), output)
}

/// Encode `input` to an uppercase hex string.
///
/// # Errors
/// - [`EncodeError`] if the output is too big or too small.
pub fn encode_to_slice_upper<T: AsRef<[u8]>>(
    input: T,
    output: &mut [u8],
) -> Result<&mut str, Error> {
    fallback::encode_to_slice::<false>(input.as_ref(), output)
}

/// Encode `input` to a hex string.
///
/// # Panics
/// Due to limitations in const generics,
/// this function currently panics if `N != 2 * M`,
/// in the future this will turn into a compiler error.
pub fn encode_to_array<'a, const N: usize, const M: usize>(
    input: &[u8; N],
    output: &'a mut [u8; M],
) -> &'a mut str {
    fallback::encode_array::<N, M, false>(input, output)
}

/// Encode `input` to an uppercase hex string.
///
/// # Panics
/// Due to limitations in const generics,
/// this function currently panics if `N != 2 * M`,
/// in the future this will turn into a compiler error.
pub fn encode_to_array_upper<'a, const N: usize, const M: usize>(
    input: &[u8; N],
    output: &'a mut [u8; M],
) -> &'a mut str {
    fallback::encode_array::<N, M, true>(input, output)
}
