# The target gate, check by check

The quality parity program in this repository measures itself against a public
merge gate that already exists: the one on `iderex/jellyfin-plugin-sso`. That
repository is public and its workflow files are readable, so the target is a
fact somebody else can re-derive rather than a description of it.

This file is the list. Every check on the target has an entry, and every entry
carries a verdict. A check on the target with no entry here is the failure this
file exists to prevent, because it makes a parity claim untrue in the direction
nobody notices.

It is a snapshot. The target moves, and a list with no reference point silently
becomes wrong, so the commit it was read at is recorded and every count below
was produced by a command a reader can run again.

## What was read, and how

Read on 2026-08-08.

The target at commit `2b37832ae0525cc4f548b3b3b34a2a76381b9123`, committed
2026-08-07T22:52:49Z:

    git clone --depth 1 https://github.com/iderex/jellyfin-plugin-sso.git
    cd jellyfin-plugin-sso && git rev-parse HEAD
    2b37832ae0525cc4f548b3b3b34a2a76381b9123

Twenty-three workflow files:

    ls .github/workflows/ | wc -l
    23

The jobs in them were listed from the files rather than from memory, with the
events each file declares:

    python - <<'PY'
    import yaml, glob, os
    for p in sorted(glob.glob('.github/workflows/*.yml')):
        d = yaml.safe_load(open(p, encoding='utf-8'))
        on = d.get(True, d.get('on'))
        print(os.path.basename(p), list(on) if isinstance(on, dict) else on)
        for jid, j in (d.get('jobs') or {}).items():
            print('   ', jid, '::', j.get('name'))
    PY

That reads the target's YAML with a Python parser run once by hand. Nothing of
it is added to this tree.

The file list alone is not enough to establish that nothing is missing, for
three reasons this reading met in practice. A job name can be an expression, so
one job in the file becomes several checks on a commit. A check can be produced
by the platform rather than by a job, which no reading of the workflow files
finds. And a check can arrive on a commit from a workflow in a different
public repository that happens to carry the same commit.

So the derived list was compared against the checks two real pull request head
commits on the target actually carried:

    gh api repos/iderex/jellyfin-plugin-sso/commits/984b6ab55e2c1313ab7236667990a1e8a6cd3130/check-runs?per_page=100 \
      --jq '.check_runs[] | "\(.name)\t\(.conclusion)\t\(.app.slug)"' | sort
    gh api repos/iderex/jellyfin-plugin-sso/commits/3a1f181fe5a75c3d6e7865e21ee76e536c2fdc44/check-runs?per_page=100 \
      --jq '.check_runs[] | "\(.name)\t\(.conclusion)\t\(.app.slug)"' | sort

Both commits returned the same eighteen names. Two of them come from the app
`github-advanced-security` rather than from a job, and both have entries below.
A third, `submit-nuget`, is not the target's check at all: its details point
into a fork's action runs.

    gh api repos/iderex/jellyfin-plugin-sso/commits/984b6ab55e2c1313ab7236667990a1e8a6cd3130/check-runs?per_page=100 \
      --jq '.check_runs[] | select(.name=="submit-nuget") | .details_url'
    https://github.com/Flowfin/jellyfin-plugin-sso/actions/runs/31224768569/job/93016715709

That is a public repository which carries the same commit:

    gh api repos/Flowfin/jellyfin-plugin-sso/commits/984b6ab55e2c1313ab7236667990a1e8a6cd3130 --jq .sha
    984b6ab55e2c1313ab7236667990a1e8a6cd3130

A workflow run there reports onto the shared sha, so the check appears in the
target's list while belonging to somebody else's repository. It has an entry
below saying that and nothing else, because a name a reader will see on the
target's commits and cannot find in the target's workflow files is worth one
line.

Which of the checks the merge itself requires is a separate fact from which
checks run, and it was read from the ruleset rather than inferred:

    gh api repos/iderex/jellyfin-plugin-sso/rulesets --jq '.[] | "\(.id) \(.name) \(.enforcement)"'
    18802863 Protect main and 5.0 active
    gh api repos/iderex/jellyfin-plugin-sso/rulesets/18802863 \
      --jq '[.rules[] | select(.type=="required_status_checks") | .parameters.required_status_checks[].context]'

