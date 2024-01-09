use core::mem::MaybeUninit;

use super::Error;

#[cfg(feature = "alloc")]
use alloc::{string::String, vec::Vec};

/// A single character in hexidecimal.
///
/// The valid range of a hex character is ascii-`0-9`, ascii-`a-f`, and ascii-`A-F`.
type HexChar = u8;

/// Convert a 4-bit number to a single hex character.
#[inline]
const fn nibble_to_hex<const UPPER: bool>(nibble: u8) -> HexChar {
    debug_assert!(nibble < 0x10);

    let ascii_a = if UPPER { b'A' } else { b'a' };

    // Because of the way ascii is structured,
    // the conversion for numbers from 0-9 is simple: just start with ascii-`0` and add `nibble`.
    let digit = nibble + b'0';

    // However, for numbers from 0xa-0xf, we need to do a bit of extra work,
    // because we'll currently have a character that's one of: `:;<=>?`, which are... Not what we're looking for.
    // So, we need to move from the `0-9` range of ascii to the a-f (or A-F) range, which can be done by adding in the following adjustment:
    let adjustment = ascii_a - 1 - b'9';

    // This is written as a multiplication by a bool instead of an if/else because the compiler gets confused when there's conditionals,
    // and it doesn't output code that's quite as good.
    // Likewise, we compare `digit` to ascii-9 instead of `tmp` to the number 9 because the compiler generates slightly better code.
    digit + (digit > b'9') as u8 * adjustment
}

/// Convert a byte of input data to two hex characters.
///
/// If `UPPER` then this will return uppercase hex characters,
/// otherwise, lowercase hex is used instead.
///
/// This method is the common primitive of all the hex encoding functions in this module.
#[inline(always)]
const fn byte_to_hex<const UPPER: bool>(byte: u8) -> [HexChar; 2] {
    [
        nibble_to_hex::<UPPER>(byte >> 4),
        nibble_to_hex::<UPPER>(byte & 0xf),
    ]
}

#[cfg(feature = "alloc")]
pub(super) fn encode<const UPPER: bool>(input: &[u8]) -> String {
    // This solution was chosen out of the following 4:
    // 1. `Vec::with_capacity` + `Vec::push`. This ended up emitting a resize-check for `push`,
    //    which prevents autovectorization.
    // 2. `flat_map` + `collect`. Pays for an unneeded assert to ensure we don't overflow a `usize`
    //    (even though `collect` does a more restrictive check just a bit later).
    // 3. `vec![0; {len}]` + `encode_to_slice`. This would require emitting a `rust_allocate_zeroed` instead of a `rust_allocate`.
    // 4. (this solution) `Vec::with_capacity` + `Vec::spare_capacity_mut`.
    //    This requires extra unsafe code...
    let out_len = input.len() * 2;

    let mut output = Vec::with_capacity(out_len);
    unsafe {
        // vec only guarantees we get "at least" `out_len` capacity, we might have a bit extra.
        encode_impl::<UPPER>(input, &mut output.spare_capacity_mut()[..out_len]);
        // safety: `encode_inner` guarantees that `output.len()` elements are written.
        output.set_len(out_len);
    }

    // Safety: for all values of input bytes both output bytes will be valid ascii-hex (as asserted by tests for `byte_to_hex`).
    // Ascii hex characters are valid UTF-8 (because ascii is valid UTF-8).
    // Therefore, this is a valid conversion.
    unsafe { String::from_utf8_unchecked(output) }
}

// note: There *is* a way to deduplicate this with the slice/array impls, but honestly, it just isn't worth it with the current stdlib.
// coincidentally, this function existing means we *could* expose a write once method.
// For instance if someone needs to do an in-place init of an array/slice.
///
/// # Safety
/// `output` *must* have exactly the right length for `input`.
///
/// It is safe to assume that all of `output` is initialized after this function returns normally.
///
/// This function will *never* write uninitialized values.
#[inline(always)]
fn encode_impl<const UPPER: bool>(input: &[u8], output: &mut [MaybeUninit<u8>]) {
    // array chunks would be _neat_, but relying on LLVM here is _fine_ (just make sure it code-gens well).
    for (output, input) in output.chunks_exact_mut(2).zip(input.iter().copied()) {
        let byte = byte_to_hex::<UPPER>(input);
        [output[0], output[1]] = [MaybeUninit::new(byte[0]), MaybeUninit::new(byte[1])];
    }
}

// todo: const fn when `&mut` in const fn is stable... And everything else.
pub(super) fn encode_to_slice<'a, const UPPER: bool>(
    input: &[u8],
    output: &'a mut [u8],
) -> Result<&'a str, Error> {
    if output.len() != input.len() * 2 {
        return Err(Error);
    }

    // array chunks would be _neat_, but relying on LLVM here is _fine_ (just make sure it code-gens well).
    for (output, input) in output.chunks_exact_mut(2).zip(input.iter().copied()) {
        [output[0], output[1]] = byte_to_hex::<UPPER>(input);
    }

    // Safety: for all values of input bytes both output bytes will be valid ascii-hex (as asserted by tests for `byte_to_hex`).
    // Ascii hex characters are valid UTF-8 (because ascii is valid UTF-8).
    // Therefore, this is a valid conversion.
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
) -> &'a str {
    assert!(N * 2 == M);

    // Array chunks would be _neat_, but relying on LLVM here is _fine_ (just make sure it code-gens well).
    for (output, input) in output.chunks_exact_mut(2).zip(input.iter().copied()) {
        [output[0], output[1]] = byte_to_hex::<UPPER>(input);
    }

    // Safety: for all values of input bytes both output bytes will be valid ascii-hex (as asserted by tests for `byte_to_hex`).
    // Ascii hex characters are valid UTF-8 (because ascii is valid UTF-8).
    // Therefore, this is a valid conversion.
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
            let [high, low] = byte_to_hex::<false>(byte);
            assert!(high.is_ascii_hexdigit());
            assert!(low.is_ascii_hexdigit());

            let [high, low] = byte_to_hex::<true>(byte);
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
