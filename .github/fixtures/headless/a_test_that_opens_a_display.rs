//! A test that needs a display, written the way one reaches for it without a
//! toolkit: a connection to the X server's socket.
//!
//! This is the plotting window somebody adds to see what a depth profile looks
//! like and then leaves behind an assertion. It passes on the machine it was
//! written on, where a desktop session is running, and it is refused wherever
//! there is no display.
//!
//! Not a member of any workspace and nothing builds it with the tree. The
//! workflow beside this directory compiles it with `rustc --test` and requires
//! it to fail.

/// The socket the X server listens on for display zero. Named directly rather
/// than through `DISPLAY`, so this file is refused because there is no display
/// rather than because a variable was unset.
const DISPLAY_SOCKET: &str = "/tmp/.X11-unix/X0";

#[test]
fn shows_the_depth_profile() {
    let connection = std::os::unix::net::UnixStream::connect(DISPLAY_SOCKET);

    assert!(
        connection.is_ok(),
        "no display to draw on: {DISPLAY_SOCKET} did not accept a connection"
    );
}
