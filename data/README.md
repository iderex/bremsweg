# data

This directory holds data and no code.

Nothing here is tracked except this file. What a clone gets by cloning is this
sentence; what it gets by running

    cargo fetch-compilation

is the experimental stopping compilation for the version `bremsweg-fit` names,
each table with a provenance record beside it. The repository's `.gitignore`
keeps both out of it, because whether the compilation may be redistributed here
is the third entry of issue #1 and is open, and `docs/data-terms.md` reads the
terms as unclear on exactly that.

What a record has to carry is fixed in `docs/decisions/0011` and the gate refuses
a departure from it, including a file here with no record and a record whose hash
is not the hash of the file beside it. So a table that was edited after it was
fetched turns the gate red rather than being fitted against.
