//! Every cargo alias in this tree resolves from the committed lock file.
//!
//! The failure this refuses is quiet in both directions. An alias is itself a
//! cargo invocation, so `cargo gate` resolves the workspace and rewrites
//! `Cargo.lock` before the first leg runs. Without `--locked` on the alias, a
//! manifest naming a dependency the committed lock file does not carry is
//! resolved by the gate itself: the gate then passes, because every leg after
//! that point sees a lock file agreeing with the manifests, and the rewritten
//! lock file sits in the contributor's working tree waiting to be committed
//! alongside a change nobody reviewed it as part of.
//!
//! The flag cannot be on the leg commands for the same reason. By the time a
//! leg runs, the resolution has already happened and a `--locked` there can
//! never fire.
//!
//! So the property is one flag in one file, and one flag in one file is exactly
//! what gets dropped by somebody editing the line for another reason. This is
//! what refuses that.

use std::path::{Path, PathBuf};

/// The repository root, from cargo rather than from the working directory.
fn repo_root() -> PathBuf {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    manifest_dir
        .parent()
        .unwrap_or_else(|| panic!("no directory above {}", manifest_dir.display()))
        .to_path_buf()
}

/// The cargo subcommands that resolve the workspace, which are the ones an
/// alias has to be locked for. An alias for a subcommand outside this list
/// touches no lock file and is not this test's subject.
const RESOLVES: [&str; 7] = ["build", "check", "clippy", "test", "run", "bench", "doc"];

/// The aliases declared in a cargo configuration, as name and command.
///
/// A line reader rather than a parser, for the reason `workflow_name` in the
/// gate gives: one table of `name = "string"` lines is what is wanted, and a
/// TOML parser would be the first dependency in this crate.
fn aliases(config: &str) -> Vec<(String, String)> {
    let mut found = Vec::new();
    let mut inside = false;
    for line in config.lines() {
        let line = line.trim();
        if line.starts_with('[') {
            inside = line == "[alias]";
            continue;
        }
        if !inside || line.starts_with('#') {
            continue;
        }
        let Some((name, value)) = line.split_once('=') else {
            continue;
        };
        found.push((
            name.trim().to_string(),
            value.trim().trim_matches('"').to_string(),
        ));
    }
    found
}

/// The aliases that resolve the workspace and do not say `--locked`.
fn unlocked(config: &str) -> Vec<String> {
    aliases(config)
        .into_iter()
        .filter(|(_, command)| {
            command
                .split_whitespace()
                .any(|word| RESOLVES.contains(&word))
        })
        .filter(|(_, command)| !command.split_whitespace().any(|word| word == "--locked"))
        .map(|(name, _)| name)
        .collect()
}

#[test]
fn every_alias_in_this_tree_that_resolves_says_locked() {
    let path = repo_root().join(".cargo").join("config.toml");
    let config = std::fs::read_to_string(&path)
        .unwrap_or_else(|e| panic!("could not read {}: {e}", path.display()));

    let resolving: Vec<String> = aliases(&config)
        .into_iter()
        .filter(|(_, command)| {
            command
                .split_whitespace()
                .any(|word| RESOLVES.contains(&word))
        })
        .map(|(name, _)| name)
        .collect();
    assert!(
        !resolving.is_empty(),
        "no alias in {} invokes a subcommand that resolves the workspace, so this test \
         would pass while proving nothing",
        path.display()
    );

    let missing = unlocked(&config);
    assert!(
        missing.is_empty(),
        "these aliases resolve the workspace and do not pass --locked, so running one \
         rewrites the committed lock file rather than refusing: {missing:?}"
    );
}

#[test]
fn an_alias_without_the_flag_is_named() {
    // The near miss this is built around: somebody edits the alias line to add
    // a package or a flag and the `--locked` goes with the edit. The reader has
    // to name that alias rather than pass because the other one is fine.
    let config = "\
[alias]
gate = \"run --quiet --package xtask --\"
fetch-compilation = \"run --quiet --locked --package bremsweg-fit --bin fetch-compilation --\"
";
    assert_eq!(unlocked(config), vec!["gate".to_string()]);
}

#[test]
fn an_alias_that_resolves_nothing_is_not_asked_for_the_flag() {
    // The neighbour. A reader that demanded the flag of every alias would be
    // refusing something that has no lock file to be wrong about, and its
    // verdict on the tree would then mean less than it looks.
    let config = "\
[alias]
tidy = \"fmt --all\"
";
    assert!(unlocked(config).is_empty());
}

#[test]
fn a_table_that_is_not_the_alias_table_is_not_read() {
    // The `[target.…]` table below carries a key whose value names a
    // subcommand. A reader that ignored the table headings would report it as
    // an unlocked alias, and the failure would be a name nobody can find in the
    // alias table.
    let config = "\
[alias]
gate = \"run --quiet --locked --package xtask --\"

[target.x86_64-pc-windows-msvc]
rustflags = [\"-Clink-arg=/Brepro run build\"]
";
    assert!(unlocked(config).is_empty());
}
