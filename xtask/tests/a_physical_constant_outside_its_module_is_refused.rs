//! The physical constants check, proved on a fixture that must be refused and a
//! neighbouring one that must pass.
//!
//! The two fixtures live in `refused/` as tracked files rather than as literals
//! here, because what is under test is text a person would write and a reader
//! of this test should be able to see it as a source file. Neither is a member
//! of any target, so nothing compiles them.
//!
//! The cases after the pair are the ones that decide whether the check is
//! usable. A tolerance, a version number in a message, an integer count and a
//! hash constant are all things this tree is full of, and a check that refused
//! any of them would be turned off within a week.

use std::path::{Path, PathBuf};
use xtask::physical_constants::{
    Declared, MODULE, Refusal, declared, float_literals, how_the_module_declares_them, refusals,
    second_copies, significant_digits,
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

/// The constants this repository declares, read from the module the rule names
/// rather than written out here, so this test cannot disagree with it.
fn the_real_constants() -> Vec<Declared> {
    let path = repo_root().join(MODULE);
    let text = std::fs::read_to_string(&path)
        .unwrap_or_else(|error| panic!("could not read {}: {error}", path.display()));
    let found = declared(&text);
    assert!(
        !found.is_empty(),
        "{MODULE} declares nothing, so every case below would pass while proving nothing"
    );
    found
}

#[test]
fn a_constant_written_out_by_hand_is_refused() {
    let refused = second_copies(
        &fixture("a_physical_constant_written_out.rs"),
        &the_real_constants(),
        "a_fixture.rs",
    );

    assert_eq!(
        refused.len(),
        1,
        "the fixture holds one copied constant beside one number that is not one, \
         and the check found {refused:?}"
    );
    assert!(
        matches!(&refused[0], Refusal::SecondCopy { name, .. } if name.contains("BOHR")),
        "the refusal did not name the constant that was copied: {refused:?}"
    );
}

#[test]
fn the_same_expression_through_the_module_passes() {
    let refused = second_copies(
        &fixture("a_physical_constant_reached_through_the_module.rs"),
        &the_real_constants(),
        "a_fixture.rs",
    );

    assert!(
        refused.is_empty(),
        "the neighbour that reaches the constant through the module was refused too, \
         so the check above refuses an expression rather than a copy: {refused:?}"
    );
}

#[test]
fn a_constant_written_in_another_unit_is_still_a_copy() {
    // The Bohr radius in nanometre rather than in angstrom. The digits are the
    // ones the module holds and only the magnitude moved, which is the copy a
    // digit by digit comparison against the module's literal would miss.
    let refused = second_copies(
        "pub const NEARLY: f64 = 0.052_917_721;\n",
        &the_real_constants(),
        "a_fixture.rs",
    );

    assert_eq!(
        refused.len(),
        1,
        "the same constant in another unit passed: {refused:?}"
    );
}

#[test]
fn a_number_that_is_not_a_constant_is_left_alone() {
    // Four numbers this tree already has or will have: a tolerance, a
    // measurement, a count and a version. None is a physical constant and each
    // carries enough digits to be looked at.
    let refused = second_copies(
        "let tolerance = 2.5e-3;\nlet measured = 1234.5;\nlet histories = 100_000;\n\
         let rust_version = 1.97;\n",
        &the_real_constants(),
        "a_fixture.rs",
    );

    assert!(
        refused.is_empty(),
        "an ordinary number was read as a physical constant: {refused:?}"
    );
}

#[test]
fn a_constant_quoted_in_a_comment_or_a_message_is_not_a_copy() {
    // The module's own doc comments quote its values, and a failure message
    // that prints one is describing the module rather than redefining it.
    let refused = second_copies(
        "// the Bohr radius is 0.529_177_210_544 angstrom\n\
         let message = \"expected 0.529_177_210_544\";\n",
        &the_real_constants(),
        "a_fixture.rs",
    );

    assert!(
        refused.is_empty(),
        "a value written in prose was read as a definition: {refused:?}"
    );
}

#[test]
fn a_hash_constant_is_not_a_physical_one() {
    // `sha256.rs` and `crc32.rs` are nothing but long constants. They are
    // integers, and a check that reached them would be unusable here.
    let refused = second_copies(
        "const ROUND: [u32; 2] = [0x428a_2f98, 0x7137_4491];\nconst SEED: u32 = 3_988_292_384;\n",
        &the_real_constants(),
        "a_fixture.rs",
    );

    assert!(
        refused.is_empty(),
        "an integer constant was read as a physical one: {refused:?}"
    );
}

#[test]
fn a_value_written_to_three_digits_is_below_what_can_be_policed() {
    // Stated rather than hidden. Three digits of the Bohr radius are the digits
    // of ordinary numbers, and a check that refused them would refuse a
    // tolerance of 0.529 as well. This is the hole, and it is why the module
    // side of the check refuses a constant that short.
    let refused = second_copies("let coarse = 0.529;\n", &the_real_constants(), "a.rs");

    assert!(
        refused.is_empty(),
        "a three digit value was policed, which the check does not claim to do: {refused:?}"
    );
}

#[test]
fn a_constant_with_no_citation_is_refused() {
    let module = "/// The Bohr radius, in angstrom.\n\
                  pub const BOHR_RADIUS_ANGSTROM: f64 = 0.529_177_210_544;\n";

    let refused = how_the_module_declares_them(&declared(module), module);

    let fields: Vec<&String> = refused
        .iter()
        .filter_map(|it| match it {
            Refusal::NoCitation { field, .. } => Some(field),
            _ => None,
        })
        .collect();
    assert_eq!(
        fields.len(),
        2,
        "a constant with neither a source nor a revision was accepted: {refused:?}"
    );
}

#[test]
fn a_constant_with_its_citation_is_not() {
    let module = "/// The Bohr radius, in angstrom.\n\
                  ///\n\
                  /// Source: CODATA recommended values, Bohr radius\n\
                  /// Revision: CODATA 2022\n\
                  pub const BOHR_RADIUS_ANGSTROM: f64 = 0.529_177_210_544;\n";

    let refused = how_the_module_declares_them(&declared(module), module);

    assert!(
        refused.is_empty(),
        "a cited constant was refused, so the check above refuses a declaration rather \
         than a missing citation: {refused:?}"
    );
}

#[test]
fn a_constant_too_short_to_police_is_refused() {
    // The other direction the module side fails closed in. A constant written
    // to three digits could be copied anywhere and no reading of a literal
    // would find it, so it is refused where it is declared instead.
    let module = "/// A constant nobody could police.\n\
                  ///\n\
                  /// Source: somewhere\n\
                  /// Revision: 2026\n\
                  pub const ROUGH: f64 = 0.53;\n";

    let refused = how_the_module_declares_them(&declared(module), module);

    assert!(
        refused
            .iter()
            .any(|it| matches!(it, Refusal::TooFewDigits { .. })),
        "a constant too short to be found elsewhere was admitted: {refused:?}"
    );
}

#[test]
fn a_tree_with_no_module_is_refused_rather_than_passed() {
    // The failure that would be invisible: the module is renamed or removed,
    // nothing is declared, every literal in the tree matches nothing, and the
    // leg goes green having checked nothing at all.
    let root = Path::new(env!("CARGO_TARGET_TMPDIR")).join("a_tree_with_no_constants_module");
    std::fs::create_dir_all(&root)
        .unwrap_or_else(|error| panic!("could not make {}: {error}", root.display()));

    let refused = refusals(&root);

    assert!(
        matches!(refused.as_slice(), [Refusal::NoModule { .. }]),
        "a tree with no constants module passed the leg: {refused:?}"
    );
}

#[test]
fn the_digits_of_a_literal_drop_everything_that_is_not_one() {
    assert_eq!(significant_digits("0.529_177_210_544"), "529177210544");
    assert_eq!(significant_digits("6.022_140_76e23"), "602214076");
    assert_eq!(significant_digits("14.399_645f64"), "14399645");
}

#[test]
fn a_literal_is_read_where_a_range_and_a_method_call_are_not() {
    let found = float_literals("let a = 1.5;\nlet b = 1..4;\nlet c = 2.max(3);\nlet d = 4.0e-3;\n");

    let lines: Vec<usize> = found.iter().map(|(line, _)| *line).collect();
    assert_eq!(
        lines,
        vec![1, 4],
        "a range or a method call was read as a floating point literal: {found:?}"
    );
}

#[test]
fn this_tree_holds_none() {
    let refused = refusals(&repo_root());

    assert!(
        refused.is_empty(),
        "the tree writes a physical constant outside the module that holds them:\n{}",
        refused
            .iter()
            .map(ToString::to_string)
            .collect::<Vec<_>>()
            .join("\n")
    );
}
