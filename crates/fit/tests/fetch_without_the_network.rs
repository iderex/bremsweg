//! What the fetch does with a response, tested without making one.
//!
//! The live request is one call in `fetch`. Everything after it, which is where
//! the mistakes are, is a function of bytes: reading the archive, checking each
//! member against the checksum the publisher wrote, writing the table, and
//! writing the record beside it. That is what this exercises, so the ordinary
//! suite covers the path and the network is needed only to find out whether the
//! service is still answering.
//!
//! # The responses are built rather than recorded
//!
//! Issue #26 asks for recorded responses. What is here is built to the shape of
//! one: the same container, the same nesting of an archive inside an archive,
//! the same compression method, the same per member checksum. What is not here
//! is anybody's measurements, and that is deliberate rather than convenient.
//! `docs/data-terms.md` reads redistribution of the compilation as unclear and
//! states the conservative reading, and the third entry of issue #1 is where
//! that is decided. Committing even a small slice of the published tables would
//! answer it by accident.
//!
//! The bound that puts on these tests is worth stating rather than leaving to be
//! discovered: they establish that the reader handles the shape, and they cannot
//! establish that the publisher still writes that shape. The test in
//! `crates/needs-hardware-network-or-time` is what finds that out, and it is the
//! reason a live one exists at all.

use bremsweg_fit::fixture;
use bremsweg_fit::landing::{Source, Was, land};

const HEADER: &str = "projectile_name,ion_isotope,target_name,citation_reference\n";

fn a_source() -> Source<'static> {
    Source {
        named: "A compilation",
        version: "2026-01",
        request: "https://example.invalid/version/2026-01",
        obtained: "2026-08-09",
    }
}

/// A response of the shape the database sends: an archive whose members are
/// archives, with the tables inside those.
fn a_response(rows: &str) -> Vec<u8> {
    let table = format!("{HEADER}{rows}").into_bytes();
    let inner = fixture::archive_of(&[("Table.csv", table, true)]);
    fixture::archive_of(&[("Table.zip", inner, false)])
}

/// Cargo makes this directory for integration tests to write in, and
/// `crates/README.md` names it as the only place a test writes.
fn a_directory_to_write_in(case: &str) -> std::path::PathBuf {
    let at = std::path::Path::new(env!("CARGO_TARGET_TMPDIR")).join(format!("landing-{case}"));
    let _ = std::fs::remove_dir_all(&at);
    if let Err(problem) = std::fs::create_dir_all(&at) {
        // Not `expect`, because this is a helper rather than a test and the
        // exemption in `clippy.toml` reaches only a function carrying `#[test]`.
        panic!("could not make {} to write in: {problem}", at.display());
    }
    at
}

#[test]
fn a_table_lands_with_a_record_beside_it() {
    let into = a_directory_to_write_in("lands");
    let written =
        land(&a_response("He,4.0,Au,x\n"), &into, "data", &a_source()).expect("the archive lands");

    assert_eq!(written.len(), 1);
    assert_eq!(written[0].file, "data/Table.csv");
    assert_eq!(written[0].was, Was::New);
    assert!(into.join("Table.csv").is_file());

    let record =
        std::fs::read_to_string(into.join("Table.csv.provenance")).expect("the record was written");
    assert!(record.starts_with("Kind: fetched\n"), "{record}");
    assert!(record.contains("File: data/Table.csv\n"), "{record}");
    assert!(
        record.contains(&format!("Hash: {}\n", written[0].hash)),
        "{record}"
    );
    assert!(record.contains("Source-Version: 2026-01\n"), "{record}");
    assert!(record.contains("Obtained: 2026-08-09\n"), "{record}");
    assert!(record.contains("then the member Table.zip"), "{record}");
}

