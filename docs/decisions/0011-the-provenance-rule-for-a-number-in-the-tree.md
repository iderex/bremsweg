# 0011. The provenance rule for a number in the tree

Issue: #11
Status: accepted
Date: 2026-08-08

## What this record is

What provenance means here, concretely enough that a machine can refuse a file
without it.

It is written before the first data file lands. Retrofitting provenance onto a
tree that already holds fifty files means re-deriving fifty origins from
recollection, and a project whose argument is that the incumbent's numbers cannot
be audited cannot itself carry numbers whose origin is a comment.

The check that makes this bite is #27 and it does not exist yet. What follows is
the shape it implements.

## Every file under the data directory has a record

The record is a companion file beside the file it describes, named by appending
`.provenance` to the whole file name. A file `data/elements.tsv` is described by
`data/elements.tsv.provenance`.

A companion rather than a central index, because a record beside its file is moved
by whoever moves the file and deleted by whoever deletes it, while an index in one
place drifts the first time somebody renames something and does not look there.
What it costs is a directory listing with twice as many entries, which is accepted.

There is one exception and it is a path rather than a pattern: `data/README.md`,
which is documentation about the directory and carries no number that enters a
calculation. Because the exception is a single path, it cannot spread; a second one
is a visible change to both this record and the check. What it does not prevent is
somebody writing a number into that one file, and that is what review is for.

## The record is line oriented text

A record is `Key: value` lines at column zero, then a blank line, then a body.

The format is chosen so that the check needs no parser dependency. The tree
already reads a manifest as text rather than adding a TOML parser for it, in
`crates/core/tests/dependencies.rs`, and the reasoning applies with more force to a
format this repository invented: a check whose value is its exactness should not
rest on a library nobody in this tree has a reason to audit.

## The fields, and the question each one answers

Three fields are required of every record.

`Kind:` is either `fetched` or `derived`. It answers which of the two sets below
is required, and a value that is neither is refused rather than treated as one of
them.

`File:` is the path of the file this record describes, relative to the repository
root. It answers which file this is about. Its real work is catching the record
that was copied to describe a second file and never edited, which a companion
naming convention alone cannot see.

`Hash:` is `sha256:` followed by the hexadecimal digest of the file's bytes. It
answers whether the file is still the one this record was written about.

A record of kind `fetched` requires four more.

`Source:` names the compilation or the publication, with the identifier a reader
uses to find it. It answers where the numbers came from.

`Source-Version:` is the version or revision of that source. It answers which
state of something that grows. A date is not a version: a scientific database that
gains measurements is a different object at two versions, and a reader comparing
two copies needs to know which, not when somebody happened to look.

`Obtained:` is the date, as `YYYY-MM-DD`. It answers how far behind the copy is
likely to be.

`Request:` is the request that produced the bytes, complete enough to repeat. It
answers whether somebody else can fetch the same thing, which for a versioned
service means the parameters and not just the address.

A record of kind `derived` requires three more.

`Inputs:` lists the paths in this repository the file was computed from. It
answers what the file depends on, and therefore what has to be looked at again
when one of them changes.

`Command:` is the command that produced the file. It answers whether the file can
be regenerated rather than trusted.

`Commit:` is the commit of this repository the command was run at. It answers which
code produced the numbers, since the same command at two commits is two commands.

The body is required and it says what was done to the numbers between the source
and the file: units converted, rows dropped, columns renamed, a subset selected.
That is the part a reader cannot reconstruct from either end, and a record whose
body is empty is refused.

The fitted coefficient file in #43 is a derived file and takes the derived form,
with the data it was fitted against among its inputs.

## What the check refuses

Four refusals, and each is proved by its own fixture in #27.

A file under the data directory with no record beside it.

A record whose `File:` does not resolve to a tracked file.

A record whose `Hash:` does not match the bytes of the file it names.

A record missing a field its `Kind:` requires, or carrying a `Kind:` that is
neither of the two.

## What the check cannot do

It refuses absence, never vagueness.

A record whose `Source:` says "the literature" carries every required field and
passes. That record is worse than no record at all, because it looks like
provenance, and nothing a machine reads out of this tree can tell it from a good
one. Catching it is what review is for, and saying so here is what keeps the check
from being quoted as more than it is.

One further gap is worth naming because it is the kind that gets discovered late.
A derived file whose inputs changed but which was never regenerated matches its own
hash perfectly, so nothing above notices. What would catch it is re-running the
command and comparing the output, which is a different check with a different cost,
and this record does not require it. The regression check in #49 does something
adjacent for the fit and is not a substitute.

## The near miss the fixtures are built around

Not the missing record, which everybody remembers. The record that survives a file
being regenerated: the numbers changed, the record still says what it said, and the
hash is the only thing in the tree that notices. #27 builds its fixture around that
case, and it is the reason the hash is required rather than recommended.

## What this record does not yet carry

The issue behind this record asks for the check as well as the definition: a check
refusing a data file with no record, a record pointing at no file, and a record
whose hash does not match, with the fetched and derived kinds distinguished, proved
by a fixture per reason and a neighbouring fixture that passes.

None of it exists. The data directory holds a README and nothing else, so there is
no file in the tree that this rule would apply to yet. #27 is where the check and
its fixtures are owed. Until it lands, this record is a definition and nothing
refuses a violation of it.
