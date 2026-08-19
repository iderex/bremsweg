# Security policy

## Reporting

Report a vulnerability through GitHub's private advisory form for this
repository:

https://github.com/iderex/bremsweg/security/advisories/new

That channel answers today. I checked rather than assumed:

    $ gh api repos/iderex/bremsweg/private-vulnerability-reporting
    {"enabled":true}

Use it instead of a public issue for anything you think is a vulnerability. If
you turn out to be wrong about that, no harm is done and I will say so and move
it into the open, which is where the rest of this project's reasoning lives.

I promise no acknowledgement deadline. A deadline I cannot keep is worse than
none: a reporter told to expect an answer by a certain day and left without one
cannot tell whether the report was rejected, deprioritised, or never arrived at
all, and the guessing is the harm. This is one person's project with no rota
behind it. I would rather you know that going in than infer it from silence.

## What this program is today, because it decides everything below

bremsweg is meant to become an open replacement for SRIM: ion ranges,
straggling and damage, resting on a stopping-power fit somebody can audit
rather than on a semi-empirical parametrisation nobody can. That program does
not exist yet. The `bremsweg` command prints one line saying no calculation is
implemented. `crates/core` holds a placeholder function, three physical
constants and a float comparison helper, and no transport physics. The fit
itself is still an argument on the issue tracker rather than code.

So a threat model written around ion transport would describe a program that is
not here. What is here, and what does real work on bytes it did not produce, is
one command and the crate behind it.

## Where the surface actually is

`cargo fetch-compilation` downloads the IAEA electronic stopping power
compilation, which is published as a zip holding zips holding tables. Almost
everything worth reporting is on that path.

`crates/fit/src/archive.rs` reads the zip container in this tree. It takes
offsets, lengths, counts and names out of headers that an attacker who controls
the archive controls completely, and it allocates and slices from them. The
file is written to refuse rather than guess, and seven of its eleven tests are
refusals: bytes that are not an archive, an archive cut short, a wrong recorded
checksum, changed bytes that no longer match it, a wrong recorded length, an
unreadable compression method and a member that expands past the length it
claims. Four of the nine refusals it can return have no fixture at all, and I
would rather say so than let you assume they are covered: a header that runs off
the end, a wrong signature, the 64-bit extensions and a member name that is not
text. A shape that gets past the refusals that are tested, or that reaches one
of the four that are not, or that makes it read or allocate outside what the
bytes justify, is the report I most want.

`crates/fit/src/landing.rs` writes each member into `data/` under the name the
archive gave it. Nothing between the reader and that write constrains a member
name to stay inside that directory. A crafted archive that puts a file
somewhere else is in scope and I would like to see it.

`crates/fit/src/compilation.rs` parses the downloaded table as comma separated
records with quoting, over content of arbitrary size and shape.

`crates/fit/src/fetch.rs` runs `curl` as a process with an explicit argument
list and no shell, then parses the `Date` header out of a dump file it writes
into the system temporary directory under a name derived from the process id.
The transport, its TLS and its root store are curl's and the platform's, not
this tree's.

`crates/fit/src/crc32.rs` and `crates/fit/src/sha256.rs` are written here
rather than taken from a crate, with the published test vectors beside them.
They are the mechanism a bad decompression or a swapped member is supposed to
trip over, so an error in either turns a refusal that fires into one that does
not.

One thing about that mechanism is worth stating plainly rather than leaving for
somebody to discover. No digest of the download is pinned in this tree. The
CRC-32 the reader checks comes out of the same archive, so it catches bytes
corrupted in transit and not an archive substituted whole and consistently.
What stands between a fetch and a substituted archive is TLS, and what notices
a second fetch of one version disagreeing with the first is the `Changed`
finding in `landing.rs`. A way past either of those is a report.

Beyond the code, the supply chain is in scope: the single dependency
`miniz_oxide` and the `adler2` it pulls, the committed `Cargo.lock` that both
cargo aliases run `--locked` against, and the workflows in `.github/workflows`,
which pin every action to a full commit hash and use no `pull_request_target`.

`unsafe_code = "forbid"` applies to every crate in the workspace, so a
memory-safety finding here is a logic error or a dependency's, and saying which
saves us both a round trip.

## What is not a vulnerability here

There is no calculation, so nothing about ion transport, ranges, straggling or
dpa can be a vulnerability in this repository yet. A report about that code is a
report about code that does not exist.

There is no listening socket, no server, no daemon, no user account, no session,
no authentication, no authorisation boundary, no stored credential and no
database. Account takeover, privilege escalation, session fixation and the rest
of that list have nothing here to attach to. A report naming one of them is
describing some other program.

The two commands that use the network use it on purpose, you invoke each of them
yourself, and `.github/network-exceptions.md` names them with the reason.
`cargo fetch-compilation` obtains the measurements. `cargo gate` drives cargo,
which downloads what the lock file names. Neither is a leak and neither is a
finding. A calculation making any outbound connection would be, and that is the
report this project would want most loudly, once there is a calculation to make
one.

`crates/needs-hardware-network-or-time` holds five tests that reach the network,
need a particular platform, or need more time than a run on every change
allows. All five are ignored on a change, and a run that asks for them has to
declare what it may use. It doing what it declares is the design, not an escape.

A wrong number, once there are numbers, is a physics defect and belongs on the
public issue tracker rather than in an advisory. The whole point of this project
is that a stopping-power number can be argued about in the open, and moving that
argument into a private channel would defeat it. The exception is a wrong number
somebody else can cause: if you can make the fit produce a chosen result by
controlling what it reads, that is a vulnerability and not a physics bug.

Missing hardening that this project never claimed is not a finding against it.
The version is 0.0.0, there is no release, and no binary is published anywhere.

Advisories against `miniz_oxide` or `adler2` that you have not traced through
this tree belong upstream first. What I want from you is the part only somebody
looking at this repository can supply: whether the way this code calls it makes
the advisory reachable here. Likewise, scanner output with no path traced
through this repository is not yet a report.

Which licence applies to the fetched data, and whether it may be redistributed
here, are open questions in the tracker. They are licensing questions and not
security ones.

## What makes a report easy to act on

The bytes, or the code that builds them. The archive reader is tested against
fixtures constructed in the test rather than checked in, so an archive built by
a short function is the fastest thing for me to turn into a failing test that
stays in the tree. Tell me which of the refusals above you expected to fire and
did not. Say what the code does instead of what it fails to do, and I will not
have to guess which half of the file you were reading.

Only the default branch is supported. There is no release to backport to.