Thirteen contexts are required. Each entry below says whether it is one of them.

## How to read a verdict

Three verdicts, and they describe what is true in this repository now.

Adopted means the same property is enforced here.

Adapted means the property is enforced here by a different mechanism, because
this project is a different kind of program, with the difference stated.

Declined means the property is not enforced here, with the reason.

Almost everything below is declined, and that is the honest state of a tree that
carries no physics yet:

    git ls-tree -r --name-only 67f205334df2d588c2f31f927b3b7616a43f2d6a | wc -l
    28

Two kinds of declined are distinguished at every entry, because collapsing them
would hide the difference that matters:

Declined, no subject. The thing the check judges does not exist in this project
and is not planned to, so the property has nothing here to hold over. The
condition that would reverse it is stated.

Declined, not yet built. The property applies and is planned, but nothing
enforces it at this commit. The issue that owes it is named.

A verdict is not a promise. The issue named against an entry is what changes the
entry, and several issues in this milestone exist to do exactly that.

## What is already in force here

Verified at `67f205334df2d588c2f31f927b3b7616a43f2d6a`, the mainline head this
file was written against.

Four of the five guards ran on a real pull request head and were observed
rather than read out of the workflow files:

    gh api repos/iderex/bremsweg/commits/91f813569b3a6bfe8bfd23b9cf1969ac6177c4b1/check-runs?per_page=100 \
      --jq '.check_runs[] | "\(.name)\t\(.conclusion)"' | sort
    Audit workflows (zizmor)        success
    DCO sign-off                    success
    dependency-review               success
    Reject Trojan Source Unicode    success
    Reject Trojan Source Unicode    success
    zizmor                          success

`Reject Trojan Source Unicode` appears twice because the workflow declares both
`push` and `pull_request`, so the same job reports twice on a branch head.

The fifth does not run on a pull request here, the same as on the target, so it
was observed on the mainline head instead:

    gh api repos/iderex/bremsweg/commits/67f205334df2d588c2f31f927b3b7616a43f2d6a/check-runs?per_page=100 \
      --jq '.check_runs[] | "\(.name)\t\(.conclusion)"' | sort
    Audit workflows (zizmor)        success
    Reject Trojan Source Unicode    success
    Scorecard analysis              success

One difference from the target is worth stating here rather than at each entry.
The ruleset on this repository requires no status check at all:

    gh api repos/iderex/bremsweg/rulesets/20522721 \
      --jq '{rules:[.rules[].type], required:[.rules[]|select(.type=="required_status_checks")|.parameters.required_status_checks[]?.context]}'
    {"rules":["deletion","non_fast_forward","pull_request"],"required":[]}

So a guard that is adopted below is adopted in the sense that it runs and goes
red, not in the sense that it stops a merge. The target requires thirteen
contexts and this repository requires none. Nothing in this milestone owns that
gap, and it is stated here rather than left for a reader to notice.

Three entries below have moved since this section was verified, and the count of
five above is the count at `67f2053` rather than the count today. `CodeQL` and
`Analyze (actions)` are adopted and `Analyze (csharp)` is adapted, each carrying
its own observation at the commit it was verified at. This paragraph exists
because a section pinned to one commit goes on reading as current, and the
entries are the authority rather than this summary.

## The checks

### DCO sign-off

`dco.yml`, job `dco`. Walks every commit in the pull request and fails unless
each one carries a `Signed-off-by` line matching its author. Required by the
target's ruleset.

Verdict: adopted. In force here at `67f2053`, in `.github/workflows/dco.yml`,
which is the same check.

### dependency-review

`dependency-review.yml`, job `dependency-review`. Refuses a pull request that
introduces a dependency with a known vulnerability or a licence outside the
allowed set. Required by the target's ruleset.

Verdict: adopted. In force here at `67f2053`, in
`.github/workflows/dependency-review.yml`.

### Reject Trojan Source Unicode

`unicode-guard.yml`, job `bidi`. Refuses bidirectional overrides, isolates and
zero-width characters in tracked source, which is the class of attack where the
rendered text and the executed text differ. Required by the target's ruleset.

