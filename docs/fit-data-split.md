# The split rule for held out data

A fit reported against the data it was fitted to says how flexible the model is.
The number that means anything is the error on data the fit never saw, and that
number is only worth reading if nobody could have chosen which data that was.

So the rule is written here, first, and this file is landed at a commit where
`crates/fit` is empty and no measurement has been fetched into `data/`. That is
the only form of evidence for "fixed before the fit was run" that cannot be
manufactured afterwards. Every later change to this file carries the reason for
the change and leaves the numbers it replaced readable.

## Two splits, both run, reported separately

Holding out a random selection of points is the wrong unit and it flatters the
result. Points from one measurement series are correlated: the same apparatus,
the same calibration, often the same sample. A random split leaves neighbours of
every held out point in the training set, and the error it reports is far better
than the error a new measurement would see.

The dataset split holds out whole measurement series. It answers what the fit
says about a series it has never seen, for combinations of ion and target it has
seen measured by somebody else.

The system split holds out whole ion and target combinations, every series for
that combination at once. It answers a harder question, which is what the fit
says about a combination it has no measurement of at all. That is the question a
reader with an unusual target actually has.

Neither one is validation on its own. Both are run, both are reported, and
neither number is quoted without saying which of the two it is.

## The assignment

Each unit is assigned by hashing its identifier. No shuffling, no sampling, no
list of chosen points anywhere in the tree.

The hash is FNV-1a over 64 bits, chosen because it is two constants and four
lines and a reader can reimplement it in whatever language they have to check
this repository's arithmetic rather than trust it. The input is the byte string

    <seed>/<label>/<identifier>

encoded UTF-8, where `<seed>` is the sixteen lowercase hexadecimal digits of the
seed below, `<label>` is `dataset` or `system`, and `<identifier>` is described
in the next section. Starting from the offset basis `0xcbf29ce484222325`, each
byte is exclusive-ored into the accumulator and the accumulator is then
multiplied by the prime `0x100000001b3`, modulo two to the sixty-fourth.

The unit is held out when the resulting value modulo 1000 is below its
threshold. The thresholds are 200 for the dataset label and 100 for the system
label.

The seed is

    0x4d2f8a1c7b3e6905

It was typed once and it means nothing. That is the property that matters: a
seed with a meaning is a seed somebody can argue should have been a different
one, and a seed changed after a number was seen is the failure this whole file
exists against.

The label is in the hashed string so that the two splits are independent. A
system held out by the system split contains series that the dataset split may
well have put in training, and that is correct rather than a defect, because
they are two different tests. What it forbids is reading one number as evidence
for the other, and the reporting keeps them apart for that reason.

## Three properties this construction has, and one it does not

It is regenerable. Anybody with the compilation, this file and no access to this
repository's code can reproduce both held out sets exactly.

It is stable as the compilation grows. The assignment of a unit depends on that
unit's identifier and nothing else, so a new release of the compilation adding
series and combinations leaves every existing assignment where it was. A rule
that shuffled and took a fraction would not have this: adding one series would
move units across the boundary, and the held out set would quietly become one
the fit had seen. This is the property that makes the rule survive a version
bump, and it is worth more than the exactness of the fraction.

The fractions are approximate by construction. Two hundred in a thousand is what
the hash gives in the limit, not what it gives on four thousand series, and the
realised counts are reported as counts rather than as the nominal fractions.

What it does not have is any guarantee about what lands in the held out set. The
system split may take ten light systems and no heavy ones, purely by hash. If it
does, that is reported as a limit on what the system level number says. It is
not repaired by reassigning, because a held out set adjusted until it looks
representative is a held out set somebody chose.

## The identifier, and the one input still open

The identifier is the compilation's own name for the unit, taken verbatim. It is
not constructed here and it is not derived from any measured value, which is the
property that matters: an identifier computed from the data could move a unit
across the boundary when a number in it is corrected.

Which of the compilation's fields carries that name, exactly, is fixed by the
work that reads the compilation into an internal representation. Recording that
choice here is an amendment to this file and not a change to the rule, and it
will carry no numbers, because nothing will have been fitted when it is made.

The requirement it has to meet is stated now so the choice can be judged against
it rather than made freely. The identifier is stable across releases of the
compilation, unique within its label, and present for every unit. A unit whose
identifier is missing is refused rather than assigned, because a unit that
silently hashes an empty string joins whichever side that string lands on and
does so for every such unit at once.

## Where a system has one series

For a combination measured by only one series, the dataset split and the system
split are the same test. There is no separate question to ask, and reporting two
numbers for it would suggest two pieces of evidence where there is one. The
counts of such combinations are reported alongside both splits so a reader can
see how much of the dataset level result is carrying that shape.

## What no check can enforce

The held out data is not looked at while the functional form or the objective is
being chosen. Nothing in this repository can refuse a violation of that. It is
not a property of the tree, and there is no artefact a check could read that
distinguishes a form chosen blind from one chosen after a glance at the held out
residuals.

So it is written here, at the point where somebody about to do it would be
reading, and the only thing standing behind it is that this file was landed
before there was anything to look at.
