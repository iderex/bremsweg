# 0006. Determinism and the random number contract

Issue: #6
Status: accepted
Date: 2026-08-07

## Decision

Random numbers come from a counter based generator. Each history has its own
stream, derived from the run seed and the index of the history, so history
number n consumes exactly the same numbers whatever thread ran it, in whatever
order, and however many threads there were.

The generator is ChaCha with twelve rounds, used as a keyed counter based
stream: the run seed is the key, the history index selects the stream, and the
position within the stream starts at zero for every history. The pinned
implementation and its version are fixed by the scaffolding milestone, and the
property required of any implementation used here is that the sequence is a
function of the key, the stream selector and the position, with no hidden state
carried between histories.

## Why a counter based scheme rather than one generator per thread

Seeding one generator per thread is the obvious arrangement and it is the one
that produces a result nobody can reproduce. The numbers a history receives then
depend on which thread picked it up and on how many histories that thread had
already run, both of which are decided by the scheduler. The run is reproducible
on one machine with one thread count and stops being reproducible the moment
either changes, which is the worst of the available failures because it looks
fine until somebody else tries it.

A counter based scheme removes the schedule from the answer instead of hoping it
does not matter. There is no state to split, no state to merge, and no question
about whether two streams overlap, because the streams are separated by
construction rather than by a birthday argument over a shared period. It also
costs nothing at the point where the work is handed out: a worker that receives
history n needs the seed and n, and needs to be told nothing else.

Philox from the Random123 family is the other well established choice and it was
considered. It is a fine generator for this and it would have to be written and
validated in this tree, since the ecosystem this project builds on does not
carry it. ChaCha is present, tested, widely used and gives the same property, so
choosing it costs a round count and saves writing and validating a generator.
The condition that would reopen this is a measurement showing generator cost is
a real share of run time, which is a measurement the performance milestone can
make and this record does not assume.

## Recoils inside a history

A history is not one trajectory. The ion produces recoils, those recoils produce
their own, and all of them draw random numbers. They draw from the stream of the
history they belong to, in the order a depth first walk of the cascade reaches
them, and one history including its whole cascade is computed by one worker.

This is what keeps the promise true below the level of the ion. If recoils were
handed out as independent work items, the order in which they consumed the
stream would depend on the schedule again, and the property would hold for the
first collision and nothing after it.

## What identical means

Identical means bit for bit identical, in every number the run reports.

It holds across these axes, meaning that changing them changes nothing:

The number of threads, and the order in which work was handed out. This is the
axis the whole scheme exists for.

Repeating the run, on the same machine, with the same binary and the same input.

It is not promised across these, and each exclusion is a real one rather than
caution:

A different number of histories. A run of two million histories is not a run of
one million continued. It agrees within its stated statistical uncertainty and
it is not the same number, and pretending otherwise would require the tallies to
be accumulated in a way that costs more than the promise is worth.

A different version of this program. Improving the physics changes the answer.
That is the point of improving it, and a version that promised stable numbers
across versions would be a version that could not be corrected.

A different coefficient set. The coefficients are an input in every sense that
matters, and the result document records which set produced it.

A different platform, a different processor architecture or a different
toolchain version. Elementary functions from the platform maths library are not
required to be identical between implementations, and a single differing last
bit in a logarithm propagates through a cascade into a visibly different
history. Making this promise portable would mean shipping this project's own
implementations of those functions and proving them, which is a large piece of
work for a promise nobody has asked for. What is promised instead is that a
result document records the platform and the toolchain it was produced on, so a
reader who gets a different number knows where to look.

## Combining tallies, which is where this is usually lost

Per history determinism is necessary and it is not sufficient. Floating point
addition is not associative, so a tally accumulated by adding each history's
contribution in the order the histories finished gives a different last few bits
for a different thread count, even though every history produced exactly the
right numbers. A program can get the random numbers perfectly right and still
fail the test in #61.

So the accumulation order is fixed too. Contributions are combined in an order
determined by the history index alone, using a reduction whose shape does not
depend on how many workers there were or on which of them finished first. The
per worker partial sums that a naive parallel accumulation produces are not
acceptable here, because their boundaries move with the thread count.

This is a constraint on the transport and tally work rather than something this
record can show, and it is named here because it is the part that gets
discovered late.

## The seed

The operator may supply the seed. When they do not, it is drawn from the
operating system random source at the start of the run.

It is not derived from the clock. A clock derived seed looks reproducible, is
not recorded anywhere by itself, and collides between runs started in the same
moment on a machine with a coarse clock.

Either way the seed is recorded in the run manifest and printed at the start of
the run. A run whose seed was not written down cannot be repeated, and a program
that draws a seed without recording it has produced a number nobody can check.

## What this record does not yet carry

The issue behind this record asks for two tests as well: one that runs the same
input at several thread counts and compares for exact equality, and one that
runs the same input twice with the same seed and compares for exact equality.
Both live in the ordinary suite.

Neither exists at the point this record was written, because there is nothing
yet to run at several thread counts. They are owed by the transport milestone,
where #61 carries the thread count half. Recording that they are owed is what
this section does, and this record does not claim the property is tested.
