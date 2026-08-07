# 0004. The target model, and what it rules out

Issue: #4
Status: accepted
Date: 2026-08-07

## Decision

Slowing down is computed in the binary collision approximation: a history is a
sequence of independent two body collisions with target atoms whose positions
are drawn rather than placed. This is the reason a calculation of this kind is
fast enough to be useful, and it is an approximation with a boundary. The
boundary is written down here, before the core exists, so that it is a stated
limit rather than a bug report.

Three assumptions follow, and each one is stated with the class of question it
makes this program unsuitable for, and with the observable that goes wrong and
the direction it goes wrong in. The directions below are the qualitative
consequences of the assumptions and are not measurements made here. Where a
number is wanted, it comes from the validation milestone against measured data,
not from this record.

## The target is amorphous

Target atoms are drawn from a density rather than placed on a lattice. There is
no lattice, so there is no crystal direction, no channelling, no dependence on
how a sample was cut and no dependence on the angle between the beam and a
crystal axis.

For an amorphous target this is not an approximation at all, it is the physical
situation. For a polycrystalline target with grains small against the range and
no texture it is usually close enough, and whether it is close enough is a
judgement the operator makes rather than one this program makes for them.

The class of question this makes the program unsuitable for is implantation into
a single crystal, which is most of semiconductor processing. It is also
unsuitable for anything where the answer is expected to change with the
orientation of the sample, because here it cannot.

The observable and the direction: the range distribution is too shallow, and it
lacks the deep tail that channelled ions produce. An ion travelling along an
open crystal direction sees a lower density of scattering partners than the
average, loses energy more slowly and travels further, and none of that happens
in a model with no directions in it. So a channelled implant is deeper than this
program says, the depth profile is missing its tail entirely rather than having
one that is slightly wrong, and the disagreement grows as the beam is aligned
closer to an axis. Near the aligned condition the difference is not a
correction, and the honest statement to an operator is that the calculation does
not apply rather than that it is approximate.

## Collisions are independent

Each collision is computed as if the two atoms involved were alone. Correlated
collisions, and the many body effects that matter once a region is hot and dense
with moving atoms, are outside the model.

This is the assumption that fails first as the recoil energy rises. At low
recoil energy a cascade really is a sparse sequence of separate collisions. At
high recoil energy it is a dense region where many atoms move at once, most of
the displaced atoms return to a lattice site, and treating the event as a chain
of independent pairs counts displacements that do not survive.

The class of question this makes the program unsuitable for is anything that
depends on what a cascade leaves behind rather than on how much energy it
deposited: the surviving defect population, defect clustering, the morphology of
a cascade, and any comparison with a measurement of stable damage.

The observable and the direction: a displacement count computed by chaining
independent collisions is too high, and the discrepancy grows with recoil
energy. This is the reason the damage milestone reports more than one
displacement measure and never a bare displacement number, which is recorded
separately in 0008.

## The ion has no memory of the target

The target does not change while the run proceeds. Damage produced by one
history does not feed back into the target seen by the next one, there is no
accumulated amorphisation, no sputtered surface receding, no implanted species
building up a concentration, and no stress or swelling.

A run is therefore a set of independent histories and the answer does not depend
on the order in which they were computed. That is what makes the parallelism in
this project trivial and the determinism promise in 0006 achievable, so this
assumption buys something concrete rather than only costing.

The class of question this makes the program unsuitable for is high fluence: any
case where the beam has changed the target before the beam is finished. High
dose implantation past the amorphisation threshold, sputtering deep enough to
move the surface, and any profile whose shape depends on the dose are all
outside it. The program computes a per ion result; multiplying it by a fluence
is the operator's step and it is linear by construction, which is exactly the
assumption being made.

The observable and the direction: the direction is not fixed by the assumption,
which is itself worth stating. Amorphisation of a crystalline target during
implantation shuts down channelling and makes the real profile shallower than a
low dose one, while a receding surface makes a measured profile shallower still
and removes the implanted species. Whether a stated per ion answer is high or
low for a given fluence depends on which of these dominates, so the honest
statement is that the result is a low dose limit and that the departure from it
grows with dose, rather than a claim about which way it goes.

## Which interfaces are kept free of the amorphous assumption

A crystalline mode is not planned and this record does not commit to one. What
it does commit to is that the amorphous assumption is not spread through every
interface, so that adding one later is a change in one place rather than
everywhere.

At the time this record was written the tree held no code, so what follows is a
constraint on the crates the scaffolding milestone creates rather than a report
on interfaces that exist. It is checkable at the point they exist, and the issue
that creates the transport interfaces is where it is checked.

The interfaces that stay free of it:

The target description. A target is composition, density and geometry. It does
not carry the statement that atoms are randomly placed, because that statement
belongs to the routine that produces the next collision partner and not to the
material.

The choice of the next collision partner. This is the one place where the
assumption lives, and it lives there deliberately. The routine takes a position,
a direction and an energy and returns a partner and an impact parameter. An
ordered target changes the body of that routine and nothing above it.

The scattering kernel. Two atoms, a potential, an energy and an impact
parameter, in and out. It knows nothing about how the partner was chosen and is
identical in an ordered target.

The energy loss along a path, the tallies, and the result document. All of them
consume positions and energies and none of them asks how the positions arose.

The interface that does not stay free of it, and why:

The free flight path. In an amorphous target the distance to the next collision
is drawn from a distribution set by the density. In an ordered target it is
determined by the geometry rather than drawn, so a crystalline mode does not
supply a different distribution to the same interface, it replaces the step. The
alternative would be an interface general enough to cover both, designed against
a mode nobody has specified, and a general interface built from one example is a
guess that later has to be changed anyway. This one is left specific and the
cost is named here so that whoever adds an ordered mode knows this is the seam
that has to be cut rather than extended.

## Where an operator reads this

The same three limits belong in the operator guide, in the place where a reader
meets the number each one affects, in language that does not assume they read
this tree. That is #102, which names this record as one of the limits it has to
carry.
