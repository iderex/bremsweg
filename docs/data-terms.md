# What the terms of the reference data allow

This project fits against somebody else's compilation and reports numbers
derived from it. This file records what the terms attached to that compilation
say, with the document each statement was read from and the date it was read. It
records a reading, not a decision. Where the terms are unclear it says so and
states the conservative reading, and it resolves nothing by assumption.

## How this was read, and one thing that could not be

Read on 2026-08-07.

The database pages were fetched directly:

    curl -sS -L https://www-nds.iaea.org/stopping/
    curl -sS -L https://www-nds.iaea.org/stopping/description
    curl -sS -L https://www-nds.iaea.org/stopping/versions
    curl -sS -L https://www-nds.iaea.org/stopping/api

The terms of use those pages link to could not be. The live page refused the
request:

    curl -sS -L -o /dev/null -w "%{http_code}" https://www.iaea.org/about/terms-of-use
    403

So the terms below were read from an archived copy taken on 2026-07-27:

    https://web.archive.org/web/20260727073123/https://www.iaea.org/about/terms-of-use

Quotations below preserve the wording and the line of argument exactly.
Typographic characters, meaning curly quotation marks and apostrophes, are
written as their plain equivalents, and line breaks are placed to fit the
column. Nothing else is changed and nothing is elided without saying so.

The archived text has not been compared against the live page, because the live
page could not be retrieved by the route used. Every quotation below is from
that archived copy. If the live terms have changed since the snapshot, this file
is describing the snapshot, and somebody who can reach the live page should
check it before anything is published on the strength of this reading.

## The compilation

The IAEA Electronic Stopping Power Database, at `https://nds.iaea.org/stopping`.
Created in 1990 by Helmut Paul at the University of Linz, taken over in 2015 by
the Nuclear Data Section of the International Atomic Energy Agency, and compiled
since December 2015 by Claudia Montanari.

The version current when this was read:

    Version 2026-01 - released on 28th of January, 2026
    4,440 Experiments | 64,612 Datapoints
    745 References | 3,126 Authors

Every page of the database carries the line `© 2026 IAEA.` and a link to the
IAEA terms of use. No page of the database states terms of its own, so the
site-wide terms are the only ones there are to read.

## Attribution

Required, and the database states the form itself:

    Cite the database

    IAEA Stopping Power Database, version 2026-01, https://nds.iaea.org/stopping

    Reference paper:

    "The IAEA electronic stopping power database: Modernization, review, and
    analysis of the existing experimental data" C.C. Montanari, P. Dimitriou,
    L. Marian, A.M.P. Mendez, J.P. Peralta, F. Bivort-Haiek, Nucl. Instrum.
    Methods Phys. Res. B 551 (2024) 165336,
    https://doi.org/10.1016/j.nimb.2024.165336

Read from `https://www-nds.iaea.org/stopping/` on 2026-08-07.

The version number is part of the citation the database asks for, which matches
what this project needs anyway: a coefficient set has to name the version of the
compilation it was fitted against, not the date somebody downloaded it.

The general terms require acknowledgement as a condition rather than as a
courtesy, and they attach a second condition to it:

    provided that appropriate acknowledgement of the IAEA as the source is given
    and that the IAEA's endorsement of users' views, products or services is not
    stated or implied in any way

Read from the archived terms of use on 2026-08-07. So anything this project
publishes has to cite the database in the form above and has to avoid any
wording that could read as the IAEA endorsing this program or its numbers. That
is a constraint on the readme and the operator guide as much as on a data file.

## Redistribution

This is where the terms pull in two directions, and the tension is the finding
rather than an obstacle to it.

The copyright clause is permissive and explicitly covers data:

    IAEA content of this Site is protected by copyright. To ensure wide
    dissemination of its information, the IAEA is committed to making its
    content freely available and encourages the use, reproduction and
    dissemination of the text, multimedia and data presented. Content may be
    adapted, translated, copied, printed and downloaded for private study,
    research and teaching purposes, and for use in commercial and non-commercial
    products or services, provided that appropriate acknowledgement of the IAEA
    as the source is given and that the IAEA's endorsement of users' views,
    products or services is not stated or implied in any way. Specific
    restrictions and/or conditions may apply to specific Materials within this
    Site.

