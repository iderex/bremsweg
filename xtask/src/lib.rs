//! The gate.
//!
//! One command runs every check that can run on this machine, in a declared
//! order, stops at the first failure, and says for every check it did not run
//! why not and what running it would need.
//!
//! The printing is the part that is easy to leave out. A run that covered half
//! the checks and found nothing reads exactly like a run that covered all of
//! them and found nothing, and only the second is worth anything, so a leg that
//! did not run is a row in the report rather than an absence from it.
//!
//! What the legs are is declared in `legs` below and printed by the command.
//! Nothing lists them in a document, because a list in a document drifts
//! against the code that decides it.

use std::ffi::{OsStr, OsString};
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

/// Where a leg runs.
pub enum Run {
    /// Here. `probe` answers whether the tool is present, `command` is the leg
    /// itself, and `needs` says what having the tool would take when the probe
    /// does not answer.
    Here {
        probe: Vec<OsString>,
        command: Vec<OsString>,
        needs: String,
    },
    /// Somewhere other than here. `why` says where it runs instead and `needs`
    /// says what asking for it here would take.
    ///
    /// This is not a skip. A leg declared this way is named in every report, so
    /// the reader of a green run can see which checks that run did not make.
    Elsewhere { why: String, needs: String },
}

/// One check, with its name as the report prints it.
pub struct Leg {
    pub name: String,
    pub run: Run,
}

/// What became of one leg on one run.
pub enum Outcome {
    /// It ran and it passed.
    Ran,
    /// It ran and it failed. Carries what the failure was.
    Failed(String),
    /// Its tool did not answer the probe. Carries what having it would take.
    ToolMissing(String),
    /// It runs somewhere other than here. Carries where, and what asking for it
    /// here would take.
    Elsewhere(String, String),
    /// An earlier leg failed and the run stopped. Carries that reason.
    NotAttempted(String),
}

/// A leg and what became of it.
pub struct Row {
    pub name: String,
    pub outcome: Outcome,
}

/// What one run of the gate did, which is the whole of what it reports.
pub struct Report {
    pub rows: Vec<Row>,
}

impl Report {
    /// The exit status this run deserves.
    ///
    /// Non-zero for a failure and non-zero for a leg whose tool did not answer,
    /// because the promise this command makes is coverage: a caller that cannot
    /// tell an incomplete run from a complete one has been told nothing. A leg
    /// declared as running elsewhere is accounted for rather than missing and
    /// does not move this number.
    ///
    /// It is a returned value rather than a call to `exit` so that a test can
    /// assert it. `main` hands it to `exit` and does nothing else with it.
    pub fn exit_code(&self) -> i32 {
        let complete_and_passing = self.rows.iter().all(|row| {
            matches!(
                row.outcome,
                Outcome::Ran | Outcome::Elsewhere(_, _) | Outcome::NotAttempted(_)
            )
        });
        // A NotAttempted row only exists downstream of a Failed one, so it
        // cannot make a run look complete on its own.
        if complete_and_passing { 0 } else { 1 }
    }

    /// The report, as the command prints it.
    pub fn render(&self) -> String {
        let width = self
            .rows
            .iter()
            .map(|row| row.name.chars().count())
            .max()
            .unwrap_or(0);

        let mut out = String::new();
        for row in &self.rows {
            let padding = " ".repeat(width - row.name.chars().count());
            let said = match &row.outcome {
                Outcome::Ran => "ran, passed".to_string(),
                Outcome::Failed(detail) => format!("ran, FAILED: {detail}"),
                Outcome::ToolMissing(needs) => {
                    format!("did not run: its tool did not answer. Running it needs {needs}")
                }
                Outcome::Elsewhere(why, needs) => {
                    format!("not run here: {why}. Running it here needs {needs}")
                }
                Outcome::NotAttempted(why) => format!("not attempted: {why}"),
            };
            out.push_str(&format!("{}{padding}  {said}\n", row.name));
        }
        out.push_str(&self.summary());
        out
    }

    fn summary(&self) -> String {
        let mut ran = 0;
        let mut failed = 0;
        let mut missing = 0;
        let mut elsewhere = 0;
        let mut not_attempted = 0;
        for row in &self.rows {
            match row.outcome {
                Outcome::Ran => ran += 1,
                Outcome::Failed(_) => failed += 1,
                Outcome::ToolMissing(_) => missing += 1,
                Outcome::Elsewhere(_, _) => elsewhere += 1,
                Outcome::NotAttempted(_) => not_attempted += 1,
            }
        }
        format!(
            "\n{ran} ran, {failed} failed, {missing} could not run here, \
             {elsewhere} run elsewhere, {not_attempted} not attempted.\n"
        )
    }
}

