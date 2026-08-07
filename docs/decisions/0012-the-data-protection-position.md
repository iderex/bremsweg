# 0012. The data protection position

Issue: #12
Status: accepted
Date: 2026-08-07

## What this record is and is not

This is the position the program is held to. There is no code in the tree at the
point it was written, so nothing here is a measurement of how a program behaves.
It is the standard the later work is judged against, and the check that would
make it refusable is named at the end as owed rather than present.

## The position

This program reads an input file describing an ion, a target and a beam,
computes, and writes a result to the operator's disk. None of that is personal
data.

Three things follow, and they are the position rather than a description:

Personal data never leaves the host.

A calculation makes no outbound network connection at all. Not a reduced set of
connections, and not connections to a named endpoint. None, on the path that
runs a calculation.

Anything this project ever adds that would send something anywhere is off unless
the operator deliberately turns it on, and at the moment they turn it on they
are told what is sent and where it goes.

## Why this is a decision rather than an assumption

Saying it once is easy. Keeping it true is the work, and it is lost to ordinary
features that nobody adds in bad faith.

A crash reporter. It uploads a stack trace, and a stack trace carries file paths,
which carry user names and directory layouts, and on a shared research machine a
directory layout carries other people's project names. The default is that there
is none. If one is ever added it is off until the operator turns it on, and what
it would send is shown to them at that moment rather than described in a
document they will not open.

An update check. It asks a server whether a newer version exists, and the
request carries an address, a version, an operating system and often an
identifier that is stable enough to count installations with. The default is that
there is none, and a version check that a user runs deliberately is a different
thing from one the program performs on its own.

A log that ends up in a result document. This is the one that actually happens.
The program writes a result, the result carries the working directory, the input
path and the user name for context, and the operator attaches the result to a
mailing list post or a paper's supplementary material without reading it. The
default is that a result document carries no absolute path, no user name and no
host name.

## What the run manifest deliberately omits

The manifest is the one artefact this program produces that is designed to be
shared, since its whole purpose is to let somebody else reproduce a result. So
it is the artefact where the three features above do their damage.

It records what reproduction needs: the content of the input and its hash, the
seed, the number of histories, the identity and hash of the coefficient set, the
version of the program, and the platform the run happened on to the precision
0006 requires.

It does not record where on somebody's disk the input lived, who was logged in,
what the machine is called, or what else is in the directory. A hash of the
input content is what proves two runs used the same input, and it proves it
better than a path does, since a path proves only that two files had the same
name.

The manifest work is #7 and it is written against this. A field added there that
carries a path, a user name or a host name is a change to this record, made by
superseding it, rather than a detail settled in the schema.

## What is not decided here

Whether any shared or federated feature is ever built is not decided here. That
is an entry in the maintainer decision issue, #1, and the entry is open. What is
decided here is that the default is no, that turning anything on is a deliberate
act by the operator, and that the moment of turning it on is where they are told
what it does.

## What this record does not yet carry

The issue behind this record asks for two more things.

A check that refuses an outbound network call from the run path, proved by a
fixture that trips it. There is no run path and no check. The refusable half is
#93, which is where the check and its fixture are owed. Until it lands, the
position above is a statement in a document and nothing refuses a violation of
it.

The same statement in operator language in the documentation, with the notices
work not contradicting it. That is #103 for the operator wording and #104 for
the notices, and neither has landed. The notice already in the tree speaks about
lawful use and does not touch this, so there is nothing to contradict yet.
