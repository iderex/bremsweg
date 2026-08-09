//! Every physical constant this program uses.
//!
//! One module, and nothing else in the tree defines one. `0005` fixes that rule
//! and says what failure it prevents: not a wrong digit here, which a reader
//! would find, but the second copy, a factor typed into an expression because
//! reaching for this module was inconvenient and then never revised when this
//! module is. `xtask::physical_constants` is the leg that refuses the second
//! copy.
//!
//! Each constant carries the source it was taken from and the revision of that
//! source, as a `Source:` line and a `Revision:` line the leg requires. For a
//! fundamental constant the revision is the CODATA adjustment year rather than
//! the year somebody read it off a page.
//!
//! Two constants that must be consistent with each other are held as the
//! product they appear as, which `0005` requires so that no expression can be
//! assembled from two different adjustments. The Coulomb product below is that
//! case: a charge and a permittivity that never appear apart.
//!
//! The values are in this project's internal units, which `0005` fixes: energy
//! in electronvolts, length in angstrom. A constant is stored in the unit it is
//! used in, so the site that uses it carries no conversion factor, which is
//! where this error class lives.
//!
//! WHAT IS HERE IS WHAT THE DECISIONS SO FAR REQUIRE, and it is a small set
//! because the physics has not landed. Each of the three is named by a record
//! that is already accepted, and a constant nothing accepted asks for is not
//! added in advance. The set grows with the physics; what the leg holds is that
//! it grows here.

/// The Coulomb energy of two unit charges one angstrom apart, in electronvolt
/// angstrom.
///
/// Held as the product because a charge and a permittivity appear here only as
/// `e^2 / (4 pi eps0)` and never apart, so no expression in this program can be
/// built from two different adjustments of them. In this unit, dividing by a
/// separation in angstrom gives an energy in electronvolts directly, and the
/// screened Coulomb potential the scattering kernel is written from needs no
/// conversion factor.
///
/// Both inputs are exact by definition rather than measured, so this product is
/// exact to the width of the type and carries no uncertainty. The elementary
/// charge is fixed by the 2019 revision of the SI, and `1 / (4 pi eps0)` is
/// `c^2 * 1e-7` with the speed of light fixed by the same revision. Derived
/// rather than read off a table:
///
/// ```text
/// awk 'BEGIN{ e=1.602176634e-19; ke=299792458.0^2*1e-7; printf "%.17g\n", e*ke*1e10 }'
/// 14.399645470586226
/// ```
///
/// Source: CODATA recommended values of the fundamental physical constants, elementary charge and speed of light in vacuum, both exact
/// Revision: CODATA 2022
pub const COULOMB_PRODUCT_EV_ANGSTROM: f64 = 14.399_645_470_586_226;

/// The Bohr radius, in angstrom.
///
/// The length the universal screening function is written in, so it enters
/// every scattering integral through the screening length rather than through
/// an atomic radius of somebody's choosing.
///
/// Measured rather than defined. The standard uncertainty is 82 in the last two
/// digits given, which is eleven orders of magnitude below the disagreement
/// between the stopping measurements this project is fitted against, so nothing
/// here propagates it.
///
/// Source: CODATA recommended values, Bohr radius, 5.291 772 105 44(82) e-11 m
/// Revision: CODATA 2022
pub const BOHR_RADIUS_ANGSTROM: f64 = 0.529_177_210_544;

/// The Avogadro constant, per mole.
///
/// `0005` makes mass density an edge quantity: an operator gives grams per
/// cubic centimetre and a table holds them, and what the transport sees is a
/// number density in atoms per cubic angstrom. This is the constant that
/// conversion is made of, and it lives here so the conversion at the edge
/// cannot be written from a second copy of it.
///
/// Exact by definition since the 2019 revision of the SI, so it carries no
/// uncertainty.
///
/// Source: The International System of Units, the Avogadro constant, exact by definition
/// Revision: SI 2019
pub const AVOGADRO_PER_MOLE: f64 = 6.022_140_76e23;
