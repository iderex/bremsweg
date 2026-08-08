# The release readiness checklist

A plan made milestone by milestone cannot see whether the whole thing holds
together. This file is the list that looks at that, once, before a release
leaves.

It is written before there is a release, and that is the point rather than an
accident of ordering. A checklist written after a release is a description of
what happened, and every item in it passes because it was written from the
thing it was meant to judge.

Nothing in this repository refuses a violation of anything below. No check reads
this file, and none could read whether an item was run honestly. What stands
behind it is the record each run leaves, which is why the recording rules come
before the items.

## The three outcomes, and there is no fourth

An item is passed, failed or not run. Every item in a run gets exactly one of
them, and the record says which.

Passed means it was run and what came back is in the record: the command and its
output, or the observation and who made it. An item passed with nothing beside it
is not passed.

Failed means it was run and did not come back as the item requires. The record
says what was done about it, and if the answer is that nothing was done, it says
that.

Not run means it was not run. The record says why. An item whose subject does not
exist yet is not run, with that as the reason, and it is never passed and never
struck out as inapplicable. This is the outcome the checklist is most likely to
lose, because at the moment somebody is reading it a release is close and the
list is the last thing in the way.

Nothing is marked passed on the ground that it was expected to pass. An
expectation is not evidence, and an item whose only evidence is that it has
always worked is not run.

## Where a run is recorded

One file per release attempt, in `docs/release-readiness/`, named for the version
it was run against. That directory does not exist yet and the first run creates
it.

A record is added, never edited. A second attempt at the same version is a second
file rather than a correction to the first, because what the first one found is
the thing most worth keeping.

The release notes state whether the release went out with items outstanding, and
name them. A release that went out with three items not run and says so is a
different artefact from one that went out with three items not run and does not,
and the difference is the only thing a reader outside this repository can see.

## 1. The gate passes on a clean clone

Clone the repository at the commit being released into an empty directory, on a
machine carrying nothing this project needs beyond the toolchain
`rust-toolchain.toml` pins. Run the command this repository documents as its
whole gate. Record the command, its exit status and its printed account of what
it examined.

It passes when the exit status is zero and the account names every leg, with
nothing reported as skipped except the harness in
`crates/needs-hardware-network-or-time`, whose absence from a per change run is
deliberate and whose cost the account states.

The failure this catches is a gate that passes only where it was developed,
because something on that machine was installed by hand and never written down.
A clone is what a reader gets.

## 2. The guide works for a reader who did not write it

The guide is followed end to end by somebody who did not write it, on a machine
that has never had this program on it, from the guide alone. Every command they
run is copied from the guide unmodified. They ask nobody anything.

It passes when they reach the result the guide says they will reach and departed
from the text nowhere. A departure is a failure of this item even where the
reader got there anyway, because the next reader meets the same gap without the
knowledge that carried this one over it.

The record names who read it, what machine they used and every place they had to
depart, quoting the sentence they departed from.

## 3. A result is reproduced from its manifest by somebody else

Hand a result document and its manifest to somebody who did not produce it. Give
them nothing else: no working directory, no shell history and no spoken
instruction. They produce a result from the manifest.

It passes when the two results agree to the exactness
`docs/decisions/0006-determinism-and-the-random-number-contract.md` promises,
over the axes that record covers, and disagree on nothing that record does not
exclude. The record names the manifest, both result digests and both machines.

What this catches is a manifest that is complete only in the presence of the
directory it was made in.

## 4. Every published number carries the command that produced it

List every number that appears in the release notes, in `README.md`, in the guide
and in anything the release links to. For each one, name in the same document the
command that produced it or the tracked file it was read from.

It passes when nothing is left over. A number with nothing behind it is a failure
of this item, and the repair is to produce it again or to remove it, not to
describe where it came from from memory.

A number about how fast something ran carries the machine it ran on as well as
the command, because without the machine it is not a measurement of anything.

## 5. The licence question is answered and the tree agrees with the answer

The tree carries a licence file naming one licence. Every crate manifest in the
workspace declares that same licence. `README.md` names it.

It passes when all three agree and the comparison is in the record. Where the
licence is not settled, this item is not run, with that as the reason, and the
release notes carry it as outstanding.

## 6. The notices match what is shipped

Derive the third party components in the shipped artefact from the artefact and
from `Cargo.lock`, not from the notices file, and compare the two sets in both
directions.

It passes when nothing in the artefact is missing from the notices and nothing in
the notices is absent from the artefact. The second direction is the one that
goes wrong quietly, because a notices file that has only ever grown reads as
careful.

Where the release carries data or coefficients derived from the reference
compilation, the attribution that compilation requires is present in the exact
form `docs/data-terms.md` records, including its version, and nothing in the
release states or implies that the compilation's maintainer endorses this program
or its numbers.

## 7. The parity table has no unexplained gap

Regenerate the parity table against the target at a commit no older than the
commit being released. Read the group it reports for checks on the target that
have no verdict here.

It passes when that group is empty, or when every entry in it carries a sentence
saying why it has no verdict yet. The verdicts themselves are in
`docs/target-gate.md` and are not repeated here or in the release notes.

## 8. No claim in the documentation is stronger than the evidence behind it

Somebody reads every document the release ships or links to, sentence by
sentence, looking only for the distance between what a sentence asserts and what
stands behind it. Where nothing stands behind a sentence, the sentence is changed
or the claim is weakened. Where a sentence says a thing was verified and it was
argued rather than run, it says argued.

This item needs a person and it cannot be run any other way. It is also the item
worth the most, because everything else on this list checks that a mechanism
works, and this one checks whether the repository is telling the truth about
itself.

It passes when the record names who read what and lists every sentence changed,
with what it said before and what it says now. A reading that changed nothing is
recorded with what it covered and how long it took, because otherwise a reading
that found nothing and a reading that never happened leave the same trace.

## What this file is not

It is not a second copy of the milestone. An item here is a thing somebody runs
and an outcome somebody can disagree with and be shown wrong, and an item that
could only be settled by looking up whether some other piece of work is finished
does not belong on it.

It is not exhaustive, and a run that finds something this list does not cover
records it anyway rather than discarding it for being off the list. What it finds
is what the next version of this file is for.
