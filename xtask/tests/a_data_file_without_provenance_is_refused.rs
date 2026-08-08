//! Each refusal of the provenance check, proved by its own fixture, and a
//! neighbouring fixture that differs by one change and passes.
//!
//! The fixtures are built here from literals rather than stored as tracked
//! files. The bytes are the point: a hash fixture that went through this
//! repository's own text handling could be normalised on the way in, and the
//! record would then disagree with the file for a reason that has nothing to do
//! with what the check is about.
//!
//! Each fixture is a directory standing in for a repository root, with its own
//! `data/` inside it, so the check under test is the same function the gate runs
//! and not a variant of it.

use std::path::{Path, PathBuf};
use xtask::provenance::{Refusal, refusals};

/// The element data every fixture below is filed around, and its digest.
///
/// The digest is written out rather than computed here. Computing it with the
/// same function the check uses would make the passing fixture agree with itself
/// whatever that function did; this value was produced independently, by
/// `python -c "import hashlib; ..."`, so a defect in the hash breaks the
/// fixture that should pass.
const ELEMENTS: &[u8] = b"# element data\nH 1 1.008\nHe 2 4.0026\n";
const ELEMENTS_DIGEST: &str =
    "sha256:01be955fd0e12bf5503dee7070fab16b02ddebeaa1deb81ac05141812bded9d3";

/// The same file after one number gained a digit, which is what a regeneration
/// looks like.
const REGENERATED: &[u8] = b"# element data\nH 1 1.0080\nHe 2 4.0026\n";

const COEFFICIENTS: &[u8] = b"# fitted coefficients\na0 1.5\na1 -0.25\n";
const COEFFICIENTS_DIGEST: &str =
    "sha256:51a20fd76f80881c251e5687376bb7f43b658fb9ed52ad7e34af0bf5fd3edde8";

/// A complete record of a fetched file, which every fixture below spoils in
/// exactly one way.
fn a_fetched_record(file: &str, digest: &str) -> String {
    format!(
        "Kind: fetched\n\
         File: {file}\n\
         Hash: {digest}\n\
         Source: a public compilation, by the identifier a reader looks it up with\n\
         Source-Version: 2026.1\n\
         Obtained: 2026-08-08\n\
         Request: the documented query for that identifier and version\n\
         \n\
         Energies converted from keV to eV and the columns named. Nothing dropped.\n"
    )
}

/// Writes a fixture and returns the root the check is pointed at. Named for the
/// case, since the name is what a failure shows.
fn fixture(case: &str, files: &[(&str, &[u8])]) -> PathBuf {
    let root = Path::new(env!("CARGO_TARGET_TMPDIR")).join(case);
    if root.exists() {
        std::fs::remove_dir_all(&root)
            .unwrap_or_else(|e| panic!("could not empty {}: {e}", root.display()));
    }
    for (path, bytes) in files {
        let full = root.join(path);
        let directory = full
            .parent()
            .unwrap_or_else(|| panic!("{path} has no directory above it"));
        std::fs::create_dir_all(directory)
            .unwrap_or_else(|e| panic!("could not make {}: {e}", directory.display()));
        std::fs::write(&full, bytes)
            .unwrap_or_else(|e| panic!("could not write {}: {e}", full.display()));
    }
    root
}

#[test]
fn a_data_file_with_no_record_is_refused() {
    let root = fixture("no_record", &[("data/elements.tsv", ELEMENTS)]);

    assert_eq!(
        refusals(&root),
        vec![Refusal::NoRecord {
            file: "data/elements.tsv".to_string()
        }]
    );
}

#[test]
fn a_record_describing_a_file_that_is_not_there_is_refused() {
    let record = a_fetched_record("data/elements.tsv", ELEMENTS_DIGEST);
    let root = fixture(
        "record_names_no_file",
        &[("data/elements.tsv.provenance", record.as_bytes())],
    );

    assert_eq!(
        refusals(&root),
        vec![Refusal::NamesNoFile {
            record: "data/elements.tsv.provenance".to_string(),
            named: "data/elements.tsv".to_string(),
        }]
    );
}

