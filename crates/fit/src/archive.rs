//! Reading the archive the compilation is published as.
//!
//! The download is a zip holding two zips, and the tables are inside those. So
//! nothing can count a measurement in this tree without opening one, and this
//! is where that happens.
//!
//! # What is read here and what is not
//!
//! The container is read here. Its headers are fixed-width little-endian
//! fields in a format that has not moved in decades, and reading them is a
//! hundred lines whose every branch is visible to this repository's own lints
//! and suite.
//!
//! The compressed stream is not read here, and the reason is the failure mode
//! rather than the effort. An inflate that gets one bit wrong does not stop; it
//! yields a different number. A stopping power wrong in its third digit passes
//! every other check in this tree, and it would arrive through the one path
//! nobody re-reads. The argument in full is in the pull request that landed
//! this and in issue #26.
//!
//! What makes either choice safe is the same and it is in this file rather
//! than in the decompressor: every member carries a CRC-32 and an uncompressed
//! length in the archive, both are checked here, and bytes that do not match
//! what the publisher recorded are refused rather than returned. That turns the
//! quiet failure into a loud one whichever code does the inflating.
//!
//! # What it refuses
//!
//! Anything it is not sure about. A member compressed by a method other than
//! the two below, an archive that uses the 64-bit extensions, a header that
//! runs off the end of the bytes. A reader that guesses at a field it does not
//! recognise is a reader that returns numbers nobody can trace.

use crate::crc32;
use std::fmt;

/// Stored: the member's bytes are in the archive as they are.
const STORED: u16 = 0;
/// Deflated: the member's bytes are a raw DEFLATE stream.
const DEFLATED: u16 = 8;

/// The four signatures the reader looks for.
const END_OF_CENTRAL_DIRECTORY: u32 = 0x0605_4b50;
const CENTRAL_DIRECTORY_ENTRY: u32 = 0x0201_4b50;
const LOCAL_HEADER: u32 = 0x0403_4b50;

/// The value a 32-bit size or count field carries when the real one is in a
/// 64-bit extension. Seeing it is what tells this reader the archive is a shape
/// it does not read.
const NEEDS_THE_64_BIT_EXTENSION_U32: u32 = 0xFFFF_FFFF;
const NEEDS_THE_64_BIT_EXTENSION_U16: u16 = 0xFFFF;

/// The largest end-of-central-directory record: the fixed part plus the longest
/// comment its length field can express. Nothing before that in the bytes can
/// be the record, so this bounds the search backwards.
const LONGEST_END_RECORD: usize = 22 + 65535;

/// One member of an archive, with its bytes as the publisher wrote them.
#[derive(Debug, PartialEq, Eq)]
pub struct Member {
    /// The name the archive records, with forward slashes as the format
    /// requires.
    pub name: String,
    /// The uncompressed bytes, checked against the length and the checksum the
    /// archive recorded for them.
    pub bytes: Vec<u8>,
}

/// Why an archive was refused.
///
/// A value rather than a message, so a fixture asserts the reason and not the
/// wording.
#[derive(Debug, PartialEq, Eq)]
pub enum Problem {
    /// No end-of-central-directory record, so these bytes are not an archive
    /// this reader recognises, or they are truncated.
    NotAnArchive,
    /// A header ran off the end of the bytes.
    Truncated { at: usize },
    /// A signature was not the one the format puts there.
    WrongSignature { expected: u32, found: u32 },
    /// A member compressed by a method this reader does not read.
    UnreadableMethod { name: String, method: u16 },
    /// The archive uses the 64-bit extensions, which this reader does not read.
    NeedsThe64BitExtension,
    /// The bytes came out at a length other than the one the archive recorded.
    WrongLength {
        name: String,
        recorded: u64,
        found: usize,
    },
    /// The bytes came out with a checksum other than the one the archive
    /// recorded. This is the refusal the whole file exists for.
    WrongChecksum {
        name: String,
        recorded: u32,
        found: u32,
    },
    /// The decompressor refused the stream.
    NotAValidStream { name: String, reason: String },
    /// A member name that is not text.
    NameIsNotText { at: usize },
}

