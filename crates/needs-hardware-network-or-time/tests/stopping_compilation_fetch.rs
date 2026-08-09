//! The live request for the experimental compilation.
//!
//! Issue #26 asks that the fetch itself be a test here, declaring that it needs
//! the network. Everything the fetch does with a response is covered in the
//! ordinary suite against bytes built in the tree, so what this adds is the one
//! thing that cannot be established without leaving the machine: whether the
//! service is still answering, at the address the tree records, with an archive
//! of the shape the reader expects, holding what the database says that version
//! holds.
//!
//! It is here rather than in the ordinary suite because it depends on somebody
//! else's uptime. A red run here is therefore not automatically a defect in this
//! tree, and the assertions below are written so a reader can tell the two
//! apart: a refusal naming a checksum is about the bytes, a refusal naming a
//! count is about the version, and a refusal from `curl` is about the service or
//! the network.

use bremsweg_fit::{COMPILATION_VERSION, archive, compilation, compilation_request, fetch, sha256};
use bremsweg_needs_hardware_network_or_time::{Requirement, require};

/// What the database states for version 2026-01, read from the front page of
/// <https://www-nds.iaea.org/stopping/> on 2026-08-09, which reported
/// `Version 2026-01 - released on 28th of January, 2026` and
/// `4,440 Experiments | 64,612 Datapoints`.
///
/// Only the datapoint figure is asserted. The experiment figure is a unit the
/// database holds and the published table does not reconstruct, so nothing here
/// counts towards it.
const DATAPOINTS_THE_DATABASE_STATES: usize = 64_612;

/// The members the archive is expected to carry, innermost names.
const TABLES: [&str; 2] = ["StoppingPower.csv", "StoppingPower_refs.csv"];

#[test]
#[ignore = "reaches a service this project does not run"]
fn the_compilation_is_where_the_tree_says_and_holds_what_the_database_states() {
    require(&[Requirement::Network]);

    let request = compilation_request(COMPILATION_VERSION);
    let response = fetch::get(&request).expect("the compilation answered");

    // Every member is checked against the checksum and the length the publisher
    // wrote, inside this call. Bytes that do not match do not come back.
    let outer = archive::members(&response.bytes).expect("the download is an archive");
    let mut tables = Vec::new();
    for member in &outer {
        for inner in archive::members(&member.bytes).expect("the member is an archive") {
            tables.push(inner);
        }
    }

    let mut names: Vec<&str> = tables.iter().map(|t| t.name.as_str()).collect();
    names.sort_unstable();
    let mut expected = TABLES;
    expected.sort_unstable();
    assert_eq!(
        names, expected,
        "the archive does not hold the tables the tree expects"
    );

    let measurements = tables
        .iter()
        .find(|t| t.name == "StoppingPower.csv")
        .expect("the measurements table is in the archive");
    let counts = compilation::counts(&measurements.bytes).expect("the measurements table counts");

    assert_eq!(
        counts.datapoints, DATAPOINTS_THE_DATABASE_STATES,
        "version {COMPILATION_VERSION} yielded {} measurements and the database states {}. \
         Either the download has lost something or the version was restated, and neither is \
         something to pass over",
        counts.datapoints, DATAPOINTS_THE_DATABASE_STATES,
    );
    assert!(
        counts.systems > 0,
        "no ion and target combination came out of {} measurements",
        counts.datapoints
    );
}

/// The second condition of issue #26, measured rather than assumed: two fetches
/// of one version give the same bytes.
///
/// Two requests in one run establishes stability over minutes and nothing
/// longer. What holds over a longer interval is what the provenance record
/// beside a landed table is for, and it is checked by the gate rather than here.
#[test]
#[ignore = "reaches a service this project does not run, twice"]
fn two_fetches_of_one_version_give_the_same_bytes() {
    require(&[Requirement::Network]);

    let request = compilation_request(COMPILATION_VERSION);
    let first = fetch::get(&request).expect("the first request answered");
    let second = fetch::get(&request).expect("the second request answered");

    assert_eq!(
        sha256::hex(&first.bytes),
        sha256::hex(&second.bytes),
        "two fetches of version {COMPILATION_VERSION} gave different bytes, {} and {} long",
        first.bytes.len(),
        second.bytes.len(),
    );
}