Verdict: adopted. In force here at `67f2053`, in
`.github/workflows/unicode-guard.yml`.

### Audit workflows (zizmor)

`zizmor.yml`, job `zizmor`. Audits the workflow files themselves and fails on
any finding at low severity or above, which includes an action pinned by a
movable tag. Required by the target's ruleset.

Verdict: adopted. In force here at `67f2053`, in `.github/workflows/zizmor.yml`.

That it runs is observed above. That it bites has not been proved here, and
issue #24 carries that gap rather than this file.

### zizmor, from the code scanning app

Produced by `github-advanced-security` rather than by a job. The SARIF that
`zizmor.yml` uploads becomes a code scanning results check under the tool's
name. Not required by the target's ruleset.

Verdict: adopted. In force here at `67f2053`, and observed on `91f8135` above,
because this repository's `zizmor.yml` uploads the same SARIF.

### Scorecard analysis

`scorecard.yml`, job `analysis`. Runs the OSSF supply chain scorecard on the
default branch and uploads the result to code scanning. It declares `push` to
the default branch, a weekly schedule and `branch_protection_rule`, and no
`pull_request`, so it produces no check on a pull request on either repository.
Not required by the target's ruleset, and it could not be: it never reports on a
pull request head.

Verdict: adopted. In force here at `67f2053`, observed on the mainline head
above.

### build

`dotnet.yml`, job `build`, which the file leaves unnamed so the job id is the
check name. Required by the target's ruleset. One job carrying ten separate
properties: a compatibility metadata check, a VEX document check, a locked
dependency restore, a scan for vulnerable transitive packages, a build with
warnings as errors, a locked restore and build of the fuzz harness, a replay of
the fuzz seed corpus, the unit tests, a coverage run, and a coverage bar over
the security surface.

Verdict: declined, not yet built. Nothing in this repository runs any of it on a
change, because no route here runs anything on a change. The gate command is
issue #15 and the workflow that would run it is #16. The properties inside it
are owed separately: the locked restore by #24, format and lint by #17, coverage
by #23, and the vulnerable dependency scan by #87.

The compatibility metadata check and the VEX check have no subject here and are
listed under their own entries below rather than folded into this one.

### ABI floor build

`dotnet.yml`, job `abi-floor`. Derives the oldest host version the plugin
declares support for and builds against it, so a change that silently raises the
floor is refused. Required by the target's ruleset.

Verdict: declined, no subject. This project is a standalone program and links
against no host application, so there is no floor to build against. The
condition that reverses it is this project ever shipping as a component loaded
by somebody else's runtime, which nothing in the plan proposes.

### Jellyfin compatibility metadata check

A step inside `dotnet.yml`, job `build`, running `scripts/check-jellyfin-compat.sh`.
It produces no check of its own and reddens `build`. Listed because the property
is distinct from the rest of that job.

Verdict: declined, no subject. It asserts agreement between the plugin's
declared host compatibility and its build metadata, and this project declares
compatibility with no host.

### VEX document check

A step inside `dotnet.yml`, job `build`, running `scripts/check-vex.py` over
`security/vex/openvex.json`. It produces no check of its own and reddens
`build`.

Verdict: declined, not yet built. A VEX document states which known
vulnerabilities in dependencies do not apply to the way this program uses them,
and this repository has no dependencies to make such a statement about yet:

    git ls-files -- Cargo.lock | xargs grep -c '^\[\[package\]\]'
    3

Three packages, all of them this workspace's own crates. Issue #92 is the
nearest owner, and it asks for a bill of materials rather than for a VEX
document, so this property is currently unowned.

### Package (JPRM) / Build package

`dotnet.yml`, job `package`, which calls `build.yml`, job `build`. Builds the
distributable package with the plugin repository manager and uploads it as an
artefact. Required by the target's ruleset.

Verdict: declined, not yet built. The equivalent artefacts here are a binary and
a container, which is issue #100, and no route builds either on a change.

### Package (JPRM) / Generate SBOM

`build.yml`, job `sbom`, reached through the same call. Restores locked,
generates a CycloneDX bill of materials and uploads it. Conditional on the
`attest` input, so on a pull request it reports as skipped. Required by the
target's ruleset, which a skipped check satisfies.

