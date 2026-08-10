# Fixtures for the run with the network removed

Three programs that exist only to be judged, and a fourth thing they are judged
by, which is the restriction itself in `.github/workflows/no-network.yml`.

`a_program_that_checks_for_a_new_version.rs` contacts a server while starting
up. It is run twice and the pair of runs is the proof. Unrestricted it has to
succeed, because a program that failed everywhere would say nothing about what
the restriction removed. Restricted it has to be refused. The difference between
the two runs is the property, and neither run alone is.

`a_program_that_reads_only_its_input.rs` is the neighbour. It has to pass in
exactly the environment that refuses the one above, or that refusal is an
environment refusing everything.

`a_server_the_version_check_can_reach.rs` is the other end of the first one, so
the pair of runs depends on nothing outside this repository. It binds the
runner's own routable address rather than loopback, because loopback is present
inside the restriction too and a version check that reached a loopback server
would pass on both sides of it.

They are single files rather than crates, compiled with plain `rustc`, following
the fixtures beside them in `.github/fixtures/headless`. They have no dependency
and no build configuration, and nothing in the tree builds, formats or lints
them.

What these fixtures do not cover. The declared fetch in
`.github/network-exceptions.md` is not run here. Running it would reach the
IAEA on every change, which is a real service this repository does not own, and
a check that goes red when somebody else's server is busy is a check people
learn to re-run. What is proved instead is that a program doing the same thing
passes when it is declared and is refused when it is not, so the list is what
makes the difference. That the declared fetch itself works is the harness in
`crates/needs-hardware-network-or-time`, which is asked for by hand.