/// Every field the gate requires of a record of this kind, from the decision
/// that fixes them rather than from what this writer happens to emit.
#[test]
fn the_record_carries_every_field_its_kind_requires() {
    let into = a_directory_to_write_in("fields");
    land(&a_response("He,4.0,Au,x\n"), &into, "data", &a_source()).expect("the archive lands");
    let record =
        std::fs::read_to_string(into.join("Table.csv.provenance")).expect("the record was written");

    for field in [
        "Kind:",
        "File:",
        "Hash:",
        "Source:",
        "Source-Version:",
        "Obtained:",
        "Request:",
    ] {
        assert!(
            record.lines().any(|line| line.starts_with(field)),
            "no {field} in\n{record}"
        );
    }

    let body = record.split("\n\n").nth(1).unwrap_or("");
    assert!(!body.trim().is_empty(), "the record has no body:\n{record}");
}

/// The second condition of issue #26. Two fetches of one version give the same
/// hashes, and the run says so rather than leaving a reader to compare.
#[test]
fn a_second_landing_of_the_same_bytes_is_reported_as_unchanged() {
    let into = a_directory_to_write_in("unchanged");
    let response = a_response("He,4.0,Au,x\n");
    land(&response, &into, "data", &a_source()).expect("the first landing");
    let second = land(&response, &into, "data", &a_source()).expect("the second landing");
    assert_eq!(second[0].was, Was::Unchanged);
}

/// The other half of the same condition, and the half that matters. A version
/// whose bytes moved is named rather than overwritten quietly.
#[test]
fn a_landing_of_different_bytes_names_what_it_replaced() {
    let into = a_directory_to_write_in("changed");
    let first =
        land(&a_response("He,4.0,Au,x\n"), &into, "data", &a_source()).expect("the first landing");
    let second =
        land(&a_response("He,4.0,Si,x\n"), &into, "data", &a_source()).expect("the second landing");

    match &second[0].was {
        Was::Changed { previously } => assert_eq!(*previously, first[0].hash),
        other => panic!("changed bytes were not reported: {other:?}"),
    }
}

#[test]
fn a_table_that_is_not_inside_an_archive_lands_too() {
    let into = a_directory_to_write_in("flat");
    let response = fixture::archive_of(&[("Flat.csv", HEADER.as_bytes().to_vec(), true)]);
    let written = land(&response, &into, "data", &a_source()).expect("the archive lands");
    assert_eq!(written[0].file, "data/Flat.csv");

    let record =
        std::fs::read_to_string(into.join("Flat.csv.provenance")).expect("the record was written");
    assert!(!record.contains("then the member"), "{record}");
}

/// A response that is not the archive: an error page served with a 200, or a
/// transfer that stopped. Nothing may be written, because a file written
/// without its record reddens the gate in every clone and a half written table
/// looks like a small one.
#[test]
fn a_response_that_is_not_an_archive_lands_nothing() {
    let into = a_directory_to_write_in("refused");
    let error_page = b"<html><body>Service unavailable</body></html>".to_vec();
    assert!(land(&error_page, &into, "data", &a_source()).is_err());
    assert_eq!(
        std::fs::read_dir(&into)
            .expect("the directory exists")
            .count(),
        0
    );
}

/// The refusal the archive reader exists for, reached from this end so that it
/// is the fetch that is shown to refuse rather than a function under it. One
/// byte of the compressed stream is changed, which is what a corrupted transfer
/// produces, and what may not happen is a table being written.
#[test]
fn a_response_whose_bytes_are_not_the_published_ones_lands_nothing() {
    let into = a_directory_to_write_in("corrupted");
    let table = format!("{HEADER}{}", "He,4.0,Au,x\n".repeat(64)).into_bytes();
    let mut response = fixture::archive_of(&[("Table.csv", table, true)]);
    let stream_starts_at = 30 + "Table.csv".len() + 8;
    response[stream_starts_at] ^= 0x01;

    let refusal =
        land(&response, &into, "data", &a_source()).expect_err("changed bytes were not refused");
    assert!(refusal.contains("Table.csv"), "{refusal}");
    assert_eq!(
        std::fs::read_dir(&into)
            .expect("the directory exists")
            .count(),
        0
    );
}
