//! The check that refuses a data file without provenance.
//!
//! What a record is, which fields each kind requires and why each one is there is
//! fixed in `docs/decisions/0011`. This is the half that bites.
//!
//! The near miss it is built around is not the missing record, which everybody
//! remembers. It is the record that survives a file being regenerated: the
//! numbers changed, the record still says what it said, and the hash is the only
//! thing in the tree that notices.
//!
//! What it cannot do is judge what a field says. A record whose `Source:` reads
//! "the literature" carries every required field and passes here.

use crate::sha256;
use std::fmt;
use std::path::{Path, PathBuf};

/// The one path under the data directory that needs no record. Documentation
/// about the directory carries no number that enters a calculation. It is a path
/// and not a pattern, so the exception cannot spread to a second file.
const NEEDS_NO_RECORD: &str = "data/README.md";

/// What a record's name adds to the name of the file it describes.
const RECORD_SUFFIX: &str = ".provenance";

/// Required of every record, whatever its kind.
const ALWAYS_REQUIRED: [&str; 3] = ["Kind", "File", "Hash"];

/// Required of a record whose numbers came from somewhere else.
const FETCHED_REQUIRES: [&str; 4] = ["Source", "Source-Version", "Obtained", "Request"];

/// Required of a record whose numbers were computed from other files here.
const DERIVED_REQUIRES: [&str; 3] = ["Inputs", "Command", "Commit"];

/// One thing the check refuses, as a value rather than a message, so a fixture
/// asserts the reason and not the wording.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Refusal {
    /// A file under the data directory with no record beside it.
    NoRecord { file: String },
    /// A record describing a file that is not in the tree.
    NamesNoFile { record: String, named: String },
    /// A record whose `File:` names something other than the file it sits beside.
    /// This is the copied record: it resolves perfectly and describes the wrong
    /// numbers.
    NamesAnotherFile {
        record: String,
        named: String,
        beside: String,
    },
    /// A record whose `Hash:` is not the hash of the file it describes.
    HashDoesNotMatch {
        record: String,
        recorded: String,
        found: String,
    },
    /// A record missing a field its `Kind:` requires, or carrying a `Kind:` that
    /// is neither of the two.
    MissingField { record: String, field: String },
}

impl fmt::Display for Refusal {
    fn fmt(&self, out: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NoRecord { file } => write!(
                out,
                "{file} has no record. Write {file}{RECORD_SUFFIX}, as docs/decisions/0011 sets out"
            ),
            Self::NamesNoFile { record, named } => {
                write!(out, "{record} describes {named}, which is not in the tree")
            }
            Self::NamesAnotherFile {
                record,
                named,
                beside,
            } => write!(
                out,
                "{record} says File: {named} and sits beside {beside}, so it describes \
                 numbers other than the ones it is filed with"
            ),
            Self::HashDoesNotMatch {
                record,
                recorded,
                found,
            } => write!(
                out,
                "{record} records {recorded} and the file hashes to {found}. Either the \
                 file was regenerated and the record was not, or the record is about \
                 another version of it"
            ),
            Self::MissingField { record, field } => {
                write!(out, "{record} is missing {field}")
            }
        }
    }
}

/// Every refusal the data directory earns, in a stable order so two runs on one
/// tree print the same report.
///
/// A tree with no data directory earns none, which is the state this repository
/// is in until the compilation lands.
pub fn refusals(repo_root: &Path) -> Vec<Refusal> {
    let data = repo_root.join("data");
    let mut files = Vec::new();
    collect_files(&data, &mut files);
    files.sort();

    let mut refusals = Vec::new();
    for path in &files {
        let shown = relative_to(repo_root, path);
        if shown == NEEDS_NO_RECORD {
            continue;
        }
        if shown.ends_with(RECORD_SUFFIX) {
            judge_a_record_on_its_own(repo_root, path, &shown, &mut refusals);
        } else {
            judge_a_data_file(repo_root, path, &shown, &mut refusals);
        }
    }
    refusals.sort();
    refusals
}

/// The refusals as a gate leg's verdict.
pub fn as_a_leg(repo_root: &Path) -> Result<(), String> {
    let refusals = refusals(repo_root);
    if refusals.is_empty() {
        return Ok(());
    }
    Err(refusals
        .iter()
        .map(ToString::to_string)
        .collect::<Vec<_>>()
        .join("\n"))
}

/// A record's own obligation, which is that the file it is filed with exists. A
/// record left behind by a deleted file is a record nothing else in this check
/// would ever visit.
fn judge_a_record_on_its_own(
    repo_root: &Path,
    path: &Path,
    shown: &str,
    refusals: &mut Vec<Refusal>,
) {
    let described = shown
        .strip_suffix(RECORD_SUFFIX)
        .unwrap_or(shown)
        .to_string();
    if !repo_root.join(&described).is_file() {
        refusals.push(Refusal::NamesNoFile {
            record: shown.to_string(),
            named: read_field(path, "File").unwrap_or(described),
        });
    }
}

