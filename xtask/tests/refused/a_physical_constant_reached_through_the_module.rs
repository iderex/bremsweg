//! The neighbour of `a_physical_constant_written_out.rs`. The same expression,
//! the same screening constant beside it, and the one number that belongs to
//! the module is reached through the module.
//!
//! Not a member of any target. Nothing links this file, so the constant is
//! named through its full path rather than imported.

pub fn screening_length(atomic_numbers: f64) -> f64 {
    0.8854 * bremsweg_core::constants::BOHR_RADIUS_ANGSTROM / atomic_numbers
}