impl fmt::Display for Problem {
    fn fmt(&self, out: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NotAnArchive => write!(
                out,
                "no end of central directory record, so these bytes are not an archive or are \
                 truncated"
            ),
            Self::Truncated { at } => write!(out, "a header at byte {at} runs off the end"),
            Self::WrongSignature { expected, found } => {
                write!(
                    out,
                    "expected the signature {expected:#010x}, found {found:#010x}"
                )
            }
            Self::UnreadableMethod { name, method } => write!(
                out,
                "{name} is compressed by method {method}, and this reads only {STORED} and \
                 {DEFLATED}"
            ),
            Self::NeedsThe64BitExtension => write!(
                out,
                "the archive uses the 64 bit extensions, which this does not read"
            ),
            Self::WrongLength {
                name,
                recorded,
                found,
            } => write!(
                out,
                "{name} came out {found} bytes long and the archive records {recorded}"
            ),
            Self::WrongChecksum {
                name,
                recorded,
                found,
            } => write!(
                out,
                "{name} came out with checksum {found:#010x} and the archive records \
                 {recorded:#010x}, so these are not the bytes that were published"
            ),
            Self::NotAValidStream { name, reason } => {
                write!(out, "{name} is not a stream this can read: {reason}")
            }
            Self::NameIsNotText { at } => {
                write!(out, "the member name at byte {at} is not text")
            }
        }
    }
}

/// Every member of `archive`, in the order the central directory lists them.
///
/// # Errors
///
/// Every case in [`Problem`]. The reader refuses rather than skipping a member
/// it cannot read, because an archive half of which was understood is not a
/// smaller archive, it is an unknown one.
pub fn members(archive: &[u8]) -> Result<Vec<Member>, Problem> {
    let end = end_of_central_directory(archive).ok_or(Problem::NotAnArchive)?;

    let count = u16_at(archive, at(end, 10)?)?;
    if count == NEEDS_THE_64_BIT_EXTENSION_U16 {
        return Err(Problem::NeedsThe64BitExtension);
    }
    let mut cursor = usize::try_from(u32_at(archive, at(end, 16)?)?).unwrap_or(usize::MAX);

    let mut members = Vec::with_capacity(usize::from(count));
    for _ in 0..count {
        let (member, next) = one_member(archive, cursor)?;
        members.push(member);
        cursor = next;
    }
    Ok(members)
}

/// The offset of the end-of-central-directory record, found by searching
/// backwards for its signature.
///
/// Backwards because the record is last and its own length is variable. The
/// search stops at the earliest byte the record could start at, so a large
/// archive is not scanned end to end for a four byte pattern.
fn end_of_central_directory(archive: &[u8]) -> Option<usize> {
    let fixed_part = 22usize;
    let last_possible = archive.len().checked_sub(fixed_part)?;
    let earliest = last_possible.saturating_sub(LONGEST_END_RECORD.saturating_sub(fixed_part));
    (earliest..=last_possible)
        .rev()
        .find(|start| u32_at(archive, *start) == Ok(END_OF_CENTRAL_DIRECTORY))
}

/// The member whose central directory entry starts at `entry`, and where the
/// next entry starts.
fn one_member(archive: &[u8], entry: usize) -> Result<(Member, usize), Problem> {
    expect_signature(archive, entry, CENTRAL_DIRECTORY_ENTRY)?;

    let method = u16_at(archive, at(entry, 10)?)?;
    let recorded_checksum = u32_at(archive, at(entry, 16)?)?;
    let compressed_size = u32_at(archive, at(entry, 20)?)?;
    let recorded_length = u32_at(archive, at(entry, 24)?)?;
    let name_length = usize::from(u16_at(archive, at(entry, 28)?)?);
    let extra_length = usize::from(u16_at(archive, at(entry, 30)?)?);
    let comment_length = usize::from(u16_at(archive, at(entry, 32)?)?);
    let local = usize::try_from(u32_at(archive, at(entry, 42)?)?).unwrap_or(usize::MAX);

    if compressed_size == NEEDS_THE_64_BIT_EXTENSION_U32
        || recorded_length == NEEDS_THE_64_BIT_EXTENSION_U32
    {
        return Err(Problem::NeedsThe64BitExtension);
    }

    let name_at = at(entry, 46)?;
    let name = text_at(archive, name_at, name_length)?;

    // The local header repeats the name and the extra field with lengths of its
    // own, and it is the local ones that say where the data starts. They are
    // read from the local header rather than assumed equal to the central ones,
    // which they are allowed to differ from.
    expect_signature(archive, local, LOCAL_HEADER)?;
    let local_name_length = usize::from(u16_at(archive, at(local, 26)?)?);
    let local_extra_length = usize::from(u16_at(archive, at(local, 28)?)?);
    let data_at = at(local, 30)?
        .checked_add(local_name_length)
        .and_then(|o| o.checked_add(local_extra_length))
        .ok_or(Problem::Truncated { at: local })?;

    let compressed = slice(
        archive,
        data_at,
        usize::try_from(compressed_size).unwrap_or(usize::MAX),
    )?;
    let bytes = match method {
        STORED => compressed.to_vec(),
        DEFLATED => miniz_oxide::inflate::decompress_to_vec(compressed).map_err(|e| {
            Problem::NotAValidStream {
                name: name.clone(),
                reason: e.to_string(),
            }
        })?,
        other => {
            return Err(Problem::UnreadableMethod {
                name,
                method: other,
            });
        }
    };

    if u64::try_from(bytes.len()).unwrap_or(u64::MAX) != u64::from(recorded_length) {
        return Err(Problem::WrongLength {
            name,
            recorded: u64::from(recorded_length),
            found: bytes.len(),
        });
    }
    let found = crc32::of(&bytes);
    if found != recorded_checksum {
        return Err(Problem::WrongChecksum {
            name,
            recorded: recorded_checksum,
            found,
        });
    }

    let next = at(entry, 46)?
        .checked_add(name_length)
        .and_then(|o| o.checked_add(extra_length))
        .and_then(|o| o.checked_add(comment_length))
        .ok_or(Problem::Truncated { at: entry })?;
    Ok((Member { name, bytes }, next))
}

