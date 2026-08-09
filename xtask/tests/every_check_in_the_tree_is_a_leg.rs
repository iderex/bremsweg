//! Every workflow in the tree appears in the gate's report.
//!
//! The failure this refuses is the one a hand written list produces: a workflow
//! is added, the gate says nothing about it, and a reader of a green run
//! believes the run covered a check that nobody ran here and nobody was told
//! about. The declaration derives these legs from the directory for that reason,
//! and this test is what holds the derivation to the directory.

use std::ffi::OsString;
use std::path::{Path, PathBuf};

/// The repository root, from cargo rather than from the working directory.
fn repo_root() -> PathBuf {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    manifest_dir
        .parent()
        .unwrap_or_else(|| panic!("no directory above {}", manifest_dir.display()))
        .to_path_buf()
}

/// Every workflow file in the tree, read here rather than taken from the gate,
/// so the two answers are independent.
#[expect(
    clippy::expect_used,
    reason = "a helper of a test, where a panic is how an unreadable tree is reported; \
              the lint is for the long run, and this never runs in one"
)]
fn workflow_files(repo_root: &Path) -> Vec<String> {
    let directory = repo_root.join(".github").join("workflows");
    let entries = std::fs::read_dir(&directory)
        .unwrap_or_else(|e| panic!("could not read {}: {e}", directory.display()));
    let mut names: Vec<String> = entries
        .map(|entry| entry.expect("a readable directory entry").path())
        .filter(|path| {
            path.extension()
                .is_some_and(|extension| extension == "yml" || extension == "yaml")
        })
        .map(|path| {
            path.file_name()
                .expect("a workflow file has a name")
                .to_string_lossy()
                .into_owned()
        })
        .collect();
    names.sort();
    names
}

/// The report of a run in which nothing is executed, which is all this test
/// needs: the legs that run elsewhere are printed from the declaration and
/// nothing about them depends on running anything.
struct RefusesToRunAnything;

impl xtask::Execute for RefusesToRunAnything {
    fn probe(&mut self, _command: &[OsString]) -> Result<(), String> {
        Err("this test runs nothing".to_string())
    }

    fn leg(&mut self, _command: &[OsString]) -> Result<(), String> {
        Err("this test runs nothing".to_string())
    }
}

#[test]
fn every_workflow_file_is_named_in_the_report() {
    let root = repo_root();
    let files = workflow_files(&root);
    assert!(
        !files.is_empty(),
        "no workflow files found under {}, so this test would pass while proving nothing",
        root.display()
    );

    let legs = xtask::legs(&root, &OsString::from("cargo"));
    let printed = xtask::run(&legs, &mut RefusesToRunAnything).render();

    for file in files {
        assert!(
            printed.contains(&file),
            "{file} is a check in this tree and the gate report does not name it:\n{printed}"
        );
    }
}

#[test]
fn a_workflow_this_tree_does_not_hold_is_not_named() {
    let root = repo_root();
    let legs = xtask::legs(&root, &OsString::from("cargo"));
    let printed = xtask::run(&legs, &mut RefusesToRunAnything).render();

    // The neighbour to the test above. If the legs were a written list rather
    // than a reading of the directory, a name could survive the file being
    // removed, and the first test could not tell the difference.
    assert!(
        !printed.contains("a-workflow-nobody-wrote.yml"),
        "the report names a workflow that is not in the tree:\n{printed}"
    );
}
