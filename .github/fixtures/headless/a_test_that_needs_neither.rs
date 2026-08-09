//! The neighbour of the two beside it. It asks a question of the same kind an
//! ordinary test here asks, and it needs no display and no elevation.
//!
//! It is what makes the other two mean anything. An environment that refused
//! everything would refuse those two as well and would have proved nothing
//! about either, so this file has to pass in exactly the environment that
//! refuses them.
//!
//! Not a member of any workspace and nothing builds it with the tree. The
//! workflow beside this directory compiles it with `rustc --test` and requires
//! it to pass.

/// A depth in angstrom and the energy remaining there, which is the shape of
/// the numbers this project computes and needs nothing from the machine to
/// hold.
fn energy_after(depth: f64, stopping: f64, incident: f64) -> f64 {
    incident - stopping * depth
}

#[test]
fn an_ion_loses_energy_along_its_path() {
    let remaining = energy_after(100.0, 5.0, 1000.0);

    assert!(
        remaining < 1000.0,
        "an ion crossing a hundred angstrom kept all of its energy: {remaining}"
    );
}
