//! A suppression that says a lint is wrong here and does not say why.
//!
//! The near miss this fixture is built around is not somebody disabling a lint
//! across the tree. It is somebody silencing one site, meaning to come back,
//! and leaving nothing a reviewer can weigh or a later reader can retire.

#[allow(clippy::cast_possible_truncation)]
pub fn histories(requested: u64) -> u32 {
    requested as u32
}