Verdict: declined, not yet built. Issue #92 owes a bill of materials generated
at build time and attached to every release.

### CodeQL

Produced by `github-advanced-security` rather than by a job. It is the code
scanning results check that gates on the findings the analysis uploaded, as
distinct from the analysis jobs themselves. Required by the target's ruleset.

Verdict: adopted. In force here in `.github/workflows/codeql.yml`, and observed
as a check rather than read out of the file:

    gh api repos/iderex/bremsweg/commits/e970ae056de470d391a84fab7e83ffd938c2f230/check-runs?per_page=100 \
      --jq '.check_runs[] | select(.app.slug=="github-advanced-security") | "\(.name)\t\(.conclusion)"'
    CodeQL  success

It is produced here by the same app as on the target and for the same reason: it
is the results check over what the analyses uploaded, not a job. Both analyses
reported nothing:

    gh api "repos/iderex/bremsweg/code-scanning/analyses?ref=refs/pull/114/merge" \
      --jq '.[] | select(.tool.name=="CodeQL") | "\(.category) results=\(.results_count) rules=\(.rules_count)"'
    /language:rust results=0 rules=25
    /language:actions results=0 rules=17

So the triage register #88 asks for is empty rather than maintained, and it is
empty because forty two rules found nothing in a tree that holds no physics. A
first finding is the point at which that sentence has to be replaced by an
acceptance written down, and not before.

### Analyze (csharp)

`codeql.yml`, job `analyze`, matrix entry `csharp` with build mode `autobuild`.
Runs the CodeQL analysis over the compiled application code. Required by the
target's ruleset. The job's name is an expression, so one job in the file
becomes three checks on a commit.

Verdict: adapted. The property is the same and the language is not, so the
counterpart here is `Analyze (rust)` in `.github/workflows/codeql.yml`.

This entry previously said that CodeQL does not support the language this
project is written in. That was wrong, and the correction is what made the
adoption available. It was read rather than assumed:

    gh api repos/iderex/bremsweg/code-scanning/default-setup --jq '{state, languages}'
    {"state":"not-configured","languages":["actions","rust"]}

`not-configured` is the automatic route, which this repository does not use; the
language list is what the analyser reports it covers here either way.

One difference remains and it is not about the language. The target compiles its
application to analyse it, `autobuild`, and the analysis here runs with no build
step. What that costs in coverage has not been measured.

### Analyze (javascript-typescript)

`codeql.yml`, job `analyze`, matrix entry `javascript-typescript` with build
mode `none`. Not required by the target's ruleset.

Verdict: declined, no subject. There is no JavaScript or TypeScript in this
tree:

    git ls-files '*.js' '*.ts' '*.jsx' '*.tsx' | wc -l
    0

The condition that reverses it is such a file appearing, which the plan does not
propose.

### Analyze (actions)

`codeql.yml`, job `analyze`, matrix entry `actions` with build mode `none`.
Analyses the workflow files themselves for the classes CodeQL models. Not
required by the target's ruleset.

Verdict: adopted. In force here in `.github/workflows/codeql.yml`, job name
`Analyze (actions)`, with the same build mode as the target and over this
repository's own workflow files.

The name is a literal here where it is a matrix entry there. That is deliberate
and it is the reason this file had to read the target's check runs rather than
its workflow files: a name generated from a matrix moves when the matrix does,
and a protection rule requiring the old one then matches nothing while going on
looking green.

This overlaps `Audit workflows (zizmor)` and does not replace it. The two model
different classes over the same files, and neither is evidence about the other.

### Enforce greppable invariants

`opengrep.yml`, job `opengrep`. Installs a pinned, checksum-verified Opengrep
binary and runs the rules in `tools/opengrep/rules.yml` with `--error`, so a
repository invariant that no compiler checks becomes something a machine
refuses. Required by the target's ruleset.

Verdict: declined, not yet built. Issue #88 owes the same mechanism over this
project's own invariants, and names four of them: no physical constant defined
outside the constants module, no floating point equality in a test outside the
places that want one, no direct comparison against a table where the fitted
model should be used, and no numeric literal of an atomic mass.

