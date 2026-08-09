# Fixtures for the headless and unprivileged run

Three test files that exist only to be judged. One needs a display, one raises
its own privileges, and one needs neither. The workflow beside this directory
compiles each with `rustc --test` and requires the first two to fail in the
environment the ordinary suite runs in, and the third to pass in it.

They are single files rather than crates. They have no dependency and no build
configuration, so a manifest would add a workspace question to answer and
nothing else, and `rustc --test` on one file is the whole of what compiles them.
Nothing in the tree builds, formats or lints them.

The elevation fixture is run twice, and both runs are the proof. In the ordinary
runner environment it has to pass, because a test that failed everywhere would
say nothing about what the restricted environment removed. In the restricted one
it has to fail. The difference between the two runs is the property, and neither
run alone is.

The display fixture names the X server's socket directly rather than reading
`DISPLAY`. A fixture that failed because a variable was unset would be refused
by the environment's own arrangement rather than by the absence of a display,
which is a check confirming what it just did.

The third file is not optional. An environment that refused every test would
refuse the first two as well and would have established nothing about either, so
the neighbour has to pass in exactly the environment that refuses them.
