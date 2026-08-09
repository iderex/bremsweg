//! The neighbour of `a_float_compared_with_an_operator.rs`. The same question
//! about the same two numbers, asked the way `bremsweg_core::close` asks it,
//! which is of the bits.
//!
//! It is written out here rather than calling that function because nothing
//! links this file: the linter is handed it on its own, with no crate to
//! depend on.

pub fn agrees(computed: f64, expected: f64) -> bool {
    computed.to_bits() == expected.to_bits()
}
