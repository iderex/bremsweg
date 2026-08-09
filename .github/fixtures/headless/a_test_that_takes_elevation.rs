//! A test that raises its own privileges, which is the near miss this
//! environment is built around.
//!
//! It is not an obvious violation. It is the fixture that needs a file under a
//! system path, or a device node, or a port below a thousand, and reaches for
//! the one command that makes all three available. It passes on an ordinary
//! runner, where that command is there for the asking, and the whole point of
//! the environment is that it stops passing there.
//!
//! Not a member of any workspace and nothing builds it with the tree. The
//! workflow beside this directory compiles it with `rustc --test` and runs it
//! twice: once in the ordinary environment, where it has to pass, and once in
//! the environment the suite runs in, where it has to fail. Both halves are the
//! proof. Without the first it would be a test that fails everywhere.

#[test]
fn writes_the_reference_table_where_the_system_keeps_them() {
    let raised = std::process::Command::new("sudo")
        .args(["-n", "/usr/bin/true"])
        .status();

    assert!(
        raised.is_ok_and(|status| status.success()),
        "could not raise privileges, so the system path this test writes to is \
         out of reach"
    );
}
