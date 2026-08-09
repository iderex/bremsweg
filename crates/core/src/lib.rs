//! The transport physics.
//!
//! Nothing in this crate reads a file, writes a file, opens a socket or touches
//! a terminal. Input and output live in `bremsweg-cli` and `bremsweg-fit`. That
//! boundary is the reason the suite runs with no display, no temporary
//! directory and no elevated privilege, and `tests/dependencies.rs` refuses a
//! dependency being added here that would undo it.
//!
//! The crate holds no physics yet. What is here is a placeholder so that the
//! next change modifies something instead of creating everything.
//!
//! `constants` is here because the physics that will use it is here, and
//! because the rule in `0005` is that one module holds every physical constant
//! and nothing else in the tree defines one. A module in a crate the transport
//! does not depend on would be a module the transport would be tempted to copy
//! from.
//!
//! `close` is here rather than in a crate of its own because this crate's
//! dependency table is declared empty and `tests/dependencies.rs` refuses an
//! entry in it, development entries included. A comparison the suite shares has
//! to be reachable without one, and everything that has tests and floating
//! point numbers already depends on this crate.

pub mod close;
pub mod constants;

/// Doubles `n`, saturating at the maximum rather than wrapping.
///
/// A placeholder. It exists so the crate has a function to modify and a test
/// that can fail, and the first real physics replaces it. Nothing depends on
/// it.
#[must_use]
pub fn saturating_double(n: u64) -> u64 {
    n.saturating_mul(2)
}

#[cfg(test)]
mod tests {
    use super::saturating_double;

    #[test]
    fn doubles_an_ordinary_value() {
        assert_eq!(saturating_double(21), 42);
    }

    #[test]
    fn saturates_instead_of_wrapping() {
        assert_eq!(saturating_double(u64::MAX), u64::MAX);
    }
}