None of those four has a subject in the tree yet, so this half of #88 did not
move with the code scanning half and the issue stays open for it. A rule proved
only against a fixture, with no real subject anywhere, reads afterwards exactly
like a rule that holds.

### Deterministic PR-hygiene checks

`pr-hygiene.yml`, job `hygiene`. Runs a script over the pull request's own
metadata on open, edit, synchronize and reopen. Required by the target's
ruleset.

Verdict: declined, not yet built. Issue #89 owes the deterministic half of it,
and the word deterministic is the whole of the adaptation: a hygiene check that
judges prose is a judgement, and only the part that a machine can decide the
same way twice is being taken.

### prettier

`prettier.yml`, job `prettier`, unnamed so the job id is the check name. Runs a
formatter over the tree through a third party action. Required by the target's
ruleset.

Verdict: declined, not yet built. The property is that formatting is decided by
a tool rather than by a reviewer, and it survives the change of language. The
mechanism does not: this project's toolchain carries its own formatter, so
adopting the action would add a dependency for a job the toolchain already does.
Issue #17 owes the format and lint gates that fail closed.

### Fuzz, per target

`fuzz.yml`, job `fuzz`, name an expression over the matrix. Instruments the
built assembly with SharpFuzz, runs libFuzzer against each named parse surface
for a bounded time, archives crashers and the evolved corpus, and fails the run
if a crasher was found. Runs on a weekly schedule and on manual dispatch, and
never on a pull request, so it is not on the merge gate and is not required by
the ruleset.

Verdict: declined, not yet built. Issue #90 holds whether this is adopted with
named targets or declined with a reason and the condition that reverses it, and
its outcome replaces this line. What can be said today is that nothing here
fuzzes anything, and that the parse surfaces the issue would name, the input
document and the experimental data files, do not exist yet.

### Mutation testing, per scope

`stryker-mutation.yml`, job `mutation`, name an expression over the matrix.
Restores pinned tools, runs Stryker over a scoped set of files and uploads the
report. The step's own name says the score is non-blocking. Weekly schedule and
manual dispatch only, so it is not on the merge gate.

Verdict: declined, not yet built. Issue #91 holds whether it is adopted or
declined for the numeric core. The target's own choice to keep the score
non-blocking is worth carrying into that decision.

### E2E login harness, per provider

`e2e-login.yml`, jobs `select` and `e2e`. Brings up real identity providers in
containers, installs the packaged plugin into a real server, drives a login
through it and then asserts what the plugin wrote to disk. It declares a nightly
schedule, release publication, manual dispatch, and `pull_request` filtered to
`test/e2e/**` and its own workflow file, so it reports on a pull request only
when that pull request touched those paths. Not required by the target's
ruleset, which is consistent: a path-filtered check cannot be required without
blocking every pull request that does not touch those paths.

Verdict: declined, not yet built. The property is that the whole program is
exercised against something real rather than only in units, and this project's
equivalents are the harness for hardware and network bound paths in issue #22,
the reproduce-from-manifest proof in #97, and the validation runs in #98 and
#99. None of them exists.

### Assert manifest-beta lists the newest beta release per generation

`manifest-freshness.yml`, job `check`. Fetches the public plugin manifest the
way a server would and asserts it names the newest beta release for each
generation. Daily schedule and manual dispatch only.

Verdict: declined, no subject. The manifest is a plugin distribution channel a
Jellyfin server reads. This project publishes no such channel and no equivalent
is planned. The condition that reverses it is this project ever publishing an
index that a third party client fetches to discover releases.

### Report any workflow that concluded non-success on the default branch

`publish-failure-alert.yml`, job `sweep`. Sweeps recent runs on the default
branch every thirty minutes and files or updates an issue naming any workflow
that did not succeed, deriving the set rather than naming workflows.

Verdict: declined, not yet built. The property is that a red run on the mainline
is noticed without somebody looking, and it applies here as soon as anything
runs on the mainline. No issue in this milestone owns it, which is a gap this
file records rather than fills.

### Dispatch the daily beta builds

`nightly-betas.yml`, job `dispatch`. Triggers the two beta publishing workflows
once a day.