Read as covering this database, that permits a copy inside a public repository,
with acknowledgement, including for use in a commercial product.

A later clause speaks about databases specifically and is narrower:

    Digital repositories and databases

    The data in the IAEA digital repositories and databases is provided free of
    charge for educational and informational use. The repositories and databases
    may contain data submitted by third parties, in which case the IAEA shall
    not be held responsible for copyright infringements related to the use of
    such data.

Both read from the archived terms of use on 2026-08-07.

`Educational and informational use` is narrower than `commercial and
non-commercial products or services`, and the copyright clause itself says that
specific restrictions may apply to specific materials. Whether the database
clause is such a specific restriction that narrows the general permission for
this database, or a description of what the repositories are for that sits
inside the general permission, is not something the text settles. Nothing on the
database's own pages resolves it either.

UNCLEAR. The conservative reading is the narrower one: the data is available for
educational and informational use, and a copy inside this repository is
defensible on that footing for study and research, while shipping the
compilation inside a commercial product on the strength of the general clause
alone is not. Under that reading, vendoring a copy for the tests and for
reproducibility is within the terms; presenting the vendored copy as something a
downstream product may take and sell is not, and this project has no way to
constrain that once a copy is in a public tree.

## Third party content inside the compilation

The compilation is a collection of measurements published by other people, and
the terms are explicit that the IAEA does not answer for what is inside it:

    This Site may include third party copyright material for which rights and
    permissions must be obtained from the copyright holder(s) indicated. The
    IAEA has made every reasonable effort to locate, contact and acknowledge
    rights holders and to correctly apply terms and conditions to Materials.
    Under no circumstances shall the IAEA be held responsible for any copyright
    infringements arising from the use of third party content.

Read from the archived terms of use on 2026-08-07.

The database is a compilation of numbers reported in 745 references by 3,126
authors. Permission from the IAEA is therefore not by itself permission covering
everything in the file, and the terms say as much. What that means in practice
depends on whether the individual measurements in a table are protected at all,
which is a question about facts and compilations rather than about these terms,
and it is not one this file answers.

UNCLEAR, and it is unclear in a direction that no permission from the IAEA can
fix.

## The fitted coefficients

The terms say nothing about derived work. The word does not appear in them.

The permission granted is to `use, reproduction and dissemination`, and
`Content may be adapted, translated, copied, printed and downloaded`. `Adapted`
is the closest the text comes, and a fitted coefficient set is not obviously an
adaptation of a table of measurements: it is a small set of numbers chosen so
that a function reproduces the table, and the table itself cannot be
reconstructed from it.

UNCLEAR. Two readings are available and the terms choose neither. Under the
first, a coefficient set is a derived work of the compilation and the
acknowledgement condition travels with it. Under the second, it is a set of
parameters measured from data the way any published fit is, and the obligation
is the ordinary scientific one of citing what was fitted against.

The conservative reading is the first, and it costs nothing to follow: cite the
database, with its version, wherever coefficients produced from it are
published, whatever the legal answer turns out to be. That is what this project
should do regardless, because a coefficient set whose provenance is not stated
is a set of numbers nobody can check, which is the failure this whole project
exists against.

What the conservative reading does not settle is whether the coefficient file
may carry a licence of its own. That question is entry 1 of #1, and it is open.

## Other external data sources

There are none in the tree yet:

    git ls-tree -r --name-only origin/main -- data/
    data/README.md

That file holds prose and no data. When a source is added, it gets its own
section here before it is added, not after.

## What this file does not decide

Whether a copy of the compilation is redistributed inside this repository or
fetched at build time is entry 3 of #1 and is open. This file gives that entry
the reading it was waiting for, including the two places the reading runs out.

Nothing here is legal advice, and the person who reads it should notice that the
one document everything turns on could not be retrieved from its own site by the
route used.
