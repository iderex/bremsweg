# 0007. The result document, its schema and the run manifest

Issue: #7
Status: accepted
Date: 2026-08-08

## What this record is

What a result is, as a document with a schema, and what has to travel with it so
that somebody holding only the result can reproduce it.

There is no program at the point this record was written. Nothing here is a
measurement of anything a result writer does, and the schema itself is owed
rather than present.

## One document, and the readable output is a rendering of it

A run produces one self describing document. It carries the input in full, the
run manifest, and the tallies with their units and their uncertainties.

What a person reads on the terminal is a rendering of that document and not a
second output path. The failure this prevents is the one every code with two
output routes eventually has: the summary on the screen and the file on the disk
disagree, and the one somebody quoted is the one nobody can reproduce. There is
one object, and the readable form is a function of it.

## The format is JSON, with a published schema

The reasons, in the order they decided it.

It is text, so two results diff, a reviewer reads one without a tool, and the
document that a paper's supplementary material carries is the document itself
rather than an export of it.

A schema language exists for it and the schema is itself machine readable, so
"the schema refuses a tally with no unit" is a statement something can act on
rather than a promise in prose.

Every language the audience uses reads it without anybody having to choose a
library for them. That matters more here than elegance: the point of a machine
readable result is that a beam line group's existing script can consume it on the
afternoon they first try this program.

What it costs is size. A depth profile with ten thousand bins is a large amount of
decimal text, and that cost is paid knowingly rather than discovered: profiles are
arrays of numbers with the bin edges stated once, never arrays of objects with a
repeated key per bin.

Three alternatives were considered and rejected.

The incumbent's fixed width text is what this project exists to improve on, it is
parsed with regular expressions, and it has nowhere to put an uncertainty. 0009
declines to write it for that reason and this record does not reintroduce it as an
output format.

HDF5 or NetCDF would handle the large profiles better and would bring a C library
into a tree whose physics crate declares no dependency at all. Neither diffs,
neither is readable without a tool, and a reader who wants one can convert a
document that is already complete. What would reverse this is profiles large
enough that the text form is the reason somebody stops using the program, which is
a measurement rather than a guess.

CBOR gives the size back and takes the reviewer away, which inverts the priority
above.

## How numbers are written

A floating point number is written as the shortest decimal string that reads back
to the same bits. That is what makes the round trip exact, and it is what lets the
round trip test compare for equality rather than with a tolerance.

Object keys are written in a fixed declared order rather than in whatever order a
map iterates. Two runs that produced the same result then produce the same bytes,
so a diff between two result documents shows the physics difference and not the
serialiser's mood.

## The schema, and how it is versioned

The schema is a tracked file under `docs/schema/`, one file per major version,
named for that version. The directory does not exist yet and the file is owed by
#96.

The document carries a version with a major and a minor part. The major part
increments when a document valid under the old version could be misread under the
new one: a field removed, renamed, given a different unit, or given a different
meaning. The minor part increments for an addition a conforming reader can ignore.
A reader that understands major version M accepts any minor version of M, and that
sentence is the whole contract.

A released major version's schema file is never edited afterwards. A correction to
a released schema is a new major version, because a schema that changed under a
reader is worse than one that was wrong in a way the reader could see.

## The manifest, field by field, with the question each one answers

The manifest is the part that makes reproduction possible. Each field is here
because a reader needs it to answer one question, and a field that answers no
question is not added.

The schema version. Can this reader read this document at all, before it tries.

The program version and the commit it was built from. Which code produced this
number.

Whether the tree that commit came from had uncommitted changes. Is the commit
above the whole truth, because a commit identifier from a modified working tree
names code that nobody else has.

The identity of the coefficient set and its content hash. Which stopping numbers
were used, which #43 publishes and #10 requires named beside a coefficient
uncertainty.

The seed. Can this run be repeated, which is the promise 0006 makes.

The number of histories. What the statistical uncertainty is a property of, since
it is the quantity that shrinks with it and nothing else in the document says so.

The number of worker threads. So the promise that the result does not depend on it
can be tested by a reader rather than believed, which is what #61 asserts from the
inside.

The input document in full, and its hash. What was actually asked for. The input
travels verbatim as the bytes the operator's file held, not as a re-rendering of a
parsed structure, because a re-rendering is a second chance to change what was
asked. The hash is what proves two runs used the same input, and it proves it
better than a file name does.

The platform and the toolchain, to the precision 0006 requires. Where to look when
a reader on another machine gets a different number.

What the manifest does not record is fixed by 0012 and is part of this decision
rather than a courtesy: no absolute path, no user name, no host name. The manifest
is the one artefact here designed to be shared, so it is the one where those do
their damage.

## Every tally carries a unit and an uncertainty

A tally is an object with a value, a unit, and an uncertainty, and the schema
requires all three. A tally object with an extra key is refused as well, so a
field somebody invented locally does not travel as though this schema blessed it.

The uncertainty is itself an object that names its estimator, because 0010
requires a reader not to have to guess whether an interval is the standard error
of a mean or something else. Where the coefficient contribution has been computed
it sits in its own place beside the statistical one and is never summed into it,
which 0010 fixes and this schema has to make possible.

A tally without an uncertainty is what invites a reader to treat Monte Carlo noise
as a physical effect, and it is the specific thing the incumbent's output format
makes easy.

## The raised conditions are part of the document

The document carries the list of conditions the run raised, each with the stable
identifier #13 gives it, at the top level rather than inside a tally. A result
computed outside the range the fit was constrained in cannot be read without
seeing that it was, and the rendering prints the list before the numbers rather
than after them.

## What this binds

The command line in #95 and the machine readable output in #96 are written against
this document rather than against their own output shapes. The reproduction test in
#97 works from the manifest alone and nothing else. The operator guide in #102
describes this document, and the schema is what it points at for detail rather than
restating field by field.

## What this record does not yet carry

The issue behind this record asks for three things beyond the positions above: the
schema itself, a round trip test that writes a result and reads it back, and a
schema validation test that refuses a document with a missing manifest field.

None of the three exists. There is no result writer and no schema file, so this
record does not claim any part of the shape above is enforced. The schema is owed
by #96 and the two tests are owed where it lands.

One consequence of the round trip test needs recording before it is written,
because it contradicts something already in the tree. Comparing a document that
was written and read back is a comparison for exact equality, which makes it a
third place in this project where exact equality is the property under test.
`crates/README.md` says that happens twice and names the two cases from 0006. That
sentence is owed an amendment where the round trip test lands, and the places #19
lists as deliberately exact have to include this one. It is recorded here rather
than fixed here because the test does not exist yet and a document listing a third
case while the tree holds two would be the drift it warns about.
