# What may reach the network here, and what may not

`docs/decisions/0012` fixes the position: a calculation makes no outbound
network connection at all. Not a reduced set of connections, and not
connections to a named endpoint. None, on the path that runs a calculation.

This file is the half a machine reads. Every binary this workspace builds is
named below exactly once, either as part of the run path, which is judged with
the network removed, or as an exception that may reach it, with the reason it
may. `.github/workflows/no-network.yml` is the check that reads it.

What the check does with each kind. A binary named `Run-path:` is executed in an
environment where the network has been removed, and it has to succeed there. A
binary named `Allowed:` is not executed under that restriction, so being on this
list is the whole of what exempts it.

Three ways this file fails the check, and each of them is a way an exception
gets added without anybody deciding to add one.

A binary the workspace builds and this file does not name. That is the diff
nobody notices: a new command arrives, it is on neither list, and nothing has
said whether it may reach the network. It is refused until somebody writes the
line.

A name here that the workspace no longer builds. An exception outlives the
command it was written for, and the next command to take that name inherits it.

An `Allowed:` line with no `Reason:` line under it. An exception nobody had to
justify is the one that gets added, so the reason is required rather than
encouraged.

The lines below are what the check reads and the prose around them is not.

Run-path: bremsweg
Reason: the command an operator runs to compute a result. It reads the files it
is given, computes, and writes to the operator's disk. Nothing about a
calculation needs anything from anywhere else, and `0012` is the position this
binary is held to.

Allowed: fetch-compilation
Reason: obtains the experimental stopping compilation from the IAEA, which is
where the measurements the fit is made against come from. It is a separate
command an operator invokes deliberately, it computes nothing, and it is not
reachable from a calculation. Issue #26 is where what it fetches and what it
records was argued.

Allowed: xtask
Reason: the gate rather than the run path. It drives cargo, and cargo downloads
what the lock file names, so running the gate against a clone with a cold cache
reaches the registry. Nothing in it contacts anything belonging to this project,
and no calculation runs through it.
