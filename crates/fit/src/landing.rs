//! Putting what was fetched into the data directory, with its record.
//!
//! A file and its record are written together or not at all. That is not
//! tidiness: the gate refuses a data file with no record and a record naming no
//! file, so a fetch that wrote one of the two would redden every clone, and a
//! fetch that wrote the file and an out-of-date record would leave the one thing
//! that notices a changed file saying nothing had changed.
//!
//! What the record has to carry and why each field is there is fixed in
//! `docs/decisions/0011` and is not restated here.

use crate::archive::{self, Member};
use crate::sha256;
use std::path::Path;

/// What appending this to a file's name gives, as `docs/decisions/0011` fixes
/// it.
const RECORD_SUFFIX: &str = ".provenance";

/// What a file was, against what was already there.
#[derive(Debug, PartialEq, Eq)]
pub enum Was {
    /// Nothing of this name was here.
    New,
    /// The same bytes as the record beside it already described.
    Unchanged,
    /// Different bytes from the ones the record described. The second fetch of
    /// one version is supposed to produce none of these, so each one is a
    /// finding rather than a step.
    Changed {
        /// What the record said before this run.
        previously: String,
    },
}

/// One file this run wrote.
#[derive(Debug)]
pub struct Written {
    /// The path, relative to the repository root.
    pub file: String,
    /// The digest recorded for it, `sha256:` and hexadecimal.
    pub hash: String,
    /// What it was against what was there.
    pub was: Was,
    /// How many bytes it holds.
    pub bytes: usize,
}

/// Where the members of an archive of archives came from.
struct Found {
    member: Member,
    /// The names of the archives it was inside, outermost first.
    inside: Vec<String>,
}

/// Every table in `bytes`, which is an archive whose members may themselves be
/// archives.
///
/// One level of nesting is what the compilation uses. Deeper is not read,
/// because a reader that recursed without a bound would follow whatever it was
/// handed.
///
/// # Errors
///
/// Every case in [`archive::Problem`].
fn tables(bytes: &[u8]) -> Result<Vec<Found>, archive::Problem> {
    let mut found = Vec::new();
    for member in archive::members(bytes)? {
        if member.name.ends_with(".zip") {
            for inner in archive::members(&member.bytes)? {
                found.push(Found {
                    member: inner,
                    inside: vec![member.name.clone()],
                });
            }
        } else {
            found.push(Found {
                member,
                inside: Vec::new(),
            });
        }
    }
    Ok(found)
}

/// Writes every table in `bytes` into `directory`, each with its record.
///
/// `shown` is how the directory is named in a record and in the report, which is
/// relative to the repository root so that a record reads the same wherever the
/// repository is checked out.
///
/// # Errors
///
/// When the archive cannot be read, or when a file or a record cannot be
/// written.
pub fn land(
    bytes: &[u8],
    directory: &Path,
    shown: &str,
    source: &Source<'_>,
) -> Result<Vec<Written>, String> {
    let found = tables(bytes).map_err(|problem| problem.to_string())?;
    let whole = format!("sha256:{}", sha256::hex(bytes));

    let mut written = Vec::new();
    for table in found {
        let name = &table.member.name;
        let file = format!("{shown}/{name}");
        let hash = format!("sha256:{}", sha256::hex(&table.member.bytes));
        let record_at = directory.join(format!("{name}{RECORD_SUFFIX}"));

        let was = match previous_hash(&record_at) {
            None => Was::New,
            Some(previously) if previously == hash => Was::Unchanged,
            Some(previously) => Was::Changed { previously },
        };

        let record = record(&file, &hash, &whole, &table.inside, source);
        std::fs::write(directory.join(name), &table.member.bytes)
            .map_err(|e| format!("could not write {file}: {e}"))?;
        std::fs::write(&record_at, record)
            .map_err(|e| format!("could not write {file}{RECORD_SUFFIX}: {e}"))?;

        written.push(Written {
            file,
            hash,
            was,
            bytes: table.member.bytes.len(),
        });
    }
    written.sort_by(|a, b| a.file.cmp(&b.file));
    Ok(written)
}

/// What the source of a fetch is called in every record this run writes.
pub struct Source<'a> {
    /// The compilation, with the identifier a reader uses to find it.
    pub named: &'a str,
    /// The version of it.
    pub version: &'a str,
    /// The address the bytes came from.
    pub request: &'a str,
    /// The date the service dated the response.
    pub obtained: &'a str,
}

fn previous_hash(record_at: &Path) -> Option<String> {
    let text = std::fs::read_to_string(record_at).ok()?;
    for line in text.lines() {
        if line.trim().is_empty() {
            break;
        }
        if let Some(value) = line.strip_prefix("Hash:") {
            return Some(value.trim().to_string());
        }
    }
    None
}

fn record(file: &str, hash: &str, whole: &str, inside: &[String], source: &Source<'_>) -> String {
    let mut request = source.request.to_string();
    for archive_name in inside {
        request.push_str(", then the member ");
        request.push_str(archive_name);
    }
    if !inside.is_empty() {
        request.push_str(", then the member this file is named after");
    }

    format!(
        "Kind: fetched\n\
         File: {file}\n\
         Hash: {hash}\n\
         Source: {}\n\
         Source-Version: {}\n\
         Obtained: {}\n\
         Request: {request}\n\
         \n\
         Taken out of the archive the database publishes for this version and written \
         here unchanged. No unit was converted, no row was dropped, no column was renamed \
         and no row was reordered: these are the publisher's bytes for this member. \
         Reading them into the representation the fit runs against is issue #30, and the \
         file it produces will be a derived one with this file among its inputs.\n\
         \n\
         The whole archive this came out of hashes to {whole}. That is here so the file \
         can be checked against the download as well as against itself, which the hash \
         above does. Every member of the archive also carries a checksum the publisher \
         wrote, and it was checked before this file was written; bytes that did not match \
         it are refused rather than landed.\n",
        source.named, source.version, source.obtained,
    )
}
