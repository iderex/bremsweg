# 0002. The means: language, toolchain and workspace

Issue: #2
Status: accepted
Date: 2026-08-07

## Decision

The tree is written in Rust. One Cargo workspace holds the physics library, the
fit and the command line. The toolchain is pinned by a file in the tree and the
dependency set is pinned by a lock file that is committed.

The concrete values:

Language: Rust, 2024 edition, with the workspace dependency resolver that
edition selects. The newest edition is chosen because this is a tree with no
history to migrate, so the migration cost that normally argues for an older
edition is zero here, and an edition behind on the first commit is a migration
somebody pays for later with no benefit taken in the meantime.

Minimum and pinned version: 1.97.0. The pin lives in `rust-toolchain.toml` at
the workspace root, and the minimum supported version recorded in the workspace
manifest is the same number. They are the same number deliberately, because two
numbers that are allowed to differ are two numbers nobody keeps in step. Raising
the pin is a change with an issue behind it, like any other.

Build tool: Cargo, the one the toolchain ships. It supplies the lock file, the
test command, the benchmark harness and a linter that can be configured to fail
on a warning, so the gate work does not have to bring its own.

Lock file: `Cargo.lock`, committed at the workspace root, for a workspace that
produces a binary. Builds in the automated run resolve nothing that is not in
it.

Workspace layout, at the root of the repository:

    Cargo.toml              the workspace manifest and the shared dependency set
    Cargo.lock              committed
    rust-toolchain.toml     the pinned toolchain
    crates/core/            the physics: no input, no output, no file system
    crates/fit/             the fit: reads measurements, writes coefficients
    crates/cli/             argument parsing, file reading, result writing
    data/                   holds no code
    docs/decisions/         these records

The crate names are `bremsweg-core`, `bremsweg-fit` and `bremsweg-cli`, and the
binary the command line crate produces is `bremsweg`.

The boundary that matters is between `core` and everything else. The physics
crate has no dependency that opens a file, a socket or a terminal, which is what
makes the whole suite runnable with no display, no temporary directory and no
elevated privilege. That is a property a check can refuse, and #14 owes the
check.

## What is forced from outside this repository

The workflow file format is forced. The automated run happens on a platform that
reads YAML from `.github/workflows/`, and nothing about that is this project's
choice. It is held to its smallest surface, which is #16's requirement that the
workflow invoke the gate command and contain no logic of its own.

Nothing else here is forced. Rust is chosen, not imposed. The operating systems
that matter are chosen by where the audience already is rather than by any
dependency, and Windows is on that list because a large part of the incumbent's
users are there.

## The four questions

Can the means carry a property a machine can refuse, a proof that runs, and a
claim that cites the command behind it? Yes to all three. `cargo test` is the
proof that runs and it is one command. A refusable property is an ordinary test
or a lint configured to deny, and `cargo clippy -- -D warnings` turns a warning
into a refusal. A claim about this tree is backed by a command anyone with the
toolchain can repeat, and the toolchain being pinned is what makes the repeat
give the same answer.

Is anything outside this repository forcing it? No, and the section above says
what is forced instead.

Does it add a language, a runtime or a dependency the tree does not already
carry, and is that cost paid knowingly? It adds a language to a tree that today
holds Markdown and workflow YAML, so this is the moment the cost is taken on
rather than a moment it is increased. The cost is named in the next section and
is not small. What it does not add is a runtime: the artefact an operator
receives is a binary with no interpreter and no environment behind it.

Is the result testable by the suite that will exist, or does it need a parallel
apparatus nobody maintains? One suite, `cargo test`, covering the library, the
fit and the command line, because they are three crates in one workspace rather
than three projects. The one thing that does need a second harness is the set of
tests that need particular hardware or the network, which is #22, and that
harness is named for the reason it is separate rather than being an escape hatch.

## Why, against this project rather than in general

Four things pull at once here and they pull in different directions.

The transport core is a long numeric loop that has to give the same answer
twice. A compiled language with no garbage collector gives it predictable timing
and no pauses in the middle of a run. The ion loop is a set of independent
histories, which is the one shape where a data parallel library gives close to
linear scaling without the code being restructured around it.

The fit has to be reproducible by anyone who clones this repository. Putting it
in the same workspace and the same language as the code that consumes its output
means the published coefficients come out of code that is built, tested and
gated exactly like the rest, rather than out of a script kept beside it.

