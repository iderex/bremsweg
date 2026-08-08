//! The harness for tests that cannot run on every change.
//!
//! Three kinds of test cannot live in the ordinary suite. One needs a
//! particular machine, because a scaling measurement made on an unknown core
//! count measures nothing. One needs the network and a remote service this
//! project does not run. One needs more minutes than anybody will spend per
//! change. Putting any of them in the ordinary suite makes it slow, machine
//! dependent or dependent on somebody else's uptime, and leaving them out of
//! the tree means the claims they would check have no evidence behind them.
//!
//! This crate is where they go, and its name is what they need rather than when
//! they run. `crates/needs-hardware-network-or-time/README.md` says what runs
//! them and on what.
//!
//! # What this module is for
//!
//! A test here declares what it needs before it does anything, and an unmet
//! requirement makes it **fail**. That direction is the whole point. The
//! ordinary shape, returning early when the machine is not right, turns a test
//! that never ran into a green one, and a suite of those reports success for a
//! machine on which nothing was checked.
//!
//! Two of the requirements can be read off the machine. Two cannot, and neither
//! is guessed at:
//!
//! - Whether the network is reachable cannot be established without using it,
//!   and reaching one host says nothing about the service a test actually
//!   wants. So the operator declares it.
//! - How long a run may take is not a fact about the machine at all. It is a
//!   statement about what the person starting it is willing to wait for. So the
//!   operator declares that too.
//!
//! Both declarations go in one environment variable, named by
//! [`GRANT_VARIABLE`], and a test whose grant is absent says which word is
//! missing and what the whole command would be.

use std::fmt;

/// The environment variable in which the operator declares what this run may
/// use. Comma separated; the words are [`Grants::NETWORK`] and
/// [`Grants::TIME`].
pub const GRANT_VARIABLE: &str = "BREMSWEG_HARNESS";

/// What a test in this harness needs before it may run.
///
/// Every variant is something a test cannot supply for itself. A requirement
/// that the test could arrange, such as a directory to write in, is not one of
/// these and does not belong here.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Requirement {
    /// A machine of this architecture, operating system and toolchain
    /// environment. Used where the result is only claimed for one target, which
    /// is the ordinary case for anything measuring the bytes a linker produced.
    Platform {
        /// As `std::env::consts::ARCH` spells it, such as `x86_64`.
        arch: &'static str,
        /// As `std::env::consts::OS` spells it, such as `windows`.
        os: &'static str,
        /// As the target triple spells it, such as `msvc`, `gnu` or `musl`.
        env: &'static str,
    },
    /// At least this many cores the process may actually use. A scaling
    /// measurement on fewer is a measurement of something else.
    Cores(usize),
    /// The network, and a service this project does not run.
    Network,
    /// Roughly this many minutes, which is more than a per change run allows.
    Minutes(u32),
}

impl Requirement {
    /// A [`Requirement::Platform`], written so a test reads as a sentence.
    #[must_use]
    pub const fn platform(arch: &'static str, os: &'static str, env: &'static str) -> Self {
        Self::Platform { arch, os, env }
    }
}

impl fmt::Display for Requirement {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Platform { arch, os, env } => write!(f, "the {arch}-{os}-{env} target"),
            Self::Cores(n) => write!(f, "at least {n} usable cores"),
            Self::Network => write!(f, "the network"),
            Self::Minutes(n) => write!(f, "roughly {n} minutes"),
        }
    }
}

/// What the machine running this actually is.
///
/// A value rather than a set of calls, so that the decision in [`unmet`] is a
/// pure function of what it was told and can be tested against machines this
/// one is not.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Machine {
    /// As `std::env::consts::ARCH` spells it.
    pub arch: &'static str,
    /// As `std::env::consts::OS` spells it.
    pub os: &'static str,
    /// As the target triple spells it. Empty where the target has no
    /// environment component and nothing here pretends otherwise.
    pub env: &'static str,
    /// Cores this process may use, which is not always what the machine has.
    pub cores: usize,
}

impl Machine {
    /// The machine this was compiled for and is running on.
    ///
    /// The architecture and the operating system come from the standard
    /// library. The environment component of the triple has no entry there, so
    /// it is read from the configuration the compiler was given, and a target
    /// whose environment is none of the three named is reported as empty rather
    /// than as a guess.
    #[must_use]
    pub fn observed() -> Self {
        const ENV: &str = if cfg!(target_env = "msvc") {
            "msvc"
        } else if cfg!(target_env = "gnu") {
            "gnu"
        } else if cfg!(target_env = "musl") {
            "musl"
        } else {
            ""
        };

        Self {
            arch: std::env::consts::ARCH,
            os: std::env::consts::OS,
            env: ENV,
            // A machine that cannot report its parallelism is treated as one
            // core, so a test asking for more is refused rather than run on an
            // unknown number.
            cores: std::thread::available_parallelism().map_or(1, std::num::NonZero::get),
        }
    }
}

