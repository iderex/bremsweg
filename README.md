# bremsweg

SRIM computes ion ranges, straggling and radiation damage and is the de facto standard for implantation, sputtering, damage research and ion therapy, in particular for dpa. It is closed source, written in Visual Basic 5.0, Windows and IA-32 only, and people drive its GUI from outside under Wine. The paid and free alternatives all rest on Ziegler's tables. The real contribution is not the trajectories, which parallelise embarrassingly well, but an open re-fit of the electronic stopping cross sections against the IAEA databases, because the ZBL parametrisation is a semi-empirical fit nobody can audit.

Planning happens on the issue tracker first. Every decision that shapes
the architecture is written down there with its reasons before the code
that depends on it exists.

## What leaves your machine

Nothing you compute, and nothing about you.

This program is meant to be run on a machine that holds other people's work, so
what it does with a network is fixed in advance rather than settled feature by
feature. Nothing here computes anything yet, so what follows is the standard the
program is held to, and the part of it that something already refuses is marked
as such at the bottom.

The program reads the files you give it, computes, and writes results to your
disk. Running a calculation makes no outbound network connection at all. Not a
reduced set of connections, and not connections to one named address. None.
There is no telemetry, no crash reporting and no usage statistics, so there is
nothing here for you to find and switch off.

Two commands do use the network, and you invoke each of them yourself.

`cargo fetch-compilation` obtains the experimental stopping compilation from the
IAEA electronic stopping power database, which is where the measurements the fit
is made against come from. It computes nothing, and nothing on the path that runs
a calculation reaches it.

`cargo gate` runs the checks. It drives cargo, and cargo downloads the
dependencies the committed lock file names from the crate registry. Nothing in it
contacts anything belonging to this project.

A result carries what somebody else needs in order to reproduce it, and that is
the whole of what it carries: the input and its hash, the seed, the number of
histories, the identity of the coefficient set, the version of the program, and
the platform it ran on. Not where on your disk the input lived, not who was
logged in, not what the machine is called, and not what else was in the
directory. A result is an artefact people attach to a paper or a mailing list
post without reading it first, and it is designed for that.

If a future version ever offers to send something somewhere, it is off until you
turn it on, and it tells you what it sends and where it goes at the moment you
turn it on. That is a default for you to change, and not a claim above for
anybody to withdraw.

`docs/decisions/0012` is the position in full, with the reasoning behind each
part of it.

### What refuses a violation of this, and what it does not yet reach

`.github/workflows/no-network.yml` runs the ordinary test suite, and every binary
the run path is made of, inside a network namespace that has a loopback device
and no route off the machine. A connection to any address elsewhere fails in the
kernel before a packet exists. It removes the network and runs the thing rather
than reading the source for a networking library, because reading the source
catches what somebody thought of and misses a dependency several levels down
reaching out on its own.

`.github/network-exceptions.md` decides what is exempt from that. It names every
binary this workspace builds exactly once, either as part of the run path or as
an exception carrying the reason it may reach the network, and the check
reconciles that list against what cargo says the workspace builds. A new binary
cannot acquire the network by arriving in a diff nobody read.

What no run of that job establishes today is the larger half. There is no
calculation here: `bremsweg` prints a line saying so, and running it with the
network removed shows that the line needs no network and shows nothing further.
Until a calculation exists and is run there, a green run of that job may not be
read as saying that a calculation makes no connection. Issue #93 is where that is
held open, and it stays open for exactly this reason.

See [NOTICE.md](NOTICE.md) for the intended-use notice.

## License

AGPL-3.0, copyright 2026 Nils Lehnen.

The full text is in [LICENSE](LICENSE).
