//! The stopping-power fit.
//!
//! This crate reads the experimental compilation and writes a coefficient file.
//! It runs once per coefficient release rather than once per calculation, which
//! is why it is a separate unit from the physics it produces coefficients for.
//!
//! The fit itself is not here yet. The functional form is decided in issue #35
//! and the fit follows it. What is here is the step before it: obtaining the
//! measurements and recording exactly what was obtained, which is issue #26.
//!
//! # The parts of that, and why they are separate
//!
//! [`fetch`] gets bytes and the date the service dated them. [`archive`] reads
//! the container the compilation is published in and refuses bytes that are not
//! the ones the publisher checksummed. [`landing`] writes a table and its
//! provenance record together. [`compilation`] counts what arrived, so the
//! count can be compared with what the database states for the version rather
//! than reported on its own.
//!
//! Each is testable without the others and only [`fetch`] needs the network.
//! That is what lets the whole path be tested in the ordinary suite against
//! bytes built here, with the live request in the harness instead.

// The container reader, the counter and the record writer are read by different
// people for different reasons, so they are named modules rather than one file.
pub mod archive;
pub mod compilation;
pub mod crc32;
pub mod fetch;
pub mod fixture;
pub mod landing;
pub mod sha256;

/// The compilation, named as a provenance record names it.
///
/// The citation the database asks for carries its version number, which
/// `docs/data-terms.md` establishes as a condition rather than a courtesy, and
/// the record carries the version in a field of its own for the same reason.
pub const COMPILATION: &str =
    "IAEA Electronic Stopping Power of Matter for Ions, https://www-nds.iaea.org/stopping/";

/// The version this tree fetches.
///
/// A constant rather than an argument, so which version a clone obtains is a
/// fact of the tree and moves by a commit that says why. A version chosen at
/// the command line would make two clones of one commit disagree about what
/// they were fitted against, with nothing in either saying so.
pub const COMPILATION_VERSION: &str = "2026-01";

/// Where a version of the compilation is downloaded from.
///
/// Addressed by version rather than by "current". A service that grows is a
/// different object at two versions, and an address that always gives the
/// newest one cannot be the `Request:` of a record that has to be repeatable.
#[must_use]
pub fn compilation_request(version: &str) -> String {
    format!("https://www-nds.iaea.org/stopping/download/version/{version}")
}

#[cfg(test)]
mod tests {
    use super::{COMPILATION_VERSION, compilation_request};

    #[test]
    fn the_request_names_the_version_rather_than_the_newest() {
        let request = compilation_request(COMPILATION_VERSION);
        assert!(request.ends_with(COMPILATION_VERSION), "{request}");
        assert!(!request.contains("latest"), "{request}");
    }
}
