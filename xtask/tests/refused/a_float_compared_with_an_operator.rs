//! Two floating point numbers compared with an operator, which is the spelling
//! the linter does see.
//!
//! Not a member of any target. The fixture beside this directory hands it to
//! the linter with the deny list read out of `Cargo.toml`.

pub fn agrees(computed: f64, expected: f64) -> bool {
    computed == expected
}
