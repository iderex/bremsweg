//! Deliberately misformatted, and nothing else about it is unusual.
//!
//! Not a member of any target, so nothing compiles it and `cargo fmt --all`
//! never reaches it. The only thing that reads it is the fixture beside this
//! directory, which asks the formatter about it and expects a refusal.

pub fn stopping_power( energy:f64 )->f64{
        energy
}
