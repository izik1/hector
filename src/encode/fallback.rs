use super::Error;

#[cfg(feature = "alloc")]
use alloc::string::String;
#[cfg(feature = "alloc")]
use alloc::vec::Vec;

#[inline]
fn nibble_to_hex<const UPPER: bool>(nibble: u8) -> u8 {
    debug_assert!(nibble < 0x10);

    let ascii_a = if UPPER { b'A' } else { b'a' };

    let tmp = nibble + b'0';

    tmp + u8::from(tmp > b'9') * (ascii_a - 1 - b'9')
}

#[inline(always)]
pub fn byte_to_hex<const UPPER: bool>(byte: u8) -> (u8, u8) {
    (
        nibble_to_hex::<UPPER>(byte >> 4),
        nibble_to_hex::<UPPER>(byte & 0xf),
    )
}

pub(super) fn encode<const UPPER: bool>(input: &[u8]) -> String {
    let mut output = Vec::with_capacity(2 * input.len());

    for input in input.iter().copied() {
        let (high, low) = byte_to_hex::<UPPER>(input);
        output.push(high);
        output.push(low);
    }

    // safety: for all values of input bytes both output bytes will be valid ascii_hex and therefore make valid strings.
    unsafe { String::from_utf8_unchecked(output) }
}

// todo: const fn when `&mut` in const fn is stable.
pub(super) fn encode_to_slice<'a, const UPPER: bool>(
    input: &[u8],
    output: &'a mut [u8],
) -> Result<&'a mut str, Error> {
    if output.len() != input.len() * 2 {
        return Err(Error);
    }

    // array chunks would be _neat_, but relying on LLVM here is _fine_ (just make sure it code-gens well).
    for (output, input) in output.chunks_exact_mut(2).zip(input.iter().copied()) {
        (output[0], output[1]) = byte_to_hex::<UPPER>(input);
    }

    // safety: for all values of input bytes both output bytes will be valid ascii_hex and therefore make valid strings.
    Ok(unsafe { core::str::from_utf8_unchecked_mut(output) })
}

// pre 1.0: `N * 2` needs to work, so, const-generic exprs.
///
///
/// # Panics
/// Due to limitations in const generics,
/// this function currently panics if `N != 2 * M`,
/// in the future this will turn into a compiler error.
pub(super) fn encode_array<'a, const N: usize, const M: usize, const UPPER: bool>(
    input: &[u8; N],
    output: &'a mut [u8; M],
) -> &'a mut str {
    assert!(N * 2 == M);

    // array chunks would be _neat_, but relying on LLVM here is _fine_ (just make sure it code-gens well).
    for (output, input) in output.chunks_exact_mut(2).zip(input.iter().copied()) {
        (output[0], output[1]) = byte_to_hex::<UPPER>(input);
    }

    // safety: for all values of input bytes both output bytes will be valid ascii_hex and therefore make valid strings.
    unsafe { core::str::from_utf8_unchecked_mut(output) }
}

#[cfg(test)]
mod tests {
    use super::{byte_to_hex, encode, encode_to_slice, nibble_to_hex};

    #[test]
    fn nibble_always_valid() {
        for byte in 0..=0xf {
            assert!(nibble_to_hex::<false>(byte).is_ascii_hexdigit());
            assert!(nibble_to_hex::<true>(byte).is_ascii_hexdigit());
        }
    }

    #[test]
    fn byte_always_valid() {
        for byte in 0..=0xff {
            let (high, low) = byte_to_hex::<false>(byte);
            assert!(high.is_ascii_hexdigit());
            assert!(low.is_ascii_hexdigit());

            let (high, low) = byte_to_hex::<true>(byte);
            assert!(high.is_ascii_hexdigit());
            assert!(low.is_ascii_hexdigit());
        }
    }

    #[test]
    fn encode_all_2_byte_permutations() {
        for v in u16::MIN..=u16::MAX {
            let expected = std::format!("{v:04x}");
            let actual = encode::<false>(&v.to_be_bytes());

            assert_eq!(expected, actual);
        }
    }

    #[test]
    fn encode_slice_matches_encode() {
        for v in u16::MIN..=u16::MAX {
            let expected = encode::<false>(&v.to_be_bytes());
            let mut buf = [0; 4];
            let actual = &*encode_to_slice::<false>(&v.to_be_bytes(), &mut buf).unwrap();

            assert_eq!(expected, actual);
        }
    }
}