fn expect_signature(archive: &[u8], offset: usize, expected: u32) -> Result<(), Problem> {
    let found = u32_at(archive, offset)?;
    if found == expected {
        Ok(())
    } else {
        Err(Problem::WrongSignature { expected, found })
    }
}

/// `base + offset`, refused rather than wrapped.
fn at(base: usize, offset: usize) -> Result<usize, Problem> {
    base.checked_add(offset)
        .ok_or(Problem::Truncated { at: base })
}

fn slice(archive: &[u8], from: usize, length: usize) -> Result<&[u8], Problem> {
    let to = at(from, length)?;
    archive.get(from..to).ok_or(Problem::Truncated { at: from })
}

fn u16_at(archive: &[u8], offset: usize) -> Result<u16, Problem> {
    let bytes = slice(archive, offset, 2)?;
    let pair: [u8; 2] = bytes
        .try_into()
        .map_err(|_| Problem::Truncated { at: offset })?;
    Ok(u16::from_le_bytes(pair))
}

fn u32_at(archive: &[u8], offset: usize) -> Result<u32, Problem> {
    let bytes = slice(archive, offset, 4)?;
    let four: [u8; 4] = bytes
        .try_into()
        .map_err(|_| Problem::Truncated { at: offset })?;
    Ok(u32::from_le_bytes(four))
}

fn text_at(archive: &[u8], offset: usize, length: usize) -> Result<String, Problem> {
    let bytes = slice(archive, offset, length)?;
    String::from_utf8(bytes.to_vec()).map_err(|_| Problem::NameIsNotText { at: offset })
}

#[cfg(test)]
mod tests {
    use super::{Member, Problem, members};
    use crate::fixture;

    #[test]
    fn a_stored_member_comes_out_as_it_went_in() {
        let archive = fixture::archive_of(&[("a.csv", b"one,two\n1,2\n".to_vec(), false)]);
        assert_eq!(
            members(&archive),
            Ok(vec![Member {
                name: "a.csv".to_string(),
                bytes: b"one,two\n1,2\n".to_vec(),
            }])
        );
    }

    #[test]
    fn a_deflated_member_comes_out_inflated() {
        // Long enough that deflate has something to do, so this is a test of
        // the compressed path rather than of a stored one wearing its label.
        let content = "energy,stopping\n".repeat(64).into_bytes();
        let archive = fixture::archive_of(&[("b.csv", content.clone(), true)]);
        assert_eq!(
            members(&archive),
            Ok(vec![Member {
                name: "b.csv".to_string(),
                bytes: content,
            }])
        );
    }

    #[test]
    fn an_archive_inside_an_archive_is_read_by_reading_it_twice() {
        // The shape the compilation is published in: the outer members are
        // stored archives and the tables are inside them.
        let inner = fixture::archive_of(&[("inner.csv", b"1,2,3\n".to_vec(), true)]);
        let outer = fixture::archive_of(&[("inner.zip", inner, false)]);

        let outer_members = members(&outer).expect("the outer archive reads");
        assert_eq!(outer_members.len(), 1);
        let inner_members = members(&outer_members[0].bytes).expect("the inner archive reads");
        assert_eq!(inner_members[0].name, "inner.csv");
        assert_eq!(inner_members[0].bytes, b"1,2,3\n");
    }

