# 0005. Units, physical constants and the numeric conventions

Issue: #5
Status: accepted
Date: 2026-08-08

## What this record is

The internal units, what the type system is asked to do about them, where
conversion happens, and the numeric conventions that decide what a result means.

Unit confusion is the largest silent error class in a code like this, because a
factor of a thousand in a stopping power arrives looking like a physics
disagreement rather than a mistake. Data will arrive in eV, keV and MeV, in eV
per angstrom and keV per micrometre, in grams per cubic centimetre and in atoms
per cubic centimetre. None of that reaches the middle of the program.

## One internal unit per quantity

Energy is in electronvolts. The smallest energies that matter physically here
are a few eV, at the displacement threshold, the surface binding energy and the
low energy cutoff, so eV keeps every energy in the problem within a handful of
orders of magnitude of one and makes a wrong power of ten visible rather than
plausible.

Length is in angstrom. Interatomic spacings, screening lengths and the depths a
keV ion reaches are all naturally written in it, and the alternative that would
be tidier for output, the nanometre, would put a factor of ten into the middle of
the scattering geometry to save one at the edge.

Number density is in atoms per cubic angstrom. Mass density is not an internal
quantity: it is what an operator supplies and what a table holds, and it is
converted to a number density where it enters. What the operator gave and where
it came from still travels to the result, which #29 requires, but the transport
never sees it.

Electronic stopping cross section is in eV multiplied by square angstrom, per
atom. That unit is chosen for one reason: multiplied by a number density in
inverse cubic angstrom it gives eV per angstrom directly, so the energy loss
along a path carries no conversion factor at all. The conversion factor is where
this error class lives, and a unit pair that needs none removes it rather than
guarding it.

Scattering cross section is in square angstrom, for the same product.

Angle is in radians. Which axis an angle is measured from and which sense is
positive is a geometry question and belongs to #52; the unit is settled here so
that two parts of the geometry cannot disagree about it while agreeing about the
convention.

Atomic mass is in unified atomic mass units. Binary collision kinematics uses
mass ratios, so nothing is gained by carrying a mass in energy units, and the
number in the code stays the number in the table it was looked up from, which is
what makes the provenance record checkable by eye.

There is no internal time unit, because this calculation has no clock: a history
is a sequence of collisions and not a trajectory through time. The absence is
recorded so that nothing invents one, and a later feature that genuinely needs a
time supersedes this paragraph rather than adding a unit quietly.

## Typed quantities rather than bare floating point

The quantities above are distinct types wrapping a floating point number, not
aliases for it. There is no conversion from a bare number that the compiler will
perform on its own: construction goes through a named function that states the
unit, and taking the number back out goes through a named accessor that states it
again.

Arithmetic is implemented only where it means something. An energy plus an energy
is an energy. A stopping cross section multiplied by a number density is an
inverse length, so that product exists and returns the right type. An energy plus
a length does not compile, which is the whole return on this decision.

The cost was judged as follows. There is verbosity at every construction and
every extraction, there is boilerplate for the operators, and the inner loop
reads worse than the same arithmetic on bare numbers would. Against that, the
error class becomes a compile error rather than a plot that looks slightly wrong,
and in a code whose output is a number somebody puts in a paper that trade is not
close.

The boundary is every signature, every struct field and every stored value. A
function body may take the bare number out for a local expression, and because
extraction is a named call the departure is visible on the line where it happens
rather than implied by an absence.

A units library was considered and rejected. The one this ecosystem would reach
for brings a dependency and a large type level apparatus into a tree whose
physics crate declares none at all: `crates/core/Cargo.toml` has an empty
dependency table and `crates/core/tests/dependencies.rs` refuses an entry that is
not on an allowlist which is currently empty. The quantity set here is small
enough to write by hand. What would reverse this is the quantity set growing past
what a reader can hold in mind, or a need for dimensional analysis over products
this project cannot enumerate in advance.

## Physical constants

One module holds every physical constant. Nothing else in the tree defines one,
and that module is the authority for which constants exist rather than any
document, including this one.

