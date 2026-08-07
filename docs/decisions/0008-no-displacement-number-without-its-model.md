# 0008. No displacement number without its model

Issue: #8
Status: accepted
Date: 2026-08-07

## Decision

This program never emits a quantity called dpa.

It emits named quantities. Each one carries the name of the model that produced
it and the parameters that model needed, and they are emitted side by side. A
consumer who wants a single number chooses one explicitly, and the name they
chose travels with the number into whatever they write.

## Why

The same irradiation yields different displacement numbers depending on which
model produced them, and the difference is not a rounding matter. The
publication that introduced the corrected measures states it directly:

    the number of radiation defects produced in energetic cascades in metals is
    only ~1/3 the NRT-dpa prediction, while the number of atoms involved in
    atomic mixing is about a factor of 30 larger than the dpa value

Kai Nordlund et al., Improving atomic displacement and replacement calculations
with physically realistic damage models, Nature Communications volume 9, article
number 1084, 2018. Quoted from the abstract of that article. The sentence is
quoted rather than paraphrased because the two figures in it are the whole
argument for this policy, and a paraphrase of a factor is how a factor gets
rounded.

The standard the quotation is about is Norgett, Robinson and Torrens, A proposed
method of calculating displacement dose rates, Nuclear Engineering and Design
volume 33, pages 50 to 54, 1975. The Nordlund article describes it as the
current international standard for quantifying energetic particle damage, which
is why it is the measure most numbers in the field are reported in and why
dropping it is not an option either.

So a number reported as dpa with no model named is not a measurement of
anything. It is a number that could be a factor of three away from a differently
labelled number in the next paper, with nothing in either paper saying which.
This program is in a position to refuse to add to that, and refusing costs it
almost nothing.

## What is reported

The set below is what the damage milestone implements. Every entry is emitted
under its own name, and every entry carries the parameters it used into the
result document, not just their values but the source they came from.

The simulated displacement count. The displacements the binary collision cascade
actually produced, counted by the program rather than computed from a formula.
Parameters: a displacement threshold energy for every element in the target.
Source: the published spread of threshold energies, which is #34, and the
threshold used is recorded with the result because the published values for one
element differ between sources by more than the precision anybody reports.

The damage energy. The part of the initial energy that went into nuclear motion
rather than into electrons, which is the input every formula below needs.
Parameters: the partition itself. Source: the partition work in #67, which
records which form it used.

The standard displacement measure of Norgett, Robinson and Torrens. Parameters:
the damage energy and a displacement threshold energy per element. Source: as
above for the threshold, and the 1975 article for the model. This is the one
that lets a result here be compared with the existing literature, and its
assumptions are stated with it rather than left to the reader.

The recombination corrected displacement measure of Nordlund et al. Parameters:
the damage energy, a displacement threshold energy, and two material dependent
parameters that the 2018 article fits and tabulates. Source: that article, for
the materials it covers.

The replacement measure from the same article, which counts atomic mixing rather
than surviving defects. Parameters: as above, with its own material dependent
pair. Source: the same article.

For a material the published fits do not cover, the corrected measures have no
parameters. The program then reports that they were not computed and why, and it
does not substitute the parameters of a chemically similar element. A
substitution made silently is a number with a source that is not its own, which
is the failure this whole record exists against.

## Three consequences that bind other work

The result schema has no field that could hold an unlabelled displacement
number. Not a field named dpa, and not a general numeric field into which one
could be written without a model beside it. The schema work is #7 and the
machine readable output is #96, and both are bound by this.

The command line has no flag that collapses the set to a single number without
naming which model produced it. An operator may ask for one measure, by its
name, and what comes back is still labelled.

The documentation states what each model assumes and where it stops holding, so
that the choice a user makes is informed rather than alphabetical. That is
#102, which names this record as one of the limits the operator guide has to
carry at the point the reader meets the number.

## What this record does not decide

Whether the documentation goes further and recommends one of these measures is
not decided here. That is a position on an argument that is live in the
literature rather than an engineering call, and it is an entry in the maintainer
decision issue, #1.

## What this record does not yet carry

The issue behind this record also asks that the output schema have no field
capable of carrying an unnamed displacement number, and that a test assert it.
There is no schema and no test at the point this record was written. The schema
is #7 and the machine readable output is #96, and the assertion is owed there.
This record states the constraint; it does not claim it is enforced.