    #[test]
    fn several_members_come_out_in_the_order_the_directory_lists_them() {
        let archive = fixture::archive_of(&[
            ("first.csv", b"1\n".to_vec(), false),
            ("second.csv", b"2\n".to_vec(), true),
            ("third.csv", b"3\n".to_vec(), false),
        ]);
        let names: Vec<String> = members(&archive)
            .expect("the archive reads")
            .into_iter()
            .map(|m| m.name)
            .collect();
        assert_eq!(names, ["first.csv", "second.csv", "third.csv"]);
    }

    /// The refusal this module exists for. One byte of the compressed stream is
    /// changed, which is what a corrupted transfer or a wrong decompressor
    /// produces, and the bytes that come out are numbers rather than an error.
    #[test]
    fn a_member_whose_bytes_are_not_the_published_ones_is_refused() {
        let content = "energy,stopping\n".repeat(64).into_bytes();
        let mut archive = fixture::archive_of(&[("b.csv", content, true)]);
        let stream_starts_at = 30 + "b.csv".len();
        archive[stream_starts_at + 8] ^= 0x01;

        match members(&archive) {
            Err(Problem::WrongChecksum { name, .. }) => assert_eq!(name, "b.csv"),
            // A flipped bit can also make the stream itself unreadable, and
            // that is the same refusal for this test's purpose: what may not
            // happen is bytes coming back.
            Err(Problem::NotAValidStream { name, .. }) => assert_eq!(name, "b.csv"),
            other => panic!("changed bytes were not refused: {other:?}"),
        }
    }

    /// The same case one step earlier: bytes that inflate cleanly and are the
    /// wrong ones. Built by recording a checksum that is not theirs, which is
    /// what a member swapped for another member looks like.
    #[test]
    fn a_stored_member_whose_recorded_checksum_is_another_members_is_refused() {
        let mut archive = fixture::archive_of(&[("a.csv", b"one,two\n1,2\n".to_vec(), false)]);
        let checksum_in_the_directory = fixture::central_directory_starts_at(&archive) + 16;
        archive[checksum_in_the_directory] ^= 0xFF;

        match members(&archive) {
            Err(Problem::WrongChecksum {
                name,
                recorded,
                found,
            }) => {
                assert_eq!(name, "a.csv");
                assert_ne!(recorded, found);
            }
            other => panic!("a wrong checksum was not refused: {other:?}"),
        }
    }

    #[test]
    fn a_member_whose_recorded_length_is_wrong_is_refused() {
        let mut archive = fixture::archive_of(&[("a.csv", b"one,two\n1,2\n".to_vec(), false)]);
        let length_in_the_directory = fixture::central_directory_starts_at(&archive) + 24;
        archive[length_in_the_directory] = archive[length_in_the_directory].wrapping_add(1);

        match members(&archive) {
            Err(Problem::WrongLength { name, .. }) => assert_eq!(name, "a.csv"),
            other => panic!("a wrong length was not refused: {other:?}"),
        }
    }

    #[test]
    fn a_method_this_does_not_read_is_refused_rather_than_guessed_at() {
        let mut archive = fixture::archive_of(&[("a.csv", b"one,two\n1,2\n".to_vec(), false)]);
        let method_in_the_directory = fixture::central_directory_starts_at(&archive) + 10;
        archive[method_in_the_directory] = 14; // LZMA, which this does not read.

        assert_eq!(
            members(&archive),
            Err(Problem::UnreadableMethod {
                name: "a.csv".to_string(),
                method: 14,
            })
        );
    }

    #[test]
    fn bytes_that_are_not_an_archive_are_refused() {
        assert_eq!(
            members(b"energy,stopping\n1,2\n"),
            Err(Problem::NotAnArchive)
        );
        assert_eq!(members(b""), Err(Problem::NotAnArchive));
    }

    #[test]
    fn an_archive_cut_short_is_refused() {
        let archive = fixture::archive_of(&[("a.csv", b"one,two\n1,2\n".to_vec(), false)]);
        let cut = &archive[..archive.len() - 40];
        assert!(members(cut).is_err());
    }
}