Each constant carries the source it was taken from and the revision of that
source, which for the fundamental constants means the CODATA adjustment year
rather than the year somebody read it. Two constants that must be consistent with
each other, such as a charge and a permittivity that only ever appear as a
product, are held as the product they appear as, so no combination in the code
can be assembled from two different adjustments.

The rule is written this way because the failure it prevents is not a wrong digit
in the module. It is the second copy: a factor typed into an expression because
reaching for the module was inconvenient, which is then never revised when the
module is.

## Floating point precision and non-finite values

Every quantity is double precision. There is no single precision path. The range
this code spans, from a stopping power at eV energies to one at MeV energies, and
the accumulation of many small contributions into one tally, are what that buys;
a second precision would mean a second numerical answer to validate for a speed
gain nobody here has measured. What would reopen it is a measurement showing the
core bound by the bandwidth of the tally arrays rather than by arithmetic, and
that measurement belongs to the performance milestone.

A non-finite value produced inside the calculation is a defect and not a
condition. It stops the run where it was produced, with the site named, rather
than propagating. A NaN passes silently through every comparison and turns a
tally into a NaN for the rest of the run, so the only cheap place to find it is
where it was made. Under #13 that is a condition detected during a run whose
disposition is to stop, and 0013 records it as one.

No compiler flag that permits reassociation, contraction or flushing denormals to
zero is set for this workspace. Reassociation is precisely what the fixed
accumulation order below forbids, so a flag that grants the compiler permission
to reassociate makes the promise in 0006 unprovable rather than merely
approximate.

## Order dependent reductions

A reduction whose result reaches a reported number may not depend on the order it
was performed in. Floating point addition is not associative, so a tally summed
in the order histories finished moves with the thread count even when every
history consumed exactly the right random numbers. 0006 fixes the accumulation
order by history index for that reason and this record does not restate the
mechanism.

Where a reduction is genuinely order independent it may be done in any order, and
that is exactly the integer cases: counts of histories, counts of raised
conditions, unions of sets of identifiers. Anything holding a floating point sum
is not one of them.

## Where conversion happens

Conversion happens where data enters and where it leaves, and nowhere else. Data
enters through the compilation parser in the fit crate and the input document in
the command line crate; it leaves through the result writer in the command line
crate. The core holds no conversion factor.

That last sentence is the checkable half of this decision, and it is checkable
only because it is absolute: a factor in the core is a violation whatever its
value, which a check can read, whereas whether a factor is correct is not.

## The module, and what refuses a second copy

`crates/core/src/constants.rs` is the module. `xtask::physical_constants` is the
leg of the gate that reads it and refuses a physical constant written anywhere
else, so the rule above is enforced rather than asked for.

It reads a literal against the precision it was written to rather than against
the module's digits one by one. A value carrying four significant digits is
refused when it agrees with a constant to four, which reaches the copy that was
rounded as well as the copy that was truncated; a digit by digit comparison
would reach only the second, because the Bohr radius correctly rounded to four
digits shares three of them with the module's literal. The magnitude is dropped
before the comparison, so the same constant written in another unit is refused
too.

The leg fails closed in both directions. A constant declared with no source or
no revision above it is refused where it is declared, and a tree with no module
at all is refused rather than passed, because a leg that found nothing to
compare against would otherwise go green having checked nothing.

Three things it does not reach, and they are the residual rather than the whole.
A value written to fewer than four significant digits cannot be told from an
ordinary number, so it is not policed anywhere in the tree and a constant that
short is refused at its declaration instead. A constant folded into another
number before it was typed, or assembled by arithmetic from two others, is
invisible to it. And it reads Rust sources only, so a value quoted in a document
is prose about the module rather than a second definition of it, which is what
lets this record quote one.

What the module holds is small, because the physics has not landed. Each
constant in it is one an accepted record already asks for, and a constant
nothing accepted asks for is not added in advance. The set grows with the
physics; what the leg holds is that it grows there and nowhere else.
