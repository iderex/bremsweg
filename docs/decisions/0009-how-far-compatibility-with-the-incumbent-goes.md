# 0009. How far compatibility with the incumbent goes

Issue: #9
Status: accepted
Date: 2026-08-08

## What this record is

How far this program meets an existing user where they already are: what it reads
of the incumbent's files, what it refuses to write, and what neither of those
decides.

There is no converter and no command line at the point this record was written.

## The decision

One direction, and it is a converter rather than a compatibility mode.

A separate command reads an incumbent input deck and writes this program's input
document. The core, the transport and the calculation never learn the old format.

Writing output the incumbent's downstream tooling can read is declined, and the
reason is recorded here rather than left to be inferred.

## Why a converter and not a compatibility mode

A compatibility mode hides the translation inside the run. A converter puts it on
the disk, where it is a file somebody reads, diffs and argues with before a single
number is computed.

That difference matters most on the day the two models disagree about what a field
means, which is the day a compatibility mode is at its worst: the run produces a
plausible number and nothing anywhere records which reading of the field produced
it. With a converter the reading is a document, and a wrong reading is caught by a
person looking at the converted input rather than by somebody noticing that a range
came out ten percent short.

It also keeps one input format in the program. A parser for an undocumented fixed
width format is a thing that will be wrong about edge cases for years, and confining
it to a command that runs once per deck rather than to the path every run takes is
the difference between a bug in a tool and a bug in the engine.

## Why output compatibility is declined

The incumbent's output format has nowhere to put an uncertainty.

A compatibility output would therefore drop, silently and by construction, the one
thing this project adds to every number it reports. A user's existing script would
keep working and would keep producing tables of values with no interval beside
them, which is the state this project exists to improve on, now with this program's
name on it.

That is the whole reason, and it is not a judgement about effort. If the format
could carry an uncertainty the answer might well be different.

What is offered instead is that the result document in 0007 is machine readable
and documented, so a script that today parses fixed width columns can be pointed at
a document with a schema. Converting a result document into whatever a particular
laboratory's tooling wants is a script that laboratory can write and read, and this
project would rather help with that than ship a format it cannot stand behind.

## What the converter has to produce

The converted document has to carry everything the calculation needs: the ion and
its isotope, its energy, the incidence angle, the number of histories, and the
layer stack with each layer's thickness, composition and density, in the
conventions #33 fixes rather than in the incumbent's.

Two rules bind it, and they matter more than the field list.

It refuses rather than guesses. A token the converter does not understand is an
error that names the token and where it was found, and it is never a default that
lets the run proceed. A converter that silently supplies a missing value is a
converter that produces a result nobody can attribute to a deck.

A field whose meaning belongs to the incumbent's model is refused rather than
mapped. The per element energies a deck of this kind carries are parameters of a
particular damage model, and 0008 forbids emitting a displacement number without
naming the model that produced it. Carrying those numbers across into this
program's parameters would be exactly the quiet substitution 0008 refuses, so the
converter names them, states that they are not translated, and leaves the operator
to state the parameters this program's measures need in this program's own terms.

The converted document records what it was converted from: the name and content
hash of the deck, and the version of the converter. A converted input that cannot
be traced to the deck it came from removes the reason for having a converter.

## What is not decided here

Whether this program's numbers are expected to agree with the incumbent's is not
compatibility and is not settled here. It is entry 4 of the maintainer decision
issue, #1, and the entry is open. The comparison is published either way, which
#47 does; what the entry decides is how a difference is described.

## What this record does not yet carry

The issue behind this record asks for two things the tree cannot support yet.

The field level mapping: which token of the incumbent's input the converter
understands, and which it refuses. This record deliberately does not list them.
The format is undocumented, no deck is in this tree, and a list written from
recollection of a fixed width layout is precisely the defect this project was
started over, one level down. The list is owed against a real deck, in the change
that builds the converter, and the rules above are what it has to satisfy.

The paragraph that tells a reader arriving with an existing deck what to do. That
belongs in the documentation, and the operator guide is #102. The contributing
guide, #25, is where a contributor would look, and it is held by the first entry of
#1.

So what stands today is the direction, the refusal, and the rules the converter is
bound by. The converter itself does not exist and nothing in this record claims it
does.
