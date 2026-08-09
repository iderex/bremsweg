# Fixtures for the lock file check

Three crates that exist only to be judged. `honoured` names a path dependency
its lock file holds, `drifted` names the same dependency and its lock file does
not, and `dependency` is that dependency. The two differ by one entry in one
file, which is the near miss the check has to tell apart.

They are outside the workspace. Each manifest carries an empty `[workspace]`
table so it is its own root, and the root manifest lists its members explicitly
rather than by a pattern, so nothing here is built, formatted or linted with the
tree.

The dependency is a path rather than a registry crate on purpose. A registry
dependency would make the fixture's verdict depend on an index fetch, and a
fixture that needs the network to say what it says is one that goes red for a
reason it is not about.
