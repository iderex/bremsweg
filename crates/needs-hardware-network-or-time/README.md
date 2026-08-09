# Tests that need hardware, the network or time

This is the second harness. It holds the tests that cannot run on every change,
and it is named for what they need rather than for when they run, because
`extended`, `extra` and `nightly` all describe a schedule and none of them tells
a reader why a test is not in the ordinary suite.

Three things put a test here.

It needs a particular machine. A scaling measurement made on an unknown core
count measures nothing, so the test says how many cores it needs and is refused
on a machine that has fewer.

It needs the network, and a service this project does not run. Nothing in the
ordinary suite reaches outside this machine, and a test that does would make the
suite depend on somebody else's uptime.

It needs more time than a per change run is allowed. The first test here builds
the workspace twice from an empty target directory, which took 31.26 s against
an ordinary suite that finishes in hundredths of a second. That number is the
cost of building a tree that holds no physics yet, so it is a floor.

## Running it

Nothing runs this on a change. It is run deliberately, and the run declares what
it is willing to give:

    BREMSWEG_HARNESS=network,time cargo test -p bremsweg-needs-hardware-network-or-time -- --ignored

Both words are optional and a test whose word is missing says which one it
wanted. Declaring `time` says you are willing to wait; declaring `network` says
this run may reach outside the machine.

The ordinary suite still compiles this crate and reports its slow tests as
ignored, which is the point: a reader of

    cargo test --workspace

sees a count of what was not run rather than nothing at all.

## Why an unmet requirement fails rather than skips

A test that returns early on the wrong machine is a green test that checked
nothing, and a harness full of those reports success for a machine on which
nothing ran. So an unmet requirement is a failure, it names what was missing,
and it prints the command that would have met it.

The consequence is worth stating plainly, because it is the part that surprises
somebody. Running `cargo test --workspace -- --ignored` with nothing declared
turns this harness red. That is the design working: the alternative is a run
that reports every test here as passed without having started one.

## What the harness cannot check

Two of the four requirements are declarations rather than measurements, and
neither is guessed at.

Whether the network is reachable cannot be established without using it, and
reaching one host says nothing about the service a test actually wants. How long
a run may take is not a fact about the machine at all; it is a statement about
what the person starting it is willing to wait for. Both are therefore taken
from the operator, and a test whose grant is absent is refused rather than run
and rather than skipped.

## Where a reader is told this exists

A harness nobody is told about is a harness nobody runs, and the ordinary suite
reports these tests as an ignored count that says nothing about what they are
for. So the gate names this directory on every run that did not ask for it, with
what asking would need:

    hardware, network and time harness  not run here: it is asked for by hand rather than on every change, because it builds the workspace twice. Running it here needs the run in `crates/needs-hardware-network-or-time/README.md`, which declares what that run may use

That row points back at this file rather than restating what a run may use, so
the two cannot come apart while one of them still tells a reader where to look.