/// Runs the commands a leg is made of.
///
/// A trait rather than a function so that the run above can be exercised
/// against a declaration whose tool is deliberately absent and against one
/// whose leg deliberately fails, neither of which can be arranged by installing
/// or breaking something on the machine running the tests.
pub trait Execute {
    /// Asks whether a tool answers. Shows nothing: a version banner in the
    /// middle of a gate report is noise.
    fn probe(&mut self, command: &[OsString]) -> Result<(), String>;

    /// Runs a leg, showing its output, since the output is what a contributor
    /// fixes the failure from.
    fn leg(&mut self, command: &[OsString]) -> Result<(), String>;
}

/// Runs the legs in order and stops at the first failure.
pub fn run(legs: &[Leg], exec: &mut dyn Execute) -> Report {
    let mut rows = Vec::with_capacity(legs.len());
    let mut stopped_at: Option<String> = None;

    for leg in legs {
        // A leg that runs elsewhere reports the same thing whatever happened
        // here, because nothing here was going to run it. Turning it into "not
        // attempted" after a local failure would say this run changed something
        // about a check it never touched.
        if let (Some(name), Run::Here { .. }) = (&stopped_at, &leg.run) {
            rows.push(Row {
                name: leg.name.clone(),
                outcome: Outcome::NotAttempted(format!("the run stopped when {name} failed")),
            });
            continue;
        }

        let outcome = match &leg.run {
            Run::Elsewhere { why, needs } => Outcome::Elsewhere(why.clone(), needs.clone()),
            Run::Here {
                probe,
                command,
                needs,
            } => {
                if exec.probe(probe).is_err() {
                    Outcome::ToolMissing(needs.clone())
                } else {
                    match exec.leg(command) {
                        Ok(()) => Outcome::Ran,
                        Err(detail) => {
                            stopped_at = Some(leg.name.clone());
                            Outcome::Failed(detail)
                        }
                    }
                }
            }
        };
        rows.push(Row {
            name: leg.name.clone(),
            outcome,
        });
    }

    Report { rows }
}

/// The legs, in the order they run.
///
/// The three that run here come first and cheapest first, so a contributor who
/// forgot to format is told in a second rather than after a test run.
///
/// The legs that run elsewhere are derived from `.github/workflows/` rather than
/// written out, so a workflow added tomorrow appears in this report without
/// anybody remembering to add it here. That is the whole reason the directory is
/// read instead of listed.
pub fn legs(repo_root: &Path, cargo: &OsStr) -> Vec<Leg> {
    let mut legs = vec![
        Leg {
            name: "format".to_string(),
            run: Run::Here {
                probe: argv(cargo, &["fmt", "--version"]),
                command: argv(cargo, &["fmt", "--all", "--check"]),
                needs: "the rustfmt component, which `rust-toolchain.toml` asks for".to_string(),
            },
        },
        Leg {
            name: "lint".to_string(),
            run: Run::Here {
                probe: argv(cargo, &["clippy", "--version"]),
                command: argv(
                    cargo,
                    &[
                        "clippy",
                        "--workspace",
                        "--all-targets",
                        "--",
                        "-D",
                        "warnings",
                    ],
                ),
                needs: "the clippy component, which `rust-toolchain.toml` asks for".to_string(),
            },
        },
        Leg {
            name: "tests".to_string(),
            run: Run::Here {
                probe: argv(cargo, &["--version"]),
                command: argv(cargo, &["test", "--workspace"]),
                needs: "cargo, which the pinned toolchain supplies".to_string(),
            },
        },
        Leg {
            name: "hardware, network and time harness".to_string(),
            run: Run::Elsewhere {
                why: "it is asked for by hand rather than on every change, because it \
                      builds the workspace twice"
                    .to_string(),
                needs: "the run in `crates/needs-hardware-network-or-time/README.md`, \
                        which declares what that run may use"
                    .to_string(),
            },
        },
    ];

    for workflow in workflow_files(repo_root) {
        let shown = workflow
            .strip_prefix(repo_root)
            .unwrap_or(&workflow)
            .display()
            .to_string()
            .replace('\\', "/");
        legs.push(Leg {
            name: workflow_name(&workflow).unwrap_or_else(|| shown.clone()),
            run: Run::Elsewhere {
                why: format!("it runs on the forge, declared in {shown}"),
                needs: format!("the runner image and whatever {shown} installs"),
            },
        });
    }

    legs
}

