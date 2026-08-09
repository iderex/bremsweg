//! The neighbour of `a_cast_that_can_truncate.rs`. Same signature, same
//! question asked of the same two types, and the case where the value does not
//! fit is decided rather than discarded.

pub fn histories(requested: u64) -> u32 {
    u32::try_from(requested).unwrap_or(u32::MAX)
}
