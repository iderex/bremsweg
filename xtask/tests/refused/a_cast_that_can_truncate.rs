//! A cast that can lose the top of a value, which is the class this tree denies
//! rather than warns about: it produces a plausible wrong number instead of a
//! crash.
//!
//! Not a member of any target. The fixture beside this directory hands it to
//! the linter with the deny list read out of `Cargo.toml`, so this file is
//! refused because of what that manifest says and not because of a flag written
//! into a test.

pub fn histories(requested: u64) -> u32 {
    requested as u32
}
