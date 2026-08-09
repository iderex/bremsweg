//! The format and lint legs fail closed, proved one refusal at a time.
//!
//! The failure this refuses is the ordinary one: a formatting check that only
//! reformats, a linter whose findings are printed rather than counted, and a
//! suppression that turns one of them off with nothing beside it saying why.
//! Each of the three is arranged here as a file that must be refused, next to a
//! file saying the same thing that must pass, because a check that refuses
//! everything and a check that refuses the right thing look identical from one
//! side.
//!
//! The severities are not written here. They are read out of `Cargo.toml`,
//! which is where the tree declares them, so removing a lint from that table
//! turns its fixture red rather than quietly retiring it. The manifest is read
//! as text rather than parsed, for the reason `crates/core/tests/dependencies.rs`
//! already gives: a parser would be the first dependency in this crate.

use std::path::{Path, PathBuf};
use std::process::Command;

/// The repository root, from cargo rather than from the working directory.
fn repo_root() -> PathBuf {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    manifest_dir
        .parent()
        .unwrap_or_else(|| panic!("no directory above {}", manifest_dir.display()))
        .to_path_buf()
}

fn workspace_manifest() -> String {
    let path = repo_root().join("Cargo.toml");
    std::fs::read_to_string(&path)
        .unwrap_or_else(|error| panic!("could not read {}: {error}", path.display()))
}

/// One of the files in `refused/`, none of which is a member of any target.
fn fixture(name: &str) -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("refused")
        .join(name)
}

/// Every lint the named table sets, as the flag that asks a compiler for the
/// same severity.
///
/// `[workspace.lints.rust] warnings = "deny"` becomes `-Dwarnings`, and
/// `[workspace.lints.clippy] cast_sign_loss = "deny"` becomes
/// `-Dclippy::cast_sign_loss`.
fn severities(manifest: &str, table: &str, prefix: &str) -> Vec<String> {
    let mut flags = Vec::new();
    let mut inside = false;
    for line in manifest.lines() {
        let line = line.trim();
        if line.starts_with('[') {
            inside = line == format!("[{table}]");
            continue;
        }
        if !inside || line.is_empty() || line.starts_with('#') {
            continue;
        }
        let Some((name, level)) = line.split_once('=') else {
            continue;
        };
        let level = level.trim().trim_matches('"');
        let flag = match level {
            "forbid" => "-F",
            "deny" => "-D",
            "warn" => "-W",
            "allow" => "-A",
            _ => continue,
        };
        flags.push(format!("{flag}{prefix}{}", name.trim()));
    }
    flags
}

/// The edition the workspace sets, so a fixture is read the way the tree is.
fn edition(manifest: &str) -> String {
    manifest
        .lines()
        .find_map(|line| line.trim().strip_prefix("edition"))
        .and_then(|rest| rest.split_once('='))
        .map(|(_, value)| value.trim().trim_matches('"').to_string())
        .unwrap_or_else(|| panic!("the workspace manifest sets no edition"))
}

/// What a tool said about one file, and whether it refused it.
struct Verdict {
    refused: bool,
    said: String,
}

fn verdict_of(mut command: Command, tool: &str) -> Verdict {
    let output = command
        .current_dir(repo_root())
        .output()
        .unwrap_or_else(|error| {
            panic!(
                "{tool} did not run: {error}. It is a component `rust-toolchain.toml` asks \
                 for, and this test cannot be skipped without saying nothing"
            )
        });
    let mut said = String::from_utf8_lossy(&output.stdout).into_owned();
    said.push_str(&String::from_utf8_lossy(&output.stderr));
    Verdict {
        refused: !output.status.success(),
        said,
    }
}

fn ask_the_formatter(file: &Path) -> Verdict {
    let mut command = Command::new("rustfmt");
    command
        .arg("--edition")
        .arg(edition(&workspace_manifest()))
        .arg("--check")
        .arg(file);
    verdict_of(command, "rustfmt")
}

