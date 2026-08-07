# 0010. How uncertainty travels from the fit to a reported number

Issue: #10
Status: accepted
Date: 2026-08-07

## Decision

Three sources of uncertainty reach a number this program reports. They are kept
apart everywhere, they are never added into a single field, and the one that
cannot be computed is not given a number.

## The three sources

Statistical uncertainty, from the finite number of histories. It is a property
of this run and it shrinks predictably as histories are added.

Coefficient uncertainty, from the fitted stopping coefficients. It does not
shrink with histories at all. It is the same for every run that uses that
coefficient set, and two runs using the same set share it rather than each
having their own.

Model uncertainty, from the transport approximations recorded in 0004. It is not
a quantity anybody can compute from inside the calculation, because computing it
would require knowing the answer the approximations are wrong about.

Confusing the first two is the ordinary failure. A run with a hundred million
histories has a tiny statistical uncertainty and exactly the same coefficient
uncertainty as a run with a thousand, and a report that shows only the first
tells a reader the number is far better determined than it is.

## What is reported and by what mechanism

Statistical uncertainty is always reported, per tally, in every run. It is not a
flag and there is no way to turn it off, because a Monte Carlo number without it
is not a result. Each tally carries the estimator appropriate to it and names
which estimator that was, so a reader is not left to guess whether a quoted
interval is a standard error of a mean or something else.

Coefficient uncertainty is reported when the operator asks for it. The mechanism
is re run: coefficient sets are drawn from the covariance of the fit, the whole
calculation is repeated for each draw with everything else held fixed including
the seed, and the spread of the reported quantity across the ensemble is the
contribution. Holding the seed fixed is what makes the spread attributable to
the coefficients rather than to noise, since every member of the ensemble then
runs the same histories.

The mechanism is chosen over propagating derivatives through the transport
because the transport is a Monte Carlo process with no derivative to propagate,
and a linearised estimate would need the very sensitivity the ensemble measures.
It costs about as much as the ensemble size times a plain run, which is why it
is not the default. What that number is on a real machine is a measurement the
performance milestone makes rather than one this record asserts.

Model uncertainty is never reported as a number. There is no field for it, no
default value and no estimate. What exists instead is the comparison against
measurements in the validation milestone, which is where a reader finds its
bound: #47 reports the deviation against the incumbent tables in both
directions, #48 names the systems where the fit is not reliable, and #98 and #99
compare against published measurements. The result document points a reader at
that comparison rather than leaving the absence unexplained.

Saying nothing about the third source would be the dishonest option and
inventing a number for it would be worse, because an invented number is one a
reader can add in quadrature to the other two and arrive at a total that means
nothing.

## What the schema and the command line are bound to

The result schema has a distinct place for the statistical contribution and a
distinct place for the coefficient contribution. They are never summed into one
field unless both components remain present beside the sum, and a run that did
not compute the coefficient contribution says so rather than leaving the place
empty and letting a reader take the absence for a zero.

A result document that carries a coefficient contribution also carries the
identity of the coefficient set it came from and the source of the covariance
used to sample it. A spread with no named covariance behind it is a number
nobody can reproduce, and the identity is what ties it to the file that #43
publishes.

The command line has one way to ask for the coefficient contribution, and the
documentation states what it costs in the same place it states how to ask.

## What this record does not yet carry

The issue behind this record asks for three things beyond the position above:
the distinct places in the result schema, the command line switch and its
documented cost, and a test asserting that a document carrying a coefficient
contribution also carries the coefficient set identity and the covariance
source.

None of the three exists at the point this record was written. The result
document and its schema are #7, the machine readable output is #96, and the
command line is #95. The test is owed where the schema exists. This record fixes
what those must do and does not claim any of it is in place.
