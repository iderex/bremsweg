# The exclusion rules for the fit's input data

A fit is as good as the data selection behind it, and the selection is where a
fit stops being reproducible if nobody writes it down. The compilation holds
measurements from a century of work, made by techniques of very different
reliability, some superseded by their own authors, some with no stated
uncertainty at all. Fitting all of it unweighted would be wrong. Excluding what
disagrees with the model would be worse, because it produces a fit nobody can
audit and one that looks better the more it is tampered with.

So the rules are written and argued here, first, and this file is landed at a
commit where nothing has been fetched into `data/` and `crates/fit` is empty.
That is the only form of evidence for "written before the selection was run"
that cannot be manufactured afterwards. Every later change to this file carries
the reason for the change and leaves what it replaced readable.

The file is not complete and says where it is not. Two of the four classes below
are stated in full. One is stated with a bound that belongs to the work fixing
the functional form. One is named with its requirements and no members, because
its members can only be written against the compilation's own vocabulary.

## The rule no rule may be

No exclusion rule refers to agreement with the model. This is the sentence the
whole file exists to carry, and it is written here before any rule rather than
after, because a document that acquires it once the rules were tuned reads
exactly like one that always had it.

It rules out more than its plainest form. A rule may not exclude a point for its
distance from a current fit, for the size of its residual, for being an outlier
where outlier is defined against a fitted curve, and it may not be a rule whose
threshold was moved until the residuals improved. The last is the one that
arrives disguised, because the rule that results has a metrological wording and
a number chosen from the wrong place.

Disagreement between two measurements is also not a ground. Where two series
disagree and no metrological rule separates them, both stay. Preferring one is a
judgement about which is right, and this file has no way to make that judgement
that is not the model making it.

## What an exclusion is, and what it is not

An excluded point is not a removed point. Every point read from the compilation
stays in the tree carrying the identifier of the rule that excluded it, so that
somebody who disagrees with a rule can run the fit without it and see what
changes. A selection that is auditable only in the sense that it was disclosed
is not what this asks for.

Rule identifiers are lowercase, stable, and never reused. A retired rule keeps
its identifier and its entry here, with the date and the reason it was retired,
because a coefficient set published while it was in force was produced by a
selection that included it.

Exclusion is not weighting, and the two are separate mechanisms with separate
justifications. Weighting a point by its stated uncertainty is the objective's
business and leaves the point in the fit. A rule here removes the point
entirely, which is the stronger act, so its ground has to be a statement about
how the measurement was made rather than about how much it should count.

## Supersession by the same group

Rule `superseded-by-author`. A measurement series is excluded when the group
that published it later published a series describing the earlier one as
superseded, in the source itself.

Both halves are required and each carries weight. The same group, because a
later measurement by somebody else is a disagreement rather than a withdrawal.
And an explicit statement in the source, because a later measurement by the same
group is often a different energy range, a different sample or a different
technique, and treating every later publication as a withdrawal of the earlier
would exclude a large amount of good data on an inference nobody made.

The justification is that a group withdrawing its own measurement is the
strongest statement available about that measurement, and it is a statement
about how the measurement was made rather than about what it agrees with. It is
also the one ground on this list that needs no threshold and no judgement of
degree.

The rule records, per excluded series, the later publication that superseded it
and the words it did so in. A supersession asserted without that citation is not
this rule being applied; it is somebody's reading of the literature, and it is
refused.

## Stated uncertainty above a threshold

Rule `stated-uncertainty-above-threshold`. A point whose stated uncertainty
exceeds the threshold is excluded.

The threshold is not fixed in this file yet and the reason is worth more than
the number. Where it may come from is settled here, because that is the part
that cannot be added later.

It may be read from the distribution of stated uncertainties in the compilation.
That distribution is a property of the measurements and of how their authors
reported them, and looking at it is looking at the data's metrology. It may not
be read from residuals, from a held out error, or from any run of the fit. Those
two are easy to conflate once the data is in reach and the second one arrives
wearing the wording of the first, so the distinction is written down while
nothing has been fitted and nobody has a preference yet.

Fixing the threshold is an amendment to this file. It carries the command that
produced the distribution it was read from, and it is made before the selection
is run against a fit. If it is ever fixed after a fit exists, this file says so
at the number rather than presenting it as one that was chosen blind. A number
whose provenance is unstated is worse here than a missing one, because a missing
one is visibly missing.

A point with no stated uncertainty at all is not covered by this rule and is not
a point with a large uncertainty. Absence and a number are different states, and
the representation the compilation is read into keeps them apart for that
reason. What is done with an absent uncertainty is a separate decision and is
not made here, because making it here would settle it by implication in the one
place a reader would not look for it.

## Outside the energy range the fit claims

Rule `outside-claimed-range`. A point outside the energy range the fit claims to
cover is excluded from the fit.

The justification is not about the point. A fit that claims a range and is
fitted against measurements outside it has been shaped by data it does not
answer for, and the coefficients a reader uses inside the claimed range then
carry the pull of points the claim excludes.

The bound is deliberately not written here. The range the fit claims is fixed by
the work that decides the functional form, and a range written in this file
would be this file deciding it by accident. When that range is fixed, recording
it here is an amendment naming the claim it came from.

One thing about the bound is fixed now, because it is the direction this rule
can go wrong. The range comes from what the fit claims to cover, and never from
where the fit turns out to do badly. A range narrowed after residuals were seen
is the forbidden rule above with a different name on it.

## A technique with a systematic the compilation does not correct for

This class has no members and cannot have any yet. It is named here so that its
absence is visible rather than silent, and so that the requirements a member has
to meet are fixed before any member is written.

A rule in this class names four things. The technique, exactly as the
compilation names it, rather than by a name chosen here. The systematic, stated
as what it does to a measured stopping value and in which direction. The
published source establishing that systematic, which is a citation and not a
recollection. And that the compilation does not itself correct for it, which is
a statement about the compilation and has to be read out of it.

Any of the four missing leaves a preference about techniques rather than a rule,
and a preference about techniques correlates with agreement with the model
closely enough that the difference stops being visible.

The technique vocabulary is the compilation's own and is not in reach until the
compilation is. Writing members now against invented technique names would
produce rules rewritten the day the real names arrive, and a rule rewritten
after the data has been seen is what the order of this file exists to prevent.

## The counts, and the route to run without a rule

Turning any single rule off and rerunning is one command, and the counts of
points excluded by each rule are reported with the command that produced them.
Neither is in this file. Both need a selection running over fetched data and a
fit to rerun, and there is no data and no fit at the commit this landed at.

What is fixed now is what those counts are for. A rule excluding a large share
of the compilation is not thereby wrong, and a rule excluding almost nothing is
not thereby harmless. The count is what makes the argument about a rule possible
rather than what settles it.

## What no check can enforce

Nothing in this repository can refuse a rule that was tuned. The order these
rules were written in is not a property of the tree, and there is no artefact a
check could read that tells a rule fixed blind from one adjusted until the fit
improved. The wording of a tuned rule is the wording of an honest one.

So it is written here, where somebody about to do it would be reading, and the
only thing standing behind it is the commit this file was landed at.