#[test]
fn a_record_copied_onto_a_second_file_is_refused() {
    // Both files are present and both records resolve, so nothing about this
    // fixture is missing. The second record describes the first file's numbers,
    // which is the failure the File field exists for.
    let record = a_fetched_record("data/elements.tsv", ELEMENTS_DIGEST);
    let root = fixture(
        "record_names_another_file",
        &[
            ("data/elements.tsv", ELEMENTS),
            ("data/elements.tsv.provenance", record.as_bytes()),
            ("data/coefficients.tsv", COEFFICIENTS),
            ("data/coefficients.tsv.provenance", record.as_bytes()),
        ],
    );

    assert_eq!(
        refusals(&root),
        vec![
            Refusal::NamesAnotherFile {
                record: "data/coefficients.tsv.provenance".to_string(),
                named: "data/elements.tsv".to_string(),
                beside: "data/coefficients.tsv".to_string(),
            },
            Refusal::HashDoesNotMatch {
                record: "data/coefficients.tsv.provenance".to_string(),
                recorded: ELEMENTS_DIGEST.to_string(),
                found: COEFFICIENTS_DIGEST.to_string(),
            },
        ]
    );
}

#[test]
fn a_record_that_survived_the_file_being_regenerated_is_refused() {
    // The near miss this check exists for. The record is complete, every field
    // says something, the file it names is the file it sits beside, and the
    // numbers are not the numbers it was written about.
    let record = a_fetched_record("data/elements.tsv", ELEMENTS_DIGEST);
    let root = fixture(
        "hash_does_not_match",
        &[
            ("data/elements.tsv", REGENERATED),
            ("data/elements.tsv.provenance", record.as_bytes()),
        ],
    );

    let found = match refusals(&root).as_slice() {
        [
            Refusal::HashDoesNotMatch {
                recorded, found, ..
            },
        ] => {
            assert_eq!(recorded, ELEMENTS_DIGEST);
            found.clone()
        }
        other => panic!("expected one hash refusal, got {other:?}"),
    };
    assert_ne!(
        found, ELEMENTS_DIGEST,
        "the regenerated file hashed to the recorded digest"
    );
}

#[test]
fn a_record_missing_a_field_its_kind_requires_is_refused() {
    let without_a_version = a_fetched_record("data/elements.tsv", ELEMENTS_DIGEST)
        .lines()
        .filter(|line| !line.starts_with("Source-Version:"))
        .collect::<Vec<_>>()
        .join("\n");
    let root = fixture(
        "missing_field",
        &[
            ("data/elements.tsv", ELEMENTS),
            ("data/elements.tsv.provenance", without_a_version.as_bytes()),
        ],
    );

    assert_eq!(
        refusals(&root),
        vec![Refusal::MissingField {
            record: "data/elements.tsv.provenance".to_string(),
            field: "Source-Version:".to_string(),
        }]
    );
}

#[test]
fn a_derived_record_missing_the_command_that_produced_it_is_refused() {
    // The other half of distinguishing the two kinds. The record above is refused
    // for a field only a fetched record owes; this one is refused for a field only
    // a derived record owes, and the passing fixture below carries a fetched
    // record with none of the derived fields. Without all three, the check could
    // be requiring the union of both sets and every test would still pass.
    let without_a_command = format!(
        "Kind: derived\n\
         File: data/coefficients.tsv\n\
         Hash: {COEFFICIENTS_DIGEST}\n\
         Inputs: data/elements.tsv\n\
         Commit: 0000000000000000000000000000000000000000\n\
         \n\
         Fitted from the file named above and nothing else.\n"
    );
    let root = fixture(
        "derived_missing_command",
        &[
            ("data/coefficients.tsv", COEFFICIENTS),
            (
                "data/coefficients.tsv.provenance",
                without_a_command.as_bytes(),
            ),
        ],
    );

    assert_eq!(
        refusals(&root),
        vec![Refusal::MissingField {
            record: "data/coefficients.tsv.provenance".to_string(),
            field: "Command:".to_string(),
        }]
    );
}

#[test]
fn a_kind_that_is_neither_of_the_two_is_refused() {
    let neither = a_fetched_record("data/elements.tsv", ELEMENTS_DIGEST)
        .replace("Kind: fetched", "Kind: from the literature");
    let root = fixture(
        "unknown_kind",
        &[
            ("data/elements.tsv", ELEMENTS),
            ("data/elements.tsv.provenance", neither.as_bytes()),
        ],
    );

    assert_eq!(
        refusals(&root),
        vec![Refusal::MissingField {
            record: "data/elements.tsv.provenance".to_string(),
            field: "a Kind: of either fetched or derived".to_string(),
        }]
    );
}

