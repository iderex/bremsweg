# 0035. The functional form of the electronic stopping fit

Issue: #35
Status: accepted
Date: 2026-08-09

## What this record is

The shape the electronic stopping cross section is fitted in: the variables it
is written in, what is imposed at each end rather than fitted, how many free
parameters there are and what each of them is for, and what the shape cannot
represent.

There is no fit and no compilation in the tree at the point this record was
written. Nothing here is a measurement, no residual has been computed, and no
statement below is a report on how any form behaves against data. It is the
standard the fit is built to and judged against.

0003 puts electronic stopping on the replaced side of the line and says why:
outside the high energy limit there is no closed theory covering all ions in all
targets across the range this tool serves. This record decides what replaces it.

## The reference quantity and the variable

The fitted object is the proton electronic stopping cross section of one target
element, per target atom, in the unit 0005 fixes for it, as a function of proton
energy.

Protons rather than each ion separately, because the measurements are dense for
protons and sparse for everything else, and because a per ion fit would mean a
parameter set for combinations that have no data at all. Heavy ions reach this
object through the effective charge scaling in #41 and compounds through
additivity and its measured departure in #42, and neither adds a free parameter
of its own to a system.

Energy per nucleon is the variable the branches below are written in. It is the
variable both limits are naturally expressed in, since both are statements about
the projectile's velocity, and using it means the two constraints are imposed on
the same axis the fit is performed on rather than through a conversion.

## The form

Two branches, each of which is a physical limit, joined so that the region
between them is what the fit decides.

The low energy branch is proportional to the square root of the energy, which is
velocity proportional stopping. Its exponent is imposed and not fitted. The
theory is Lindhard and Scharff's, in which a slow ion moving through a free
electron gas loses energy at a rate proportional to its velocity. It holds below
the maximum and it says nothing about where the maximum is.

The high energy branch is the Bethe form: a logarithm of the projectile's energy
over the target's mean excitation energy, divided by that energy. Its prefactor
is imposed, because it is a theoretical constant rather than a fitted one, and
it is assembled from the constants module rather than typed. The mean excitation
energy inside the logarithm is looked up under 0003's third category, with its
source and revision, and is not a free parameter of this fit. #38 is where the
fit is held to this branch.

The two are joined by a reciprocal sum, generalised by one exponent. At the
exponent's neutral value this is the reciprocal join the field already uses: the
smaller branch dominates, the crossover between them produces the maximum, and
neither limit is disturbed. The generalisation lets the crossover be sharper or
broader without moving either end, so the shape of the maximum is something the
data decides and the ends are not.

Written out, with `s` the stopping cross section and `e` the energy per nucleon:

    s_low(e)  = a1 * sqrt(e)
    s_high(e) = (bethe_prefactor / e) * ln(1 + shell / e + e / excitation)
    s(e)      = ( s_low(e)^(-p) + s_high(e)^(-p) ) ^ (-1/p)

`bethe_prefactor` and `excitation` are not fitted. `a1`, `shell` and `p` are.

## Three free parameters per target element, and none shared

Three, and the argument for three is what the measurements determine
independently. The data fixes the slope of the low energy rise, the height and
position of the maximum, and how quickly the curve settles onto its high energy
limit from below. Three features, three parameters, and each of the three below
is the handle on one of them.

`a1` is the amplitude of the velocity proportional branch. Its exponent is the
theory's and only its size is this target's.

`shell` is the term inside the logarithm that carries the departure from the
bare Bethe form as the projectile's velocity falls towards the target's electron
velocities. It is the one place in the high energy branch where a fitted number
is admitted, and it is admitted because that departure is what no closed theory
covers over the whole range here.

`p` is the sharpness of the crossover, and it is the parameter that shapes the
maximum. It exists so that the maximum is fitted rather than being whatever the
join happened to produce, and it is bounded away from values at which either
branch is disturbed outside the crossover.

Nothing is shared across target elements. Each element carries its own three,
because a parameter shared between two elements is a claim that their electronic
structure agrees in a way the compilation is not being asked about. Sharing in
this fit is across ions and across compounds, in #41 and #42, and it is
structural rather than a shared coefficient: those relations carry an existing
parameter set to a system that has none, and add nothing per system.

The count is the substance of this decision and not a detail of it. Too many
parameters and the constrained form is the flexible interpolant below in
disguise, with the constraints becoming decoration on a curve that follows the
data wherever it goes. The ceiling is the number of features the data determines
independently, and a fourth parameter has to name the feature it is for.

## The rejected alternatives, and what would reverse each

A globally flexible interpolant in reduced variables, fitted with a smoothness
penalty. It follows the data more closely wherever there is data, which is
visible in every residual plot, and it costs everything outside the data: it
extrapolates arbitrarily, it can absorb a systematic error in one dataset as
physics, and a reader cannot separate its wiggles into measurements and basis
functions.

What would reverse it is a measurement rather than a preference: the constrained
form missing the maximum, for a majority of the systems with the best data, by
more than those measurements disagree with each other. #37 is where that number
is produced and it is not produced here. The reversal would also have to carry
the limits as hard constraints on the interpolant rather than dropping them,
because the extrapolation cost is the reason for the rejection and it is not
paid by fitting better.

A purely data driven model of the kind the recent literature has produced for
this database. It is a legitimate research direction and it is the wrong
contribution here: the objection this project exists over is that numbers are
published which nobody can regenerate or inspect, and replacing one uninspectable
object with another satisfies the letter of an open re-fit and none of its point.

What would reverse it is a change in what such a model is, not in how well it
fits. If a model of that kind became inspectable in the sense the objection
means, so that a reader can see which assumption each part carries, the argument
against it here goes away. Fitting better does not reverse it and neither does
the field adopting it.

## What this form cannot represent

Stated here so that a later disagreement with data is recognised as a limit of
the form rather than hunted as a defect in the code.

The periodic dependence of stopping on the projectile's atomic number. Heavy
ions carry no free parameters of their own here, reaching the fit through the
scaling in #41, so a systematic oscillation with projectile atomic number cannot
appear in this form at all. It is the sharpest limit of the three and #41 is
where its size is reported.

The periodic dependence on the target's atomic number is a weaker case. Each
element has its own three parameters, so an oscillation across the periodic
table is absorbed element by element rather than represented, and nothing here
predicts the value for an element with no data. #48 is where such an element is
named as one the fit is not reliable for.

A threshold at the low energy end. The square root branch is imposed, so a
target in which stopping falls away below some velocity, which is what a band gap
produces in an insulator, cannot be reproduced by this form. A fit forced to
represent one would do it by distorting `a1` for that element, which is a wrong
number that the residuals would show as a poor fit rather than as an absent
mechanism.

Structure sharper than one crossover. A second maximum, a shoulder, or a step
cannot appear, because the form has exactly one crossover and one exponent
shaping it.

Anything that depends on the target being a crystal. That is already outside by
0004, which fixes the amorphous target model, and it is repeated here only so
this list is not read as complete on its own.

## What this record does not yet carry

The issue behind this record asks for one thing beyond the positions above: that
the fit code makes the form replaceable rather than hard coded in the objective.

There is no fit code. What the requirement means here is recorded so it is not
reinvented later: the form is a component the objective is handed, so rerunning
the fit under a different form is a substitution rather than an edit, and the
sensitivity of the coefficients to this decision is something #39 can measure
rather than assume. Until that component exists, the paragraph above is a
requirement on a change that has not been made.

Every issue in the fit milestone is written against this record. That half of the
condition is met by the issues as they stand and is not a claim about code.
