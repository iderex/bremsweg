//! CRC-32, as the zip format uses it.
//!
//! It is here because it is what turns a decompressor's mistake into a
//! refusal. A wrong bit out of an inflate is not an error, it is a number, and
//! a stopping power that is wrong in its third digit passes every other check
//! in this tree. The archive records a checksum and a length per member, so the
//! bytes can be judged against what the publisher said they would be rather
//! than against nothing.
//!
//! The polynomial and the reflected form are the ones the zip specification
//! fixes, so this is an implementation of somebody else's constant rather than
//! a choice made here.

/// The reflected form of the CRC-32 polynomial the zip format uses.
const POLYNOMIAL: u32 = 0xEDB8_8320;

/// The CRC-32 of `bytes`, as a zip member header records it.
#[must_use]
pub fn of(bytes: &[u8]) -> u32 {
    let mut crc = u32::MAX;
    for byte in bytes {
        crc ^= u32::from(*byte);
        for _ in 0..8 {
            // Shifting right cannot overflow and the branch is the whole of the
            // algorithm, so there is nothing here to check for.
            let carry = crc & 1;
            crc >>= 1;
            if carry == 1 {
                crc ^= POLYNOMIAL;
            }
        }
    }
    crc ^ u32::MAX
}

#[cfg(test)]
mod tests {
    use super::of;

    /// The value in the specification's own worked example. Anything else here
    /// would be this implementation checked against itself.
    #[test]
    fn the_published_check_value() {
        assert_eq!(of(b"123456789"), 0xCBF4_3926);
    }

    #[test]
    fn nothing_hashes_to_zero() {
        assert_eq!(of(b""), 0);
    }

    #[test]
    fn one_bit_changes_the_value() {
        assert_ne!(of(b"123456789"), of(b"123456780"));
    }
}
