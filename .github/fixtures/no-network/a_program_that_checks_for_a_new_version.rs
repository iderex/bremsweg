//! A program that contacts a server while starting up.
//!
//! This is the near miss the check is built around, and it is deliberately not
//! an upload of anything a run produced. Nobody adds one of those by accident.
//! What gets added is this: somebody asks whether a newer version exists, the
//! request carries a version, a platform and an address the other end can
//! count, and every part of it was added for a good reason.
//!
//! It exits zero only when it reached the other end and read an answer back, so
//! a run of it says whether the connection happened rather than whether the
//! program ran.

use std::io::{Read, Write};
use std::net::TcpStream;

fn main() {
    let Some(address) = std::env::args().nth(1) else {
        eprintln!("usage: a_program_that_checks_for_a_new_version <host:port>");
        std::process::exit(2);
    };

    let mut stream = match TcpStream::connect(&address) {
        Ok(stream) => stream,
        Err(reason) => {
            eprintln!("the version check could not reach {address}: {reason}");
            std::process::exit(1);
        }
    };

    let request = format!("bremsweg 0.0.0 {}\n", std::env::consts::ARCH);
    if let Err(reason) = stream.write_all(request.as_bytes()) {
        eprintln!("the version check reached {address} and could not send: {reason}");
        std::process::exit(1);
    }

    let mut answer = String::new();
    if let Err(reason) = stream.read_to_string(&mut answer) {
        eprintln!("the version check sent to {address} and read nothing back: {reason}");
        std::process::exit(1);
    }

    print!("the version check reached {address} and was told: {answer}");
}
