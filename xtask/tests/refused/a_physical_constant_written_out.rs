//! A physical constant typed into an expression instead of taken from the one
//! module that holds it.
//!
//! The near miss this fixture is built around is not a second definition of the
//! Bohr radius, which a reader would see and argue with. It is the rounded copy
//! in the middle of an expression, written because reaching for the module was
//! inconvenient. Nothing about the number looks wrong, it agrees with the
//! module to every digit it shows, and it goes on saying what it says on the
//! day the module is revised.
//!
//! The other number on the same line is the screening constant, which is a
//! fitted parameter of a particular screening function and not a physical
//! constant. It carries as many digits and it is not in the module, so a check
//! that refused this file for two numbers rather than one would be refusing
//! digits rather than constants.
//!
//! Not a member of any target. The fixture beside this directory hands the text
//! of this file to the check and expects one refusal, naming this line.

pub fn screening_length(atomic_numbers: f64) -> f64 {
    0.8854 * 0.5292 / atomic_numbers
}