#[test]
fn a_record_written_with_no_value_after_a_field_is_refused() {
    // A field present and empty is absence with a colon in front of it, and it is
    // the shape somebody produces by writing the template and not filling it in.
    let empty_source = a_fetched_record("data/elements.tsv", ELEMENTS_DIGEST)
        .lines()
        .map(|line| {
            if line.starts_with("Source:") {
                "Source:"
            } else {
                line
            }
        })
        .collect::<Vec<_>>()
        .join("\n");
    let root = fixture(
        "empty_field",
        &[
            ("data/elements.tsv", ELEMENTS),
            ("data/elements.tsv.provenance", empty_source.as_bytes()),
        ],
    );

    assert_eq!(
        refusals(&root),
        vec![Refusal::MissingField {
            record: "data/elements.tsv.provenance".to_string(),
            field: "Source:".to_string(),
        }]
    );
}

#[test]
fn a_record_with_no_body_is_refused() {
    let header_only = a_fetched_record("data/elements.tsv", ELEMENTS_DIGEST)
        .lines()
        .take_while(|line| !line.is_empty())
        .collect::<Vec<_>>()
        .join("\n");
    let root = fixture(
        "no_body",
        &[
            ("data/elements.tsv", ELEMENTS),
            ("data/elements.tsv.provenance", header_only.as_bytes()),
        ],
    );

    assert_eq!(
        refusals(&root),
        vec![Refusal::MissingField {
            record: "data/elements.tsv.provenance".to_string(),
            field: "a body saying what was done to the numbers between the source and the file"
                .to_string(),
        }]
    );
}

#[test]
fn a_tree_whose_records_answer_every_question_passes() {
    // The neighbour to all of the above, differing from each by one change. It
    // carries both kinds, because the two require different fields, and the one
    // path that needs no record.
    let fetched = a_fetched_record("data/elements.tsv", ELEMENTS_DIGEST);
    let derived = format!(
        "Kind: derived\n\
         File: data/coefficients.tsv\n\
         Hash: {COEFFICIENTS_DIGEST}\n\
         Inputs: data/elements.tsv\n\
         Command: cargo run --package bremsweg-fit\n\
         Commit: 0000000000000000000000000000000000000000\n\
         \n\
         Fitted from the file named above and nothing else.\n"
    );
    let root = fixture(
        "everything_answered",
        &[
            ("data/README.md", b"What is in this directory."),
            ("data/elements.tsv", ELEMENTS),
            ("data/elements.tsv.provenance", fetched.as_bytes()),
            ("data/coefficients.tsv", COEFFICIENTS),
            ("data/coefficients.tsv.provenance", derived.as_bytes()),
        ],
    );

    assert_eq!(
        refusals(&root),
        Vec::new(),
        "a tree that answers every question was refused"
    );
}

#[test]
fn a_directory_of_the_size_this_project_expects_is_judged_quickly() {
    // The size is an expectation and it is written down rather than implied: the
    // vendored subset for the tests, the element and isotope data, and one
    // coefficient file per release, which is tens of files with room to spare at
    // two hundred. If the compilation is ever vendored whole this number is wrong
    // and the measurement is what will say so.
    const FILES: usize = 200;

    let mut written: Vec<(String, Vec<u8>)> = Vec::new();
    for index in 0..FILES {
        let name = format!("data/system-{index:04}.tsv");
        let bytes = format!("# system {index}\n1.0e3 {index}.5\n").into_bytes();
        let digest = format!("sha256:{}", xtask::sha256::hex(&bytes));
        written.push((
            format!("{name}.provenance"),
            a_fetched_record(&name, &digest).into_bytes(),
        ));
        written.push((name, bytes));
    }
    let borrowed: Vec<(&str, &[u8])> = written
        .iter()
        .map(|(path, bytes)| (path.as_str(), bytes.as_slice()))
        .collect();
    let root = fixture("two_hundred_files", &borrowed);

    let started = std::time::Instant::now();
    let found = refusals(&root);
    let took = started.elapsed();

    assert_eq!(found, Vec::new(), "the generated tree was refused");
    // Printed rather than asserted against a bound. A wall clock number is a
    // property of the machine that ran it, so a threshold here would fail on
    // somebody else's hardware for a reason that is not about this check.
    println!("{FILES} files and {FILES} records judged in {took:?}");
}

#[test]
fn the_tree_this_check_ships_in_is_not_refused() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("a directory above the crate")
        .to_path_buf();

    assert_eq!(
        refusals(&root),
        Vec::new(),
        "the data directory of this repository earns a refusal"
    );
}
