# 0003. What is fitted here and what is inherited

Issue: #3
Status: accepted
Date: 2026-08-08

## What this record is

The line between the parts of this calculation that are taken from the field
unchanged and the part that is replaced. It is written before the transport core
exists, because a core written against a table of stopping values and a core
written against a fitted model that carries an uncertainty are different
programs, and the second one cannot be retrofitted cheaply.

Nothing here is a measurement. There is no fit and no core at the point it was
written, and the record is the standard both are judged against.

## The line

The analytic parts are inherited. The fitted part is replaced. Numbers that are
neither are looked up and carry their source.

## Nuclear stopping is inherited

The momentum transferred to target nuclei comes out of a screened Coulomb
potential and the classical scattering integral. Both are inherited.

The reason is that they are functions rather than tables. A reader can evaluate
the screening function at any argument, differentiate it, and see which
assumption each term carries. That is the property the kickoff's objection is
about: the defect named there is not that a thing is empirical, it is that
numbers are published which nobody can regenerate. A function somebody can
evaluate does not have that defect, and refitting it would spend the whole
project's effort on the half of the physics that was never the problem.

One qualification belongs here rather than in a footnote, because leaving it out
would make this record read better than the truth. The universal screening
function is itself the product of a fit, to computed interatomic potentials
rather than to measurements. Calling it analytic is a statement about its form
and not about its origin. What makes inheriting it acceptable is that its input
is not data this project holds or could refit against, and that its form is open
to inspection; what would move it across the line is this project coming to hold
that input, which is not foreseen. The source of the coefficients and its
revision are recorded where the function is implemented, which is #50, rather
than asserted here.

The approximation to the scattering integral is not covered by this inheritance.
The integral has no closed form and the field uses an analytic approximation to
it, which is a fit one level down and out of sight. #50 computes the integral
numerically as the reference and admits the fast approximation only against it.
Inheriting the potential and the integral does not license inheriting an
approximation nobody in this tree has checked.

## Electronic stopping is replaced

The energy given to the target's electrons is refitted here, against the public
experimental compilation, with the functional form, the data selection, the
objective and the fitting procedure all in the tree and runnable in one command.
This is the contribution. Everything in the fit milestone exists to serve it.

Outside the high energy limit there is no closed theory covering all ions in all
targets across the range this tool has to serve, which is why the incumbent
parametrised the gap and why this project refits it rather than deriving it. The
high energy limit is a constraint the fit has to satisfy rather than a region
that is fitted, which is #38, and satisfying a theoretical constraint is not the
same act as inheriting a coefficient.

## Looked-up constants are neither

Atomic numbers, atomic masses, natural isotopic abundances, tabulated densities,
displacement threshold energies and surface binding energies are looked up. They
are not this project's contribution and no version of this project would improve
them by refitting them. What they owe instead is provenance: the compilation they
came from and its revision, under the rule in #11, with the element and isotope
data in #29 and the damage parameters in #34.

Looked up is a real third category and not a way of avoiding a decision. The
difference from an inherited function is that a constant cannot be evaluated or
differentiated by a reader, so the only thing that makes it checkable is the
citation, and the rule that enforces the citation is what this category is for.

## There is no fourth category

Every number that enters a calculation here is fitted from the compilation,
inherited as part of a function whose form is open, or looked up with a source.
A parameter that is none of the three is one that came from the literature
without a citation, and that is the state this project exists against.

The case that will test this is the effective charge treatment in #41, whose own
parameters may be fitted here or taken from elsewhere. This record does not
decide which, because that is #41's decision and it depends on what data the
compilation holds for heavy ions. What this record settles is that the third
answer is refused: a parameter absorbed into the code with no source and no fit
behind it is not admissible whichever way #41 goes.

## What would count as failing the kickoff's argument

If the electronic stopping this program uses cannot be regenerated from the data
and the code in this repository by one command, then this project reproduced the
object it set out to replace, whatever else it achieved.

That sentence is here so that a later reader can check the outcome against the
argument rather than against the effort. Adopting published coefficients as a
starting point for the fit is not the failure; shipping numbers whose derivation
is not in the tree is.

## The interface between the transport core and the stopping model

The core asks for a value. It never indexes.

The core holds something that answers one question: for this ion in this target
material at this energy, what is the electronic stopping cross section. The
answer carries its own uncertainty and any condition raised in producing it, so
neither can be dropped by a caller who forgot to ask for it. The core holds no
energy grid, does no interpolation, and has no type that can be indexed by an
energy.

The shape is a trait declared in `bremsweg-core` and implemented outside it. The
core declares no dependency and reads no file, which `crates/core/src/lib.rs`
and its dependency test already hold it to, so an implementation of that trait
cannot smuggle a file read into the core. The coefficient set is read by the
command line or by the fit crate and handed in.

Three consequences are the reason for the shape rather than side effects of it.
Selecting a different coefficient set is not a change to the core. A value that
rests on scaling rather than on measurement can be marked where it is produced
and travel with the value, which is what #41 needs from the result. And the
uncertainty is part of the value, so #10's separation of the statistical from the
coefficient contribution has something to separate.

Nuclear stopping is not behind this interface. It is computed inside the core
from the screening function, which #50 keeps replaceable as a component rather
than injected as data.

## A precomputed grid is a cache, not a table

Evaluating the fitted form in the inner loop may turn out to cost more than the
project can pay, and the ordinary answer is a precomputed grid. That is allowed
and it is not a return to a table, on one condition: the grid is built at run
time from the model, inside the implementation of the interface above, and it is
never a tracked artefact that anything else can read.

The difference is which object is the source of truth. A grid derived from the
model on every run cannot drift from it. A grid committed to the tree is a table,
whatever it is called, and the first time somebody edits it by hand this project
has the defect it was built to remove. Where such a cache exists, its agreement
with the model it came from is a check rather than a comment.

## What this record does not yet carry

The interface above is a shape and not code. `crates/core` holds a placeholder
function and nothing else, so nothing in the tree yet implements the trait, and
this record does not claim the boundary is enforced anywhere.

What owes it: #35 for the functional form the implementation evaluates, #43 for
the coefficient file it is constructed from, #50 for the screening function as a
replaceable component, and the transport milestone for the core that asks the
question. The check that would refuse a table crossing the boundary does not
exist and is not named here as an issue, because until there is a boundary there
is nothing for it to read.
