//! Refuses a dependency being added to the physics crate.
//!
//! `bremsweg-core` may not depend on anything that performs input or output.
//! That is what lets the whole suite run with no display, no temporary
//! directory and no elevated privilege, and it is a property that decays
//! silently: a dependency added for one convenient function drags in a file
//! system, a clock or a network stack, and nothing about the change looks like
//! it did.
//!
//! What this refuses is a dependency appearing in `crates/core/Cargo.toml` that
//! is not on the allowlist below. What it cannot do is judge whether a crate on
//! the allowlist performs input or output; that judgement is made once, by a
//! person, when the entry and its reason are written. The check is the thing
//! that makes them write one.
//!
//! The manifest is read as text rather than parsed as TOML, because the physics
//! crate has no dependencies and a TOML parser would be the first one. The
//! reader is not general: it recognises the table headers this repository's own
//! manifests use, and where it is unsure it reports a name rather than omitting
//! one, so its errors are refusals rather than silent passes.

/// Crates `bremsweg-core` may depend on, each with the reason it does no input
/// or output. Empty, and an entry added here is a decision somebody argued for.
const ALLOWED: &[(&str, &str)] = &[];

fn main_manifest() -> String {
    // From cargo rather than from the working directory, which a test may
    // assume nothing about.
    let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("Cargo.toml");
    std::fs::read_to_string(&path)
        .unwrap_or_else(|e| panic!("could not read {}: {e}", path.display()))
}

/// Every dependency name any dependency table of `manifest` declares.
fn declared_dependencies(manifest: &str) -> Vec<String> {
    let mut found = Vec::new();
    let mut inside_dependency_table = false;

    for raw in manifest.lines() {
        let line = raw.split('#').next().unwrap_or("").trim();
        if line.is_empty() {
            continue;
        }

        if let Some(header) = section_header(line) {
            match dependency_table(header) {
                // `[dependencies.serde]` names the dependency in the header and
                // its keys are that dependency's own fields, not more names.
                Some(Some(named)) => {
                    found.push(named);
                    inside_dependency_table = false;
                }
                Some(None) => inside_dependency_table = true,
                None => inside_dependency_table = false,
            }
            continue;
        }

        if inside_dependency_table && let Some((key, _)) = line.split_once('=') {
            let name = key.trim().trim_matches('"').trim();
            if !name.is_empty() {
                found.push(name.to_owned());
            }
        }
    }

    found
}

/// The inside of a table header, or `None` if the line is not one.
fn section_header(line: &str) -> Option<&str> {
    let inner = line.strip_prefix('[')?.strip_suffix(']')?;
    Some(inner.trim_start_matches('[').trim_end_matches(']'))
}

/// `None` if `header` is not a dependency table. `Some(None)` if it is one
/// whose keys are dependency names. `Some(Some(name))` if it is one naming a
/// single dependency in the header itself.
fn dependency_table(header: &str) -> Option<Option<String>> {
    const KINDS: [&str; 3] = ["dependencies", "dev-dependencies", "build-dependencies"];

    let segments: Vec<&str> = header.split('.').map(str::trim).collect();
    let at = segments.iter().position(|s| KINDS.contains(s))?;

    match segments.get(at + 1) {
        None => Some(None),
        Some(named) => Some(Some(named.trim_matches('"').trim_matches('\'').to_owned())),
    }
}

#[test]
fn the_physics_crate_declares_no_dependency_outside_the_allowlist() {
    let allowed: Vec<&str> = ALLOWED.iter().map(|(name, _)| *name).collect();
    let refused: Vec<String> = declared_dependencies(&main_manifest())
        .into_iter()
        .filter(|name| !allowed.contains(&name.as_str()))
        .collect();

    assert!(
        refused.is_empty(),
        "bremsweg-core may not depend on anything that performs input or \
         output. Refused: {refused:?}. If one of these genuinely does none, add \
         it to ALLOWED in this file with the reason."
    );
}

#[test]
fn a_dependency_in_the_ordinary_table_is_found() {
    let manifest = "\
[package]\n\
name = \"bremsweg-core\"\n\
\n\
[dependencies]\n\
serde = \"1\"\n";
    assert_eq!(declared_dependencies(manifest), vec!["serde".to_owned()]);
}

#[test]
fn a_dependency_written_as_its_own_table_is_found() {
    let manifest = "\
[dependencies.serde]\n\
version = \"1\"\n\
features = [\"derive\"]\n";
    assert_eq!(declared_dependencies(manifest), vec!["serde".to_owned()]);
}

#[test]
fn a_platform_specific_dependency_is_found() {
    let manifest = "\
[target.'cfg(unix)'.dependencies]\n\
libc = \"0.2\"\n";
    assert_eq!(declared_dependencies(manifest), vec!["libc".to_owned()]);
}

#[test]
fn a_development_and_a_build_dependency_are_found() {
    let manifest = "\
[dev-dependencies]\n\
tempfile = \"3\"\n\
\n\
[build-dependencies]\n\
cc = \"1\"\n";
    assert_eq!(
        declared_dependencies(manifest),
        vec!["tempfile".to_owned(), "cc".to_owned()]
    );
}

#[test]
fn a_table_that_is_not_a_dependency_table_contributes_nothing() {
    let manifest = "\
[package]\n\
name = \"bremsweg-core\"\n\
version = \"0.0.0\"\n\
\n\
[lints]\n\
workspace = true\n\
\n\
[[bin]]\n\
name = \"nothing\"\n";
    assert!(declared_dependencies(manifest).is_empty());
}

#[test]
fn a_commented_out_dependency_is_not_a_dependency() {
    let manifest = "\
[dependencies]\n\
# serde = \"1\"\n";
    assert!(declared_dependencies(manifest).is_empty());
}