/// The linter, asked with the severities the workspace manifest declares and no
/// others.
fn ask_the_linter(file: &Path) -> Verdict {
    let manifest = workspace_manifest();
    let stem = file
        .file_stem()
        .unwrap_or_else(|| panic!("{} has no file name", file.display()));

    let mut command = Command::new("clippy-driver");
    command
        .arg("--edition")
        .arg(edition(&manifest))
        .arg("--crate-type")
        .arg("lib")
        .arg("--emit=metadata")
        .arg("-o")
        .arg(Path::new(env!("CARGO_TARGET_TMPDIR")).join(stem))
        .arg(file)
        .args(severities(&manifest, "workspace.lints.rust", ""))
        .args(severities(&manifest, "workspace.lints.clippy", "clippy::"))
        // The tree's own configuration, so this asks the question the gate
        // asks rather than a neighbouring one.
        .env("CLIPPY_CONF_DIR", repo_root());
    verdict_of(command, "clippy-driver")
}

/// Whether a refusal names the lint it is for. A fixture refused for some other
/// reason is a fixture that proves nothing, and the two are indistinguishable
/// from the exit status alone.
fn names_the_lint(said: &str, lint: &str) -> bool {
    said.contains(lint) || said.contains(&lint.replace('_', "-"))
}

#[test]
fn the_manifest_declares_something_for_these_fixtures_to_prove() {
    let manifest = workspace_manifest();
    let rust = severities(&manifest, "workspace.lints.rust", "");
    let clippy = severities(&manifest, "workspace.lints.clippy", "clippy::");

    assert!(
        rust.contains(&"-Dwarnings".to_string()),
        "the manifest does not make a warning a failure, so the lint leg prints \
         rather than refuses: {rust:?}"
    );
    assert!(
        !clippy.is_empty(),
        "no lint is set in the manifest, so the fixtures below would pass against \
         an empty deny list and prove nothing"
    );
}

#[test]
fn the_formatter_refuses_an_unformatted_file() {
    let verdict = ask_the_formatter(&fixture("unformatted.rs"));

    assert!(
        verdict.refused,
        "an unformatted file was accepted:\n{}",
        verdict.said
    );
    assert!(
        verdict.said.contains("unformatted.rs"),
        "the refusal did not name the file it was about:\n{}",
        verdict.said
    );
}

#[test]
fn the_formatter_accepts_the_same_thing_formatted() {
    let verdict = ask_the_formatter(&fixture("formatted.rs"));

    assert!(
        !verdict.refused,
        "a formatted file was refused, so the check above refuses everything:\n{}",
        verdict.said
    );
}

#[test]
fn the_linter_refuses_a_cast_that_can_lose_the_value() {
    let verdict = ask_the_linter(&fixture("a_cast_that_can_truncate.rs"));

    assert!(
        verdict.refused,
        "a truncating cast was accepted:\n{}",
        verdict.said
    );
    assert!(
        names_the_lint(&verdict.said, "cast_possible_truncation"),
        "the refusal was for something other than the cast:\n{}",
        verdict.said
    );
}

#[test]
fn the_linter_accepts_the_same_conversion_decided() {
    let verdict = ask_the_linter(&fixture("a_cast_that_cannot.rs"));

    assert!(
        !verdict.refused,
        "the conversion that cannot lose the value was refused too, so the check \
         above refuses everything:\n{}",
        verdict.said
    );
}

#[test]
fn the_linter_refuses_a_suppression_carrying_no_reason() {
    let verdict = ask_the_linter(&fixture("a_suppression_with_no_reason.rs"));

    assert!(
        verdict.refused,
        "a lint was silenced with nothing saying why, and it was accepted:\n{}",
        verdict.said
    );
    assert!(
        names_the_lint(&verdict.said, "allow_attributes_without_reason"),
        "the refusal was for something other than the missing reason:\n{}",
        verdict.said
    );
}

#[test]
fn the_linter_accepts_the_same_suppression_with_one() {
    let verdict = ask_the_linter(&fixture("a_suppression_with_a_reason.rs"));

    assert!(
        !verdict.refused,
        "a suppression carrying its reason was refused, so the check above is about \
         suppressions rather than about reasons:\n{}",
        verdict.said
    );
}