The operator has to be able to run the result without installing an environment
first. The incumbent's users are running a Windows executable under Wine today,
and a replacement that asks them to install a language runtime first is a
replacement most of them will not take. A single statically linked binary is the
strongest answer available to that, and it is the same answer on Windows.

The whole thing has to be machine testable, headless, with properties a check
can refuse. That is the fourth pull and it is the one that rules out a means
where the proof would live outside the build.

## What it costs

The numeric ecosystem is thinner here than in the alternatives. Nonlinear least
squares, linear algebra and statistical tooling exist and are usable, but they
are younger, less documented, and some of what the fit needs will be written
here rather than pulled in. That is a cost paid in the fit milestone and it is
the cost most likely to be underestimated.

The audience for this project reads and writes Python, Fortran and C++.
Choosing none of those raises the price of an outside contribution from somebody
in the field who wants to correct the physics rather than the code, and the
whole argument for this project is that the fit should be auditable by such a
person. The mitigation is that the coefficients ship as a file that stands on
its own, which is #43, so auditing the numbers does not require reading this
tree at all.

Compile times on a workspace of this size are a daily cost to whoever builds it.

## Alternatives, and what would reverse this

C++ is the field's default and the one most reviewers here will expect. It costs
a build system this project would then own, a weaker default for dependency
pinning and for testing, and considerably more ways for undefined behaviour to
enter a numeric core, where a wrong answer looks exactly like a right one.

Fortran is what much of the existing transport code in this field is written in
and it is genuinely good at the inner loop. It costs nearly everything else:
packaging, dependency management, testing, and the machinery the gate work
depends on.

Python with a compiled kernel is the strongest of the rejected alternatives. It
gives the best available libraries for the fit and the widest audience for
auditing it, which is not a small thing when auditability is the point. It costs
a runtime the operator has to install, a two language build, and a
reproducibility story resting on an environment rather than on a lock file.

The condition that reverses this decision, stated in advance so that meeting it
is a measurement rather than a mood: the fit needs optimisation machinery that
does not exist in this ecosystem and that this project is not willing to write
and validate itself. If the fit work reaches that point, the honest move is a
two language split with the fit in Python and the core in Rust, and a
coefficient file as the boundary between them. That is a decision to take when
the evidence exists, and taking it now would be guessing.

## Naming and numbering of the records that follow

Every decision record in this repository is a file at
`docs/decisions/NNNN-short-title.md`. `NNNN` is the number of the issue that
produced the record, zero padded to four digits. The short title is lower case
with hyphens and describes the decision rather than the issue.

The number comes from the issue rather than from a count of the records already
in the tree, and the reason is collisions. A number derived from the highest one
present is chosen against the tree the writer happens to have, so two records
prepared from the same starting point choose the same number and one of them has
to be renumbered after it was written and referred to. The tracker already hands
out a number that is unique by construction, and no work here starts without an
issue, so every record already has one. The cost is that the numbers are not
consecutive, which is a cost paid in reading order and nothing else.

The rest of the shape:

One decision per file. A file that records two decisions is a file that gets
half superseded later.

Every record opens with the same four lines: a title beginning with the number,
the issue it came from, a status, and the date the decision was taken.

Status is `accepted`, `superseded by NNNN`, or `withdrawn`. A record is never
edited to reverse its own decision. It keeps its number and gains a status line
naming the record that replaced it, so that a reader arriving from an old
reference finds the change rather than a document that quietly disagrees with
what they were told.

The body says what was decided, what it costs, what was rejected, and what would
reverse it. A record with no cost section is a record where the cost was not
looked for.

Records are written in English, in ordinary prose, and a number in one carries
the source it came from.

## Where this stands against the tree

No issue in the scaffolding milestone had been started when this record was
written, which is what the issue asked for. The tree at that point was commit
e62fb22 and it held no code at all. The command is pinned to that commit so it
gives the same answer after this record lands:

    git ls-tree -r --name-only e62fb22
    .github/workflows/dco.yml
    .github/workflows/dependency-review.yml
    .github/workflows/scorecard.yml
    .github/workflows/unicode-guard.yml
    .github/workflows/zizmor.yml
    NOTICE.md
    README.md

So the scaffolding matching this record is a forward obligation on #14 and on
the issues beside it, not something this record can show. What it can show is
that nothing was built before the decision was written down. Where the
scaffolding departs from the layout above, the departure is recorded here rather
than left as a difference between a document and a tree.
