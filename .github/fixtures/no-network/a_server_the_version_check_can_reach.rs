//! The other end of the version check, so that the pair of runs proving the
//! restriction needs nothing outside this repository.
//!
//! It binds an address on the runner's own network interface rather than on
//! loopback. Loopback exists inside the restricted environment as well, so a
//! version check that reached a loopback server would pass on both sides of the
//! restriction and would prove nothing. What is unreachable once the network is
//! removed is the runner's own routable address, and that is what this binds.
//!
//! It answers connections until it is killed rather than answering one and
//! exiting. A server that had already gone away would make the restricted run
//! fail for the wrong reason, and a check that goes green because its own
//! fixture died is worse than no check.
//!
//! It writes the path it is given once it is listening, which is how the caller
//! knows the port is open without polling for it.

use std::io::{Read, Write};
use std::net::TcpListener;

fn main() {
    let mut arguments = std::env::args().skip(1);
    let (Some(address), Some(ready)) = (arguments.next(), arguments.next()) else {
        eprintln!("usage: a_server_the_version_check_can_reach <host:port> <path to write once listening>");
        std::process::exit(2);
    };

    let listener = match TcpListener::bind(&address) {
        Ok(listener) => listener,
        Err(reason) => {
            eprintln!("could not listen on {address}: {reason}");
            std::process::exit(1);
        }
    };

    if let Err(reason) = std::fs::write(&ready, b"listening\n") {
        eprintln!("listening on {address} and could not write {ready}: {reason}");
        std::process::exit(1);
    }

    for connection in listener.incoming() {
        match connection {
            Ok(mut stream) => {
                let mut request = [0u8; 256];
                let _ = stream.read(&mut request);
                let _ = stream.write_all(b"0.0.0 is the newest version\n");
            }
            Err(reason) => eprintln!("a connection to {address} failed: {reason}"),
        }
    }
}
