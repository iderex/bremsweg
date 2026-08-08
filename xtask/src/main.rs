//! The command.
//!
//! `cargo gate` and nothing else. What it does is in `lib.rs`; what is here is
//! the argument handling, the repository root, and handing the report's verdict
//! to the process exit status.

use std::path::{Path, PathBuf};

fn main() {
    // A typo is refused rather than ignored. A command that accepts an argument
    // it does not understand and runs the whole gate anyway teaches a
    // contributor that the argument worked.
    let unexpected: Vec<String> = std::env::args().skip(1).collect();
    if !unexpected.is_empty() {
        eprintln!(
            "gate takes no arguments, and was given: {}",
            unexpected.join(" ")
        );
        std::process::exit(2);
    }

    let repo_root = repo_root();
    let cargo = std::env::var_os("CARGO").unwrap_or_else(|| "cargo".into());

    let legs = xtask::legs(&repo_root, &cargo);
    let mut processes = xtask::Processes {
        repo_root: repo_root.clone(),
    };
    let report = xtask::run(&legs, &mut processes);

    print!("{}", report.render());
    std::process::exit(report.exit_code());
}

/// The repository root, from cargo rather than from the working directory, which
/// the command may assume nothing about.
fn repo_root() -> PathBuf {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    manifest_dir
        .parent()
        .unwrap_or_else(|| panic!("no directory above {}", manifest_dir.display()))
        .to_path_buf()
}