/// What the operator declared this run may use.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Grants {
    network: bool,
    time: bool,
}

impl Grants {
    /// The word that grants [`Requirement::Network`].
    pub const NETWORK: &'static str = "network";
    /// The word that grants [`Requirement::Minutes`].
    pub const TIME: &'static str = "time";

    /// Reads a comma separated declaration.
    ///
    /// A word that is neither of the two is ignored rather than refused,
    /// because the refusal a reader needs is the one naming the requirement the
    /// test could not meet, and an unknown word produces exactly that refusal
    /// one line later without a second message competing with it.
    #[must_use]
    pub fn declared(value: &str) -> Self {
        let mut grants = Self::default();
        for word in value.split(',').map(str::trim) {
            match word {
                Self::NETWORK => grants.network = true,
                Self::TIME => grants.time = true,
                _ => {}
            }
        }
        grants
    }

    /// Reads [`GRANT_VARIABLE`]. An absent variable grants nothing.
    #[must_use]
    pub fn from_environment() -> Self {
        std::env::var(GRANT_VARIABLE).map_or_else(|_| Self::default(), |v| Self::declared(&v))
    }

    /// Whether the network was declared.
    #[must_use]
    pub const fn network(&self) -> bool {
        self.network
    }

    /// Whether the time was declared.
    #[must_use]
    pub const fn time(&self) -> bool {
        self.time
    }
}

/// Every requirement in `wanted` that `machine` and `grants` do not meet, each
/// with what would meet it.
///
/// The whole list rather than the first, so that a test refused on a machine
/// that is wrong in two ways says both instead of being run twice to find out.
#[must_use]
pub fn unmet(wanted: &[Requirement], machine: &Machine, grants: &Grants) -> Vec<String> {
    let mut refusals = Vec::new();

    for requirement in wanted {
        match *requirement {
            Requirement::Platform { arch, os, env } => {
                if machine.arch != arch || machine.os != os || machine.env != env {
                    refusals.push(format!(
                        "needs {requirement}, and this is {}-{}-{}",
                        machine.arch, machine.os, machine.env
                    ));
                }
            }
            Requirement::Cores(n) => {
                if machine.cores < n {
                    refusals.push(format!(
                        "needs {requirement}, and this process may use {}",
                        machine.cores
                    ));
                }
            }
            Requirement::Network => {
                if !grants.network() {
                    refusals.push(format!(
                        "needs {requirement}, which this run did not declare. \
                         Add `{}` to {GRANT_VARIABLE}",
                        Grants::NETWORK
                    ));
                }
            }
            Requirement::Minutes(_) => {
                if !grants.time() {
                    refusals.push(format!(
                        "needs {requirement}, which this run did not declare. \
                         Add `{}` to {GRANT_VARIABLE}",
                        Grants::TIME
                    ));
                }
            }
        }
    }

    refusals
}

/// Refuses the calling test unless the machine and the run meet `wanted`.
///
/// Panics with every unmet requirement and the command that would meet it.
/// Panicking is the point: a test whose requirement is absent has to be red,
/// because the alternative is a green run in which nothing was checked.
///
/// # Panics
///
/// When any requirement in `wanted` is unmet.
pub fn require(wanted: &[Requirement]) {
    let refusals = unmet(wanted, &Machine::observed(), &Grants::from_environment());
    assert!(
        refusals.is_empty(),
        "refused rather than skipped, so that a run of this harness that \
         checked nothing is never green.\n{}\n\nThe whole harness, with \
         everything declared:\n    {GRANT_VARIABLE}={},{} cargo test -p \
         bremsweg-needs-hardware-network-or-time -- --ignored",
        refusals.join("\n"),
        Grants::NETWORK,
        Grants::TIME,
    );
}

#[cfg(test)]
mod tests {
    use super::{Grants, Machine, Requirement, unmet};

    /// A machine every test below starts from and changes one thing about, so
    /// that what a case is about is the line that differs.
    const A_SIXTEEN_CORE_WINDOWS_MACHINE: Machine = Machine {
        arch: "x86_64",
        os: "windows",
        env: "msvc",
        cores: 16,
    };

    const THE_PINNED_WINDOWS_TARGET: Requirement =
        Requirement::platform("x86_64", "windows", "msvc");

    fn nothing_declared() -> Grants {
        Grants::declared("")
    }

    #[test]
    fn a_matching_platform_is_met() {
        assert!(
            unmet(
                &[THE_PINNED_WINDOWS_TARGET],
                &A_SIXTEEN_CORE_WINDOWS_MACHINE,
                &nothing_declared(),
            )
            .is_empty()
        );
    }

