# 0013. What refuses, what warns and what runs on

Issue: #13
Status: accepted
Date: 2026-08-08

## What this record is

Which conditions stop a run, which travel with the result, and which are ordinary.

A physics code that continues after something went wrong still produces a number,
and a number is what people copy into papers. Deciding the classes in advance is
what keeps a wrong answer from looking like a right one. There is no program at the
point this record was written, so nothing here is a report on how one behaves.

## Three classes, and they do not share a mechanism

The three are different in kind. Giving them one mechanism, which is what a single
warning stream is, is how the second class ends up looking like the third.

## Input that cannot be right refuses before anything runs

A negative energy. A layer of zero or negative thickness. A stoichiometry that does
not sum within the tolerance #33 states. An element that does not exist. A history
count of zero. A density that is not physical.

These are cheap to detect and there is no reading of any of them under which
continuing is useful. The whole input is validated before any computation starts,
and a refusal names the field, the layer and the amount by which it is wrong.

Every violation found is reported, not just the first. An operator who fixes a deck
one refusal per run is an operator who stops using the program, and a validator that
stops at the first problem hides the second one behind it.

Nothing in this class appears in a result document, because there is no result. The
run did not happen.

## Input outside the range where the model is trusted runs, and says so

An energy above where the fit was constrained. An ion and target combination with no
experimental data behind it. A recoil energy where the binary collision assumption is
stretched.

These run. Refusing them would make the tool useless for exactly the exploratory work
people want it for, and it is not this program's place to tell an operator which
question they may ask. Ignoring them is how the incumbent's users end up quoting
numbers from regions nobody validated.

So the run proceeds and the condition is recorded in the result document as a named
flag, at the top level, where 0007 puts it. A consumer can then refuse what the
program allowed, which is the right division: the program knows what it did, the
consumer knows what it will accept.

The rendering prints the conditions before the numbers. A result computed outside the
range the fit was constrained in cannot be read without seeing that it was, and a
list printed after a table is a list nobody reads.

## Conditions detected during a run get one policy, not one policy each

An ion that will not terminate. A cascade deeper than any sane bound. A tally that
received no counts. A non-finite value, which 0005 sends here.

The failure this class produces is not any individual case, it is the accumulation of
per case handling: fifteen conditions handled fifteen ways, one of which returns
early without recording anything, and nobody can say which. So the policy is
structural.

Every condition this program can raise is declared in one place, with a stable
identifier, its class, its disposition, and one sentence of what it means. A condition
that is not in that declaration cannot be raised, because the function that raises one
takes the declared type rather than a string. There is no route by which a warning
reaches an operator without being in the list, and no route by which one is raised and
not recorded.

A disposition is one of exactly three.

Record and continue. The condition is in the result and the run finishes. A tally that
received no counts is here: zero transmission is a legitimate answer and refusing it
would refuse a correct result, but a reader must not take the zero for a measurement.

Abandon the history, record the condition, and count it. The history contributes
nothing, the count of abandoned histories is in the result, and a tally computed from
fewer histories than were asked for cannot be read without seeing that count. An ion
that will not terminate is here.

Stop the run. Nothing is written except the reason. A non-finite value is here: it
means an expression in this program is wrong, and continuing spreads it into every
tally that touches it.

Nothing else is available. A condition whose right handling is none of the three is a
condition whose class was chosen wrongly, and the argument goes in the issue rather
than into a fourth mechanism.

## Identifiers are stable

An identifier is never reused for a different meaning and never renamed. Retiring one
leaves it retired.

The reason is the consumer. The whole point of a named condition is that somebody's
script refuses on it, and a renamed identifier turns that refusal off without failing
anything: the script looks for a name that no longer appears and finds nothing wrong.
Removing a condition is therefore a schema change under 0007's rule, not a tidy-up.

## The operator can make a condition fatal, and cannot suppress one

The command line offers a way to make raised conditions fatal, for the operator who
wants a run to stop rather than to report. It is not the default, because the default
serves the exploratory use above.

The inverse is deliberately not offered. There is no switch that suppresses a
condition, no verbosity level at which the list shortens, and no way to write a result
document with the flags removed. A result whose conditions were suppressed is
indistinguishable from a clean one, which makes every clean result worth less.

## What is deliberately not a condition

Statistical noise. A tally with a wide uncertainty is not raising anything, it is
reporting; the uncertainty is the statement, and duplicating it as a flag would train
readers to ignore flags.

A result that disagrees with the incumbent. That is a measurement, published by #47,
and treating it as a condition would decide entry 4 of #1 by accident.

## What this record does not yet carry

The issue behind this record asks for three things beyond the classes above.

The list of raised conditions in the result schema, each with its stable identifier,
positioned so that a result computed outside a trusted range cannot be read without
seeing it. The schema is owed by #96 against 0007, and neither exists.

A test asserting that a run outside the fitted energy range produces a document
carrying the corresponding flag. There is no fit, so there is no fitted range to be
outside of; the range comes from #35 and the coefficient file in #43.

The command line switch that makes any raised condition fatal. The command line is
#95.

The declaration described above does not exist either. `crates/core` holds a
placeholder function, so there is nothing in the tree that raises anything, and this
record does not claim any of the policy is in force.
