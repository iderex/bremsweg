# The crates, and how they are tested

Three crates. `core` holds the physics and performs no input or output. `fit`
reads the experimental compilation and writes a coefficient file. `cli` owns
argument parsing, file reading and result writing and holds no physics. The
argument for that split is in `docs/decisions/0002`.

What follows fixes the test conventions the rest of the suite follows. It is
here rather than in a document somebody has to find, because the conventions are
about the files beside it.

## Running the suite

    cargo test --workspace

That is the whole ordinary suite. It needs no display, no elevated privilege and
no network, and any test that would need one of those does not belong in it.

A test that genuinely needs particular hardware, the network, or more minutes
than a per change run allows goes in `crates/needs-hardware-network-or-time`,
which has its own README. Nothing runs that harness on a change and it is asked
for by hand. The ordinary run above still compiles it and reports its tests as
ignored; what the ordinary run does not yet do is say that the harness was not
asked for and what asking would need, which is issue #15.

## Where a test lives

A test of one function lives beside it, in a `#[cfg(test)] mod tests` block in
the same file. It can reach private items, and the thing it tests is on the
screen above it.

A test of what a crate promises to everything outside it lives in `tests/` in
that crate, one file per property. It sees only the public interface, which is
what makes it a test of the promise rather than of the implementation.

A test that reads something about the repository itself, such as
`crates/core/tests/dependencies.rs` reading a manifest, lives in `tests/` and
belongs to the crate whose property it is about. It is not a separate crate, so
that deleting the crate deletes the check that was about it.

## What a test may assume about where it is running

Nothing about the working directory. It is not the crate directory, it is not
the workspace root, and a test that reads a path relative to it works for one
person and fails for the next.

Paths come from cargo. `CARGO_MANIFEST_DIR` is the directory of the crate the
test belongs to and is what a test reading a tracked file starts from.
`CARGO_TARGET_TMPDIR` is a directory cargo creates for integration tests to
write in, and it is the only place a test writes. Nothing is written to the
system temporary directory, to the home directory or next to the source.

One test in the tree writes, and it writes only there: the build comparison in
`crates/needs-hardware-network-or-time` puts two target directories under
`CARGO_TARGET_TMPDIR`. The one test that reads a tracked file resolves it from
`CARGO_MANIFEST_DIR`.

## Fixtures

A fixture is named for the case it represents rather than for its shape:
`a_platform_specific_dependency`, not `manifest_two`. The name is what a reader
sees in the failure output, and a failure naming case three tells them nothing.

A small fixture is a literal in the test file, where it is on the screen with
the assertion about it. A fixture too large to read inline goes in
`tests/fixtures/` in the same crate, one file per case, named the same way.

A fixture whose exact bytes are the point, meaning line endings, trailing
whitespace or encoding, is not stored as a plain tracked file, because the
repository's own text handling can normalise it on the way in and silently
delete the byte the fixture exists to prove. Such a fixture is built in the test
from an escaped literal.

## A slow test

A test that takes long enough to change how often somebody runs the suite is
marked `#[ignore = "reason it is slow"]`. The reason is required and it says
what makes it slow, not that it is slow.

Marking it keeps it in the tree and out of the fast path. It is still run:

    cargo test --workspace -- --ignored

A test marked this way because it needs particular hardware or the network is in
the wrong place rather than merely slow, and it belongs in
`crates/needs-hardware-network-or-time` instead.

That command is therefore no longer the whole story, and the harness README says
why. A test in the harness refuses rather than skips when what it needs was not
declared, so running the line above with nothing declared turns those tests red.
The route that runs them is in the harness README and it declares what the run
may use.

## What a test asserts about a floating point number

Not equality, except where exact equality is the property under test, which
happens twice in this project: repeating a run with the same seed, and running
it at different thread counts. Both are in `docs/decisions/0006`, and both
compare exactly on purpose.

Everywhere else a comparison carries a tolerance and the tolerance has a reason
beside it. The helpers for that are issue #19 and until they exist a test
writing its own comparison says in a comment where its tolerance came from.