Verdict: declined, no subject. It is a scheduler for a release cadence this
project does not have. Issue #101 decides the release route and what each
version number promises, and until it does there is nothing here to schedule.

### wiki-lint

`wiki-lint.yml`, job `wiki-lint`, unnamed so the job id is the check name.
Clones the repository wiki and lints it against the code tree, so published
documentation cannot drift from what it documents. Weekly schedule, manual
dispatch, and push to the default branch.

Verdict: declined, no subject as written. This project has no wiki, and its
documentation is `docs/` inside the tree.

The neighbouring property does have a subject here, and it is the one worth
naming: documentation in the tree drifting from the code it describes. Nothing
in this milestone owns it. Issue #94 owns one narrow case of it, that no
document restates the target check list in prose, and this file is the document
that case is about.

### Build package, Upload release assets, Publish plugin manifest

`publish.yml`, jobs `build`, `upload` and `generate`, on a stable tag push and
on manual dispatch. Builds through `build.yml` with attestation enabled, uploads
the artefacts and their checksums to the release, then generates and verifies
the plugin manifest entry.

Verdict: declined, not yet built. The release route is issue #101 and the
release integrity half, meaning the bill of materials, the build provenance and
a coefficient file tied to the data and the commit that produced it, is #92. The
manifest generation part is declined with no subject, for the reason under the
manifest freshness entry.

### Gate on new main commits, Build and publish beta

`publish-beta.yml` and `publish-jf12-beta.yml`, jobs `gate` and `build`, manual
dispatch only. The gate job compares the mainline head against the last beta tag
and skips the build when nothing changed. The build job derives a beta version,
builds, generates a bill of materials, publishes a draft release and then
publishes the manifest.

Verdict: declined, not yet built. Same owner as the entry above, #101 for the
route and #92 for the integrity half. The gate job's property, that a publish
route does not cut a release when nothing changed, is a specific one worth
carrying into #101.

### Build and publish JF12 stable

`publish-jf12-stable.yml`, job `build`, on a tag push matching a stable pattern
and on manual dispatch. Builds against a second set of build metadata for a
second host generation.

Verdict: declined, no subject. The second generation exists because the plugin
supports two incompatible host versions at once. This project has no host, so it
has one release line rather than two. The condition that reverses it is this
project ever supporting two incompatible interfaces simultaneously, which #101
would have to decide.

### Regenerate the manifest

`regenerate-manifest.yml`, job `regenerate`, manual dispatch only. Rebuilds a
distribution manifest from the metadata attached to a named release, and refuses
to repopulate a retired channel without an explicit input saying so.

Verdict: declined, no subject, for the reason under the manifest freshness
entry.

### submit-nuget

Not the target's check. It appears on the target's commits because another
public repository carries the same shas and reports its own run onto them, and
its details point at `Flowfin/jellyfin-plugin-sso`. It is in no workflow file in
the target:

    grep -rn "submit-nuget" . ; echo "exit=$?"
    exit=1

Verdict: declined, no subject. Nothing here to adopt, and the entry exists only
so that a reader comparing this list against a commit's check list does not find
an unexplained name.

## Issues in this milestone with no counterpart on the target

Two, and naming them keeps the parity claim from being read in only one
direction.

Issue #93, no telemetry and a check that refuses an undeclared network call.
The target has no equivalent, because a plugin that authenticates against remote
identity providers makes outbound calls as its whole purpose. The property comes
from this project's own position in `docs/decisions/0012-the-data-protection-position.md`
rather than from the target.

Issue #94, generating this table and keeping it honest as the target moves. It
is about this file, so it has no entry on the target by construction.

## What this file does not establish

The list is derived from the workflow files, the checks two real commits
carried, and the ruleset, all at the commits named at the top. It is not derived
from a run, so a workflow that exists and is broken looks the same here as one
that works.

The comparison against real commits used two pull requests. Both carried the
same eighteen names, which is consistent with the derived list and is not a
proof that no other check exists: a path-filtered workflow reports only on a
pull request that touched those paths, and the E2E entry above is exactly that
case. It was found by reading the file, not by the comparison.

Nothing here has been checked against the target's own documentation of its
gate. Only the tree and the API were read.
