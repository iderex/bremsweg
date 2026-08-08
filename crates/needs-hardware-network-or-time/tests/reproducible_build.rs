//! Two release builds of this commit produce the same bytes.
//!
//! The property is the one issue #24 argues for: a binary somebody was given
//! can be checked against the source they read. It was established by hand once
//! and nothing has held it since, so a change that reintroduces a timestamp,
//! an absolute path or an embedded environment into the artefact would pass
//! every route in this tree.
//!
//! It is in this harness rather than in the ordinary suite for one reason a
//! reader can check against the clock: it builds the workspace twice from an
//! empty target directory. That took 31.26 s here against an ordinary suite
//! that finishes in hundredths, and the number is the whole of the tree at this
//! commit, so it is a floor rather than an estimate.
//!
//! What it does not cover is stated where the requirement is declared. The
//! setting that makes the property hold is in `.cargo/config.toml` and belongs
//! to one target, so this test claims nothing about any other and refuses to
//! run on one.

use bremsweg_needs_hardware_network_or_time::{Requirement, require};
use std::path::{Path, PathBuf};
use std::process::Command;

/// The workspace root, from cargo rather than from the working directory,
/// which a test may assume nothing about.
fn workspace_root() -> PathBuf {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    manifest_dir
        .ancestors()
        .nth(2)
        .unwrap_or_else(|| {
            panic!(
                "no workspace root two levels above {}",
                manifest_dir.display()
            )
        })
        .to_path_buf()
}

/// The cargo that is running this, so the build under test uses the pinned
/// toolchain rather than whichever one the path happens to offer.
fn the_running_cargo() -> PathBuf {
    std::env::var_os("CARGO").map_or_else(|| PathBuf::from("cargo"), PathBuf::from)
}

/// Builds the workspace into `target_dir`, from empty, and returns the binary.
fn build_from_scratch_into(target_dir: &Path) -> PathBuf {
    if target_dir.exists() {
        std::fs::remove_dir_all(target_dir)
            .unwrap_or_else(|e| panic!("could not empty {}: {e}", target_dir.display()));
    }

    let root = workspace_root();
    let status = Command::new(the_running_cargo())
        // The directory decides which `.cargo/config.toml` cargo reads, and the
        // setting this test exists to check is in the one at the root.
        .current_dir(&root)
        .env("CARGO_TARGET_DIR", target_dir)
        .args(["build", "--release", "--locked"])
        .status()
        .unwrap_or_else(|e| panic!("could not start cargo in {}: {e}", root.display()));

    assert!(status.success(), "the build failed: {status}");

    let binary = target_dir
        .join("release")
        .join(format!("bremsweg{}", std::env::consts::EXE_SUFFIX));
    assert!(binary.is_file(), "no binary at {}", binary.display());
    binary
}

/// The offsets at which two byte strings differ, and how many there are.
///
/// Where the bytes differ is worth more than how many: nineteen and twenty
/// differing bytes were both measured for the same two causes, because a byte
/// of a timestamp can happen to match, and the positions said which cause it
/// was when the count did not.
fn differing_offsets(left: &[u8], right: &[u8]) -> Vec<usize> {
    left.iter()
        .zip(right.iter())
        .enumerate()
        .filter_map(|(at, (l, r))| (l != r).then_some(at))
        .collect()
}

#[test]
#[ignore = "builds the workspace twice from an empty target directory"]
fn two_release_builds_of_this_commit_produce_the_same_bytes() {
    require(&[
        // `/Brepro` in `.cargo/config.toml` is an MSVC linker option and is set
        // for this target alone. What the same question needs on Linux and on
        // macOS has not been measured, so this test refuses those machines
        // rather than reporting a result it has no grounds for.
        Requirement::platform("x86_64", "windows", "msvc"),
        // One rather than four. It is asked for as a whole minute because the
        // measured 31.26 s is the cost of building this tree, and this tree is
        // a placeholder function, a manifest reader and a binary printing one
        // line. The number a reader should not trust is the one that stays
        // still while the tree grows.
        Requirement::Minutes(1),
    ]);

    let scratch = Path::new(env!("CARGO_TARGET_TMPDIR"));
    let first = std::fs::read(build_from_scratch_into(
        &scratch.join("reproducible-build-first"),
    ))
    .expect("could not read the first binary");
    let second = std::fs::read(build_from_scratch_into(
        &scratch.join("reproducible-build-second"),
    ))
    .expect("could not read the second binary");

    assert_eq!(
        first.len(),
        second.len(),
        "the two builds differ in length: {} and {} bytes",
        first.len(),
        second.len()
    );

    let differing = differing_offsets(&first, &second);
    assert!(
        differing.is_empty(),
        "two release builds of this commit differ in {} of {} bytes, at offsets \
         {:?}. Something the source does not decide is reaching the artefact: a \
         timestamp, an absolute path or an embedded environment. Issue #24 is \
         where the property is argued.",
        differing.len(),
        first.len(),
        &differing[..differing.len().min(24)],
    );
}

// The comparison itself is arithmetic and needs none of what the test above
// needs, so it is checked here where it costs nothing rather than only inside a
// run somebody has to ask for.

#[test]
fn identical_bytes_differ_nowhere() {
    assert!(differing_offsets(b"abc", b"abc").is_empty());
}

#[test]
fn a_single_changed_byte_is_reported_at_its_offset() {
    assert_eq!(differing_offsets(b"abc", b"abd"), vec![2]);
}

#[test]
fn every_differing_offset_is_reported_rather_than_the_first() {
    assert_eq!(differing_offsets(b"abcd", b"AbcD"), vec![0, 3]);
}