fn judge_a_data_file(repo_root: &Path, path: &Path, shown: &str, refusals: &mut Vec<Refusal>) {
    let record_path = PathBuf::from(format!("{}{RECORD_SUFFIX}", path.display()));
    let Ok(text) = std::fs::read_to_string(&record_path) else {
        refusals.push(Refusal::NoRecord {
            file: shown.to_string(),
        });
        return;
    };
    let record_shown = format!("{shown}{RECORD_SUFFIX}");
    let record = Record::read(&text);

    for field in ALWAYS_REQUIRED {
        if record.value(field).is_none() {
            refusals.push(Refusal::MissingField {
                record: record_shown.clone(),
                field: format!("{field}:"),
            });
        }
    }
    if record.body.is_empty() {
        refusals.push(Refusal::MissingField {
            record: record_shown.clone(),
            field: "a body saying what was done to the numbers between the source and the file"
                .to_string(),
        });
    }

    match record.value("Kind") {
        Some("fetched") => require(&record, &FETCHED_REQUIRES, &record_shown, refusals),
        Some("derived") => require(&record, &DERIVED_REQUIRES, &record_shown, refusals),
        Some(_) => refusals.push(Refusal::MissingField {
            record: record_shown.clone(),
            field: "a Kind: of either fetched or derived".to_string(),
        }),
        // The absent Kind is already refused above, and guessing which set of
        // further fields it owes would report a second failure that is really
        // the first one again.
        None => {}
    }

    if let Some(named) = record.value("File")
        && named != shown
    {
        let refusal = if repo_root.join(named).is_file() {
            Refusal::NamesAnotherFile {
                record: record_shown.clone(),
                named: named.to_string(),
                beside: shown.to_string(),
            }
        } else {
            Refusal::NamesNoFile {
                record: record_shown.clone(),
                named: named.to_string(),
            }
        };
        refusals.push(refusal);
    }

    if let Some(recorded) = record.value("Hash") {
        let Ok(bytes) = std::fs::read(path) else {
            return;
        };
        let found = format!("sha256:{}", sha256::hex(&bytes));
        if recorded != found {
            refusals.push(Refusal::HashDoesNotMatch {
                record: record_shown,
                recorded: recorded.to_string(),
                found,
            });
        }
    }
}

fn require(record: &Record, fields: &[&str], record_shown: &str, refusals: &mut Vec<Refusal>) {
    for field in fields {
        if record.value(field).is_none() {
            refusals.push(Refusal::MissingField {
                record: record_shown.to_string(),
                field: format!("{field}:"),
            });
        }
    }
}

/// A record, read as `Key: value` lines at column zero, then a blank line, then a
/// body.
///
/// A line reader rather than a parser. A record is a format this repository
/// invented, and a check whose value is its exactness should not rest on a
/// library nobody here has a reason to audit.
struct Record {
    fields: Vec<(String, String)>,
    body: String,
}

impl Record {
    fn read(text: &str) -> Self {
        let mut fields = Vec::new();
        let mut lines = text.lines();
        for line in lines.by_ref() {
            if line.trim().is_empty() {
                break;
            }
            // At column zero, or it is not a field. An indented line that looks
            // like one is left to the required-field check to miss, rather than
            // being read as something the writer did not write.
            if line.starts_with(char::is_whitespace) {
                continue;
            }
            if let Some((key, value)) = line.split_once(':') {
                fields.push((key.trim().to_string(), value.trim().to_string()));
            }
        }
        Self {
            fields,
            body: lines.collect::<Vec<_>>().join("\n").trim().to_string(),
        }
    }

    /// The value of `key`, or nothing. An empty value is nothing: a field written
    /// with no value after it is absence with a colon in front of it.
    fn value(&self, key: &str) -> Option<&str> {
        self.fields
            .iter()
            .find(|(name, _)| name == key)
            .map(|(_, value)| value.as_str())
            .filter(|value| !value.is_empty())
    }
}

fn read_field(path: &Path, key: &str) -> Option<String> {
    let text = std::fs::read_to_string(path).ok()?;
    Record::read(&text).value(key).map(ToString::to_string)
}

/// Every file under `directory`, however deep. A directory that is not there
/// contributes nothing rather than failing: the check is about the files that
/// exist.
fn collect_files(directory: &Path, found: &mut Vec<PathBuf>) {
    let Ok(entries) = std::fs::read_dir(directory) else {
        return;
    };
    for entry in entries.filter_map(Result::ok) {
        let path = entry.path();
        if path.is_dir() {
            collect_files(&path, found);
        } else {
            found.push(path);
        }
    }
}

/// A path as the report shows it: relative to the repository root, with forward
/// slashes, so the message reads the same on every platform.
fn relative_to(repo_root: &Path, path: &Path) -> String {
    path.strip_prefix(repo_root)
        .unwrap_or(path)
        .display()
        .to_string()
        .replace('\\', "/")
}
