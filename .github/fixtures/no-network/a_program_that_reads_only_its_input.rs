//! A program shaped like the run path: it reads the file it was given, computes
//! something from it, and writes the answer to its own output.
//!
//! It is not optional and it is not decoration. An environment that refused
//! every program would refuse the version check above as well, and the refusal
//! would prove nothing about what was removed. This one has to pass in exactly
//! the environment that refuses that one.

fn main() {
    let Some(path) = std::env::args().nth(1) else {
        eprintln!("usage: a_program_that_reads_only_its_input <path>");
        std::process::exit(2);
    };

    let bytes = match std::fs::read(&path) {
        Ok(bytes) => bytes,
        Err(reason) => {
            eprintln!("could not read {path}: {reason}");
            std::process::exit(1);
        }
    };

    let lines = bytes.iter().filter(|byte| **byte == b'\n').count();
    println!("{path} holds {} bytes on {lines} lines", bytes.len());
}
