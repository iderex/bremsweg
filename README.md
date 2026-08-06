# bremsweg

SRIM computes ion ranges, straggling and radiation damage and is the de facto standard for implantation, sputtering, damage research and ion therapy, in particular for dpa. It is closed source, written in Visual Basic 5.0, Windows and IA-32 only, and people drive its GUI from outside under Wine. The paid and free alternatives all rest on Ziegler's tables. The real contribution is not the trajectories, which parallelise embarrassingly well, but an open re-fit of the electronic stopping cross sections against the IAEA databases, because the ZBL parametrisation is a semi-empirical fit nobody can audit.

Planning happens on the issue tracker first. Every decision that shapes
the architecture is written down there with its reasons before the code
that depends on it exists.

See [NOTICE.md](NOTICE.md) for the intended-use notice.
