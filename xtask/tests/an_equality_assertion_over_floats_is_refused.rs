//! The float equality check, proved on a fixture that must be refused and a
//! neighbouring one that must pass.
//!
//! The two fixtures live in `refused/` as tracked files rather than as literals
//! here, because what is under test is text a person would write and a reader
//! of this test should be able to see it as a source file rather than as an
//! escaped string. Neither is a member of any target, so nothing compiles them.
//!
//! The cases after the pair are the ones that decide whether the check is
//! usable: a version number in a message, a commented out assertion, an integer
//! comparison and a lifetime are all things this tree is full of, and a check
//! that refused any of them would be turned off within a week.

use std::path::{Path, PathBuf};
use xtask::float_equality::{
    Deliberate, Refusal, equality_assertions_over_floats, refusals, refusals_against,
};

fn repo_root() -> PathBuf {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    manifest_dir
        .parent()
        .unwrap_or_else(|| panic!("no directory above {}", manifest_dir.display()))
        .to_path_buf()
}

fn fixture(name: &str) -> String {
    let path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("refused")
        .join(name);
    std::fs::read_to_string(&path)
        .unwrap_or_else(|error| panic!("could not read {}: {error}", path.display()))
}

#[test]
fn an_equality_assertion_over_a_float_is_refused() {
    let found = equality_assertions_over_floats(&fixture("an_equality_assertion_over_floats.rs"));

    assert_eq!(
        found.len(),
        1,
        "the fixture holds one equality assertion over a float and the check found \
         {found:?}"
    );
    let (_, wrote) = &found[0];
    assert!(
        wrote.contains("2.35"),
        "the refusal did not name what was compared: {wrote}"
    );
}

#[test]
fn the_same_comparison_through_the_helper_passes() {
    let found = equality_assertions_over_floats(&fixture("a_comparison_through_the_helper.rs"));

    assert!(
        found.is_empty(),
        "the neighbour that uses the shared comparison was refused too, so the check \
         above refuses a number rather than a comparison: {found:?}"
    );
}

#[test]
fn an_integer_comparison_is_left_alone() {
    // The commonest assertion in this tree. A check that took this would be
    // switched off before it caught anything.
    let found = equality_assertions_over_floats(
        "assert_eq!(saturating_double(21), 42);\nassert_eq!(count, 1_000);\n",
    );

    assert!(
        found.is_empty(),
        "an integer comparison was refused: {found:?}"
    );
}

#[test]
fn a_number_inside_a_message_is_not_a_comparison() {
    // The false positive that would hurt most: a version string reads exactly
    // like a float and appears in the manifest tests already.
    let found = equality_assertions_over_floats(
        "assert_eq!(version(), \"0.0.0\", \"the manifest says 1.97.0\");\n",
    );

    assert!(
        found.is_empty(),
        "a version number in a string was read as a float: {found:?}"
    );
}

#[test]
fn a_predicate_about_two_floats_is_not_a_comparison_of_them() {
    // `assert_eq!(relatively_close(1.0, 2.0), false)` compares two booleans.
    // The floats in it are what the predicate was asked about, and refusing
    // this would make the shared comparison unusable in the tests that prove
    // it works.
    let found = equality_assertions_over_floats(
        "assert_eq!(relatively_close(1.0, 2.0, coarse()), false);\n",
    );

    assert!(
        found.is_empty(),
        "an assertion about a predicate was read as a comparison of floats: {found:?}"
    );
}

#[test]
fn a_number_in_the_failure_message_is_not_a_comparison() {
    // Only the pair being compared is read. The arguments after it are the
    // message and what it formats.
    let found = equality_assertions_over_floats("assert_eq!(counted, 3, \"{}\", 2.5);\n");

    assert!(
        found.is_empty(),
        "a number formatted into a failure message was read as a comparison: {found:?}"
    );
}

#[test]
fn a_commented_out_assertion_is_not_an_assertion() {
    let found = equality_assertions_over_floats("// assert_eq!(stopping, 2.35);\n");

    assert!(found.is_empty(), "a comment was read as code: {found:?}");
}

#[test]
fn a_lifetime_is_not_a_character_literal() {
    // A quote that opens a lifetime and is never closed. Reading it as a
    // literal would swallow every assertion after it in the file, and the check
    // would go quiet rather than red.
    let found = equality_assertions_over_floats(
        "struct Tolerance { because: &'static str }\nassert_eq!(stopping, 2.35);\n",
    );

    assert_eq!(
        found.len(),
        1,
        "an assertion after a lifetime was not seen, so the check reads a lifetime as \
         the start of a literal: {found:?}"
    );
    assert_eq!(found[0].0, 2, "the refusal named the wrong line: {found:?}");
}

#[test]
fn this_tree_holds_none() {
    let refused = refusals(&repo_root());

    assert!(
        refused.is_empty(),
        "the tree compares floating point numbers for equality:\n{}",
        refused
            .iter()
            .map(ToString::to_string)
            .collect::<Vec<_>>()
            .join("\n")
    );
}

/// A tree of one file holding one exact comparison, which is what the register
/// is for. The real register is empty, so both of its directions are exercised
/// against a tree arranged here rather than against this repository.
fn a_tree_with_one_exact_comparison(named: &str) -> PathBuf {
    let root = Path::new(env!("CARGO_TARGET_TMPDIR")).join(named);
    let inside = root.join("crates").join("probe").join("src");
    std::fs::create_dir_all(&inside)
        .unwrap_or_else(|error| panic!("could not make {}: {error}", inside.display()));
    std::fs::write(
        inside.join("lib.rs"),
        "#[test]\nfn the_same_seed_gives_the_same_number() {\n    \
         assert_eq!(seeded(7), 0.4386);\n}\n",
    )
    .unwrap_or_else(|error| panic!("could not write into {}: {error}", inside.display()));
    root
}

const SITE: &str = "crates/probe/src/lib.rs";

#[test]
fn a_site_not_in_the_register_is_refused() {
    let root = a_tree_with_one_exact_comparison("not_in_the_register");

    let refused = refusals_against(&root, &[]);

    assert!(
        matches!(refused.as_slice(), [Refusal::FloatEquality { file, .. }] if file == SITE),
        "the comparison was not refused: {refused:?}"
    );
}

#[test]
fn a_site_in_the_register_is_not() {
    let root = a_tree_with_one_exact_comparison("in_the_register");

    let refused = refusals_against(
        &root,
        &[Deliberate {
            file: SITE,
            line_contains: "assert_eq!(seeded(7)",
            because: "the same seed producing a different number is the failure this \
                      test exists for, so anything short of exactness passes it",
        }],
    );

    assert!(
        refused.is_empty(),
        "a comparison the register covers was refused anyway: {refused:?}"
    );
}

#[test]
fn a_register_entry_naming_no_line_is_refused() {
    let root = a_tree_with_one_exact_comparison("a_stale_entry");

    let refused = refusals_against(
        &root,
        &[Deliberate {
            file: SITE,
            line_contains: "assert_eq!(a_line_nobody_wrote",
            because: "an entry that has outlived the site it was written for",
        }],
    );

    assert!(
        refused
            .iter()
            .any(|it| matches!(it, Refusal::StaleEntry { file, .. } if file == SITE)),
        "an entry naming no line survived: {refused:?}"
    );
    assert!(
        refused
            .iter()
            .any(|it| matches!(it, Refusal::FloatEquality { .. })),
        "the site the stale entry failed to cover was excused by it anyway: {refused:?}"
    );
}
