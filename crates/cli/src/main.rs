//! The command line.
//!
//! This crate owns argument parsing, file reading and result writing, and holds
//! no physics. The command line an operator actually writes is decided in issue
//! #95; what is here is enough to prove the workspace produces a binary.

fn main() {
    println!(
        "{} {}: no calculation is implemented yet.",
        env!("CARGO_BIN_NAME"),
        env!("CARGO_PKG_VERSION")
    );
}
