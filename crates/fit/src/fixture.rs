//! Archives built to the shape the compilation is published in.
//!
//! Every test of the reader and of the fetch runs against bytes built here.
//! They are constructed rather than recorded, and that is a decision rather than
//! a convenience: `docs/data-terms.md` reads redistribution of the compilation
//! as unclear and states the conservative reading, so a recorded response of it
//! is not put in this tree while entry 3 of issue #1 is open. What is
//! reproduced is the container, the compression method, the checksum and the
//! nesting. What is not reproduced is anybody's measurements.
//!
//! It is public rather than `#[cfg(test)]` because the test of what this crate
//! promises to everything outside it lives in `tests/`, and a crate's test-only
//! items are not visible from there. Nothing on the fetch path calls it.

use crate::crc32;

/// Stored, as [`crate::archive`] spells it.
const STORED: u16 = 0;
/// Deflated, as [`crate::archive`] spells it.
const DEFLATED: u16 = 8;

/// The version-needed field. 2.0 is what a deflated member requires and the
/// value every writer puts there.
const VERSION: u16 = 20;

/// An archive holding `entries`, each a name, its bytes, and whether to deflate
/// it.
///
/// Deflating is a choice per entry because the published archive uses both: its
/// outer members are stored and the tables inside them are deflated, and a
/// reader that only ever met one of the two would not be the reader this needs.
#[must_use]
pub fn archive_of(entries: &[(&str, Vec<u8>, bool)]) -> Vec<u8> {
    let mut bytes = Vec::new();
    let mut directory = Vec::new();

    for (name, content, deflate) in entries {
        let local_at = u32::try_from(bytes.len()).unwrap_or(u32::MAX);
        let (method, stream) = if *deflate {
            (DEFLATED, miniz_oxide::deflate::compress_to_vec(content, 6))
        } else {
            (STORED, content.clone())
        };
        let checksum = crc32::of(content);
        let compressed_length = u32::try_from(stream.len()).unwrap_or(u32::MAX);
        let length = u32::try_from(content.len()).unwrap_or(u32::MAX);
        let name_length = u16::try_from(name.len()).unwrap_or(u16::MAX);

        bytes.extend_from_slice(&0x0403_4b50u32.to_le_bytes());
        bytes.extend_from_slice(&VERSION.to_le_bytes());
        bytes.extend_from_slice(&0u16.to_le_bytes()); // flags
        bytes.extend_from_slice(&method.to_le_bytes());
        bytes.extend_from_slice(&0u16.to_le_bytes()); // modification time
        bytes.extend_from_slice(&0u16.to_le_bytes()); // modification date
        bytes.extend_from_slice(&checksum.to_le_bytes());
        bytes.extend_from_slice(&compressed_length.to_le_bytes());
        bytes.extend_from_slice(&length.to_le_bytes());
        bytes.extend_from_slice(&name_length.to_le_bytes());
        bytes.extend_from_slice(&0u16.to_le_bytes()); // extra field length
        bytes.extend_from_slice(name.as_bytes());
        bytes.extend_from_slice(&stream);

        directory.extend_from_slice(&0x0201_4b50u32.to_le_bytes());
        directory.extend_from_slice(&VERSION.to_le_bytes()); // version made by
        directory.extend_from_slice(&VERSION.to_le_bytes()); // version needed
        directory.extend_from_slice(&0u16.to_le_bytes()); // flags
        directory.extend_from_slice(&method.to_le_bytes());
        directory.extend_from_slice(&0u16.to_le_bytes()); // modification time
        directory.extend_from_slice(&0u16.to_le_bytes()); // modification date
        directory.extend_from_slice(&checksum.to_le_bytes());
        directory.extend_from_slice(&compressed_length.to_le_bytes());
        directory.extend_from_slice(&length.to_le_bytes());
        directory.extend_from_slice(&name_length.to_le_bytes());
        directory.extend_from_slice(&0u16.to_le_bytes()); // extra field length
        directory.extend_from_slice(&0u16.to_le_bytes()); // comment length
        directory.extend_from_slice(&0u16.to_le_bytes()); // first disk
        directory.extend_from_slice(&0u16.to_le_bytes()); // internal attributes
        directory.extend_from_slice(&0u32.to_le_bytes()); // external attributes
        directory.extend_from_slice(&local_at.to_le_bytes());
        directory.extend_from_slice(name.as_bytes());
    }

    let directory_at = u32::try_from(bytes.len()).unwrap_or(u32::MAX);
    let directory_length = u32::try_from(directory.len()).unwrap_or(u32::MAX);
    let count = u16::try_from(entries.len()).unwrap_or(u16::MAX);

    bytes.extend_from_slice(&directory);
    bytes.extend_from_slice(&0x0605_4b50u32.to_le_bytes());
    bytes.extend_from_slice(&0u16.to_le_bytes()); // this disk
    bytes.extend_from_slice(&0u16.to_le_bytes()); // disk holding the directory
    bytes.extend_from_slice(&count.to_le_bytes());
    bytes.extend_from_slice(&count.to_le_bytes());
    bytes.extend_from_slice(&directory_length.to_le_bytes());
    bytes.extend_from_slice(&directory_at.to_le_bytes());
    bytes.extend_from_slice(&0u16.to_le_bytes()); // comment length

    bytes
}

/// Where the central directory of `archive` starts, read from the archive
/// rather than computed, so a test that changes a field there is pointing at
/// the field it means.
///
/// Returns zero for bytes that carry no readable end record, which a test
/// asserting about a directory would then fail on rather than silently pass.
#[must_use]
pub fn central_directory_starts_at(archive: &[u8]) -> usize {
    let Some(end) = archive.len().checked_sub(22) else {
        return 0;
    };
    let Some(field) = archive.get(end.saturating_add(16)..end.saturating_add(20)) else {
        return 0;
    };
    let four: [u8; 4] = field.try_into().unwrap_or([0; 4]);
    usize::try_from(u32::from_le_bytes(four)).unwrap_or(0)
}
