# Fixtures for the advisory scan

Two lock files, each naming one crate from the registry, and nothing else. One
names a version an advisory has been filed against and one names a version that
has none, so the scan is shown refusing the first and passing the second rather
than only being shown to run.

They are lock files rather than crates because the scanner reads a lock file and
needs no manifest, no source and no resolution to say what it says. Nothing here
is built.

What these fixtures depend on that the tree does not control: the advisory
database is somebody else's and it moves. The version below with an advisory
against it can have that advisory withdrawn, and the version below without one
can acquire one, and either would turn this check red for a reason it is not
about. That is the cost of proving the scan bites against the real database
instead of a stub, and it is paid deliberately: a stub would prove the fixture
parses and nothing about whether the scan finds anything. When it happens, the
failure names the fixture and the advisory, which is enough to tell it apart
from a finding against this repository's own lock file.