    #[test]
    fn a_platform_differing_only_in_the_environment_is_refused() {
        let a_windows_machine_built_with_the_gnu_toolchain = Machine {
            env: "gnu",
            ..A_SIXTEEN_CORE_WINDOWS_MACHINE
        };
        let refusals = unmet(
            &[THE_PINNED_WINDOWS_TARGET],
            &a_windows_machine_built_with_the_gnu_toolchain,
            &nothing_declared(),
        );
        assert_eq!(refusals.len(), 1, "{refusals:?}");
        assert!(refusals[0].contains("x86_64-windows-gnu"), "{refusals:?}");
    }

    #[test]
    fn a_platform_differing_only_in_the_operating_system_is_refused() {
        let a_linux_machine = Machine {
            os: "linux",
            env: "gnu",
            ..A_SIXTEEN_CORE_WINDOWS_MACHINE
        };
        assert_eq!(
            unmet(
                &[THE_PINNED_WINDOWS_TARGET],
                &a_linux_machine,
                &nothing_declared(),
            )
            .len(),
            1
        );
    }

    #[test]
    fn exactly_the_asked_for_core_count_is_met() {
        assert!(
            unmet(
                &[Requirement::Cores(16)],
                &A_SIXTEEN_CORE_WINDOWS_MACHINE,
                &nothing_declared(),
            )
            .is_empty()
        );
    }

    #[test]
    fn one_core_short_is_refused() {
        let a_machine_one_core_short = Machine {
            cores: 15,
            ..A_SIXTEEN_CORE_WINDOWS_MACHINE
        };
        let refusals = unmet(
            &[Requirement::Cores(16)],
            &a_machine_one_core_short,
            &nothing_declared(),
        );
        assert_eq!(refusals.len(), 1, "{refusals:?}");
        assert!(refusals[0].contains("may use 15"), "{refusals:?}");
    }

    #[test]
    fn the_network_is_refused_when_it_was_not_declared() {
        let refusals = unmet(
            &[Requirement::Network],
            &A_SIXTEEN_CORE_WINDOWS_MACHINE,
            &nothing_declared(),
        );
        assert_eq!(refusals.len(), 1, "{refusals:?}");
        assert!(refusals[0].contains("BREMSWEG_HARNESS"), "{refusals:?}");
    }

    #[test]
    fn the_network_is_met_when_it_was_declared() {
        assert!(
            unmet(
                &[Requirement::Network],
                &A_SIXTEEN_CORE_WINDOWS_MACHINE,
                &Grants::declared("network"),
            )
            .is_empty()
        );
    }

    #[test]
    fn declaring_the_time_does_not_declare_the_network() {
        assert_eq!(
            unmet(
                &[Requirement::Network],
                &A_SIXTEEN_CORE_WINDOWS_MACHINE,
                &Grants::declared("time"),
            )
            .len(),
            1
        );
    }

    #[test]
    fn minutes_are_refused_when_the_time_was_not_declared() {
        assert_eq!(
            unmet(
                &[Requirement::Minutes(4)],
                &A_SIXTEEN_CORE_WINDOWS_MACHINE,
                &nothing_declared(),
            )
            .len(),
            1
        );
    }

    #[test]
    fn both_words_can_be_declared_at_once() {
        let grants = Grants::declared("network,time");
        assert!(grants.network() && grants.time());
        assert!(
            unmet(
                &[Requirement::Network, Requirement::Minutes(4)],
                &A_SIXTEEN_CORE_WINDOWS_MACHINE,
                &grants,
            )
            .is_empty()
        );
    }

    #[test]
    fn a_declaration_with_spaces_around_the_words_is_read() {
        let grants = Grants::declared(" network , time ");
        assert!(grants.network() && grants.time());
    }

    #[test]
    fn a_word_that_is_neither_grants_nothing() {
        let grants = Grants::declared("networking");
        assert!(!grants.network() && !grants.time());
    }

    #[test]
    fn every_unmet_requirement_is_reported_rather_than_the_first() {
        let a_two_core_linux_machine = Machine {
            os: "linux",
            env: "gnu",
            cores: 2,
            ..A_SIXTEEN_CORE_WINDOWS_MACHINE
        };
        let refusals = unmet(
            &[
                THE_PINNED_WINDOWS_TARGET,
                Requirement::Cores(16),
                Requirement::Network,
                Requirement::Minutes(4),
            ],
            &a_two_core_linux_machine,
            &nothing_declared(),
        );
        assert_eq!(refusals.len(), 4, "{refusals:?}");
    }

    #[test]
    fn a_machine_reports_its_own_platform_as_met() {
        // The one case that reads the machine this is running on rather than a
        // fixture, so that `observed` is exercised rather than only described.
        let here = Machine::observed();
        assert!(
            unmet(
                &[Requirement::platform(here.arch, here.os, here.env)],
                &here,
                &nothing_declared(),
            )
            .is_empty()
        );
        assert!(here.cores >= 1);
    }
}