/// Every workflow file, sorted, so two runs on the same tree print the same
/// report.
fn workflow_files(repo_root: &Path) -> Vec<PathBuf> {
    let directory = repo_root.join(".github").join("workflows");
    let Ok(entries) = std::fs::read_dir(&directory) else {
        return Vec::new();
    };
    let mut files: Vec<PathBuf> = entries
        .filter_map(Result::ok)
        .map(|entry| entry.path())
        .filter(|path| {
            path.extension()
                .is_some_and(|extension| extension == "yml" || extension == "yaml")
        })
        .collect();
    files.sort();
    files
}

/// The name a workflow declares for itself, which is the name a reader sees on a
/// change, or nothing when the file does not declare one.
///
/// A line reader rather than a parser: one key at column zero is what is wanted,
/// and a YAML parser would be the first dependency in this crate.
fn workflow_name(path: &Path) -> Option<String> {
    let text = std::fs::read_to_string(path).ok()?;
    text.lines()
        .find_map(|line| line.strip_prefix("name:"))
        .map(|name| name.trim().trim_matches('"').trim_matches('\'').to_string())
        .filter(|name| !name.is_empty())
}

fn argv(program: &OsStr, rest: &[&str]) -> Vec<OsString> {
    let mut argv = Vec::with_capacity(rest.len() + 1);
    argv.push(program.to_os_string());
    argv.extend(rest.iter().map(OsString::from));
    argv
}

/// Runs the legs as real processes, from the repository root, so a leg's meaning
/// does not depend on the directory the command was started in.
pub struct Processes {
    pub repo_root: PathBuf,
}

impl Processes {
    fn spawn(&self, command: &[OsString], show: bool) -> Result<(), String> {
        let (program, arguments) = command
            .split_first()
            .ok_or_else(|| "a leg with no command".to_string())?;
        let mut process = Command::new(program);
        process.args(arguments).current_dir(&self.repo_root);
        if !show {
            process.stdout(Stdio::null()).stderr(Stdio::null());
        }
        match process.status() {
            Err(error) => Err(format!("{} did not start: {error}", shown(command))),
            Ok(status) if status.success() => Ok(()),
            Ok(status) => Err(format!(
                "{} exited with {}",
                shown(command),
                status
                    .code()
                    .map_or("no code".to_string(), |c| c.to_string())
            )),
        }
    }
}

impl Execute for Processes {
    fn probe(&mut self, command: &[OsString]) -> Result<(), String> {
        self.spawn(command, false)
    }

    fn leg(&mut self, command: &[OsString]) -> Result<(), String> {
        self.spawn(command, true)
    }
}

/// A command as the report shows it. Lossy, and only ever for reading.
fn shown(command: &[OsString]) -> String {
    command
        .iter()
        .map(|part| part.to_string_lossy().into_owned())
        .collect::<Vec<_>>()
        .join(" ")
}

#[cfg(test)]
mod tests {
    use super::{Execute, Leg, Outcome, Report, Run, run};
    use std::ffi::OsString;

    /// Answers every probe and passes every leg, except that a command whose
    /// text contains `fails` fails, and a probe whose text contains
    /// `tool_absent` does not answer.
    ///
    /// Named for the case rather than for its shape: the two fields are the two
    /// things that can go wrong on a real machine, arranged deliberately.
    struct AnswersEverythingExcept {
        fails: String,
        tool_absent: String,
        legs_run: Vec<String>,
    }

    impl AnswersEverythingExcept {
        fn nothing_wrong() -> Self {
            Self {
                fails: "\u{0}nothing".to_string(),
                tool_absent: "\u{0}nothing".to_string(),
                legs_run: Vec::new(),
            }
        }

        fn where_the_command_containing(text: &str) -> Self {
            Self {
                fails: text.to_string(),
                ..Self::nothing_wrong()
            }
        }

        fn where_the_probe_containing(text: &str) -> Self {
            Self {
                tool_absent: text.to_string(),
                ..Self::nothing_wrong()
            }
        }
    }

    fn as_text(command: &[OsString]) -> String {
        super::shown(command)
    }

    impl Execute for AnswersEverythingExcept {
        fn probe(&mut self, command: &[OsString]) -> Result<(), String> {
            if as_text(command).contains(&self.tool_absent) {
                return Err("no such tool".to_string());
            }
            Ok(())
        }

        fn leg(&mut self, command: &[OsString]) -> Result<(), String> {
            let text = as_text(command);
            self.legs_run.push(text.clone());
            if text.contains(&self.fails) {
                return Err("exited with 1".to_string());
            }
            Ok(())
        }
    }

    /// Two legs that run here and one that runs elsewhere, which is the shape
    /// the real declaration has.
    fn a_declaration() -> Vec<Leg> {
        vec![
            Leg {
                name: "first".to_string(),
                run: Run::Here {
                    probe: vec![OsString::from("probe-first")],
                    command: vec![OsString::from("run-first")],
                    needs: "the first tool".to_string(),
                },
            },
            Leg {
                name: "second".to_string(),
                run: Run::Here {
                    probe: vec![OsString::from("probe-second")],
                    command: vec![OsString::from("run-second")],
                    needs: "the second tool".to_string(),
                },
            },
            Leg {
                name: "on the forge".to_string(),
                run: Run::Elsewhere {
                    why: "it runs on the forge".to_string(),
                    needs: "a runner".to_string(),
                },
            },
        ]
    }

    fn report_with(exec: &mut AnswersEverythingExcept) -> Report {
        run(&a_declaration(), exec)
    }

    #[test]
    fn every_leg_is_named_whether_it_ran_or_not() {
        let report = report_with(&mut AnswersEverythingExcept::nothing_wrong());
        let printed = report.render();

        for name in ["first", "second", "on the forge"] {
            assert!(printed.contains(name), "{name} is missing from:\n{printed}");
        }
        assert_eq!(report.exit_code(), 0, "nothing went wrong:\n{printed}");
    }

    #[test]
    fn a_leg_that_ran_elsewhere_is_reported_with_what_asking_here_would_need() {
        let printed = report_with(&mut AnswersEverythingExcept::nothing_wrong()).render();

        assert!(
            printed.contains("not run here: it runs on the forge. Running it here needs a runner"),
            "the forge leg said nothing about itself:\n{printed}"
        );
    }

    #[test]
    fn a_leg_whose_tool_is_absent_is_reported_rather_than_skipped() {
        let report = report_with(&mut AnswersEverythingExcept::where_the_probe_containing(
            "probe-second",
        ));
        let printed = report.render();

        assert!(
            printed.contains("second") && printed.contains("did not run"),
            "the absent tool was skipped silently:\n{printed}"
        );
        assert!(
            printed.contains("the second tool"),
            "the report did not say what running it would need:\n{printed}"
        );
        assert_ne!(
            report.exit_code(),
            0,
            "an incomplete run reported itself as a complete one:\n{printed}"
        );
    }

    #[test]
    fn every_leg_that_fails_gives_the_run_a_non_zero_status() {
        // Derived from the declaration rather than written out, so a leg added
        // to it is covered without anybody adding a case here.
        for leg in a_declaration() {
            let Run::Here { command, .. } = &leg.run else {
                continue;
            };
            let text = super::shown(command);
            let report = report_with(&mut AnswersEverythingExcept::where_the_command_containing(
                &text,
            ));
            let printed = report.render();

            assert_ne!(
                report.exit_code(),
                0,
                "{} failed and the run passed:\n{printed}",
                leg.name
            );
            assert!(
                printed.contains("FAILED"),
                "{} failed and the report did not say so:\n{printed}",
                leg.name
            );
        }
    }

    #[test]
    fn the_run_stops_at_the_first_failure_and_says_the_rest_was_not_attempted() {
        let mut exec = AnswersEverythingExcept::where_the_command_containing("run-first");
        let report = run(&a_declaration(), &mut exec);
        let printed = report.render();

        assert_eq!(
            exec.legs_run,
            vec!["run-first".to_string()],
            "the run continued past a failure:\n{printed}"
        );
        assert!(
            printed.contains("not attempted: the run stopped when first failed"),
            "the legs after the failure were not accounted for:\n{printed}"
        );
        assert!(
            printed.contains("not run here: it runs on the forge"),
            "a failure here changed what the report says about a check it never \
             touched:\n{printed}"
        );
    }

    #[test]
    fn an_ordinary_failure_is_told_apart_from_an_absent_tool() {
        let failed = report_with(&mut AnswersEverythingExcept::where_the_command_containing(
            "run-first",
        ));
        let absent = report_with(&mut AnswersEverythingExcept::where_the_probe_containing(
            "probe-first",
        ));

        assert!(matches!(failed.rows[0].outcome, Outcome::Failed(_)));
        assert!(matches!(absent.rows[0].outcome, Outcome::ToolMissing(_)));
    }
}
