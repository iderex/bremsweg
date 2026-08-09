//! The parse surfaces, fed bytes they did not write.
//!
//! Two functions in this project read bytes somebody else produced: the archive
//! reader and the counter over the published table. Both are reached by a
//! download, so what arrives is whatever a server, a proxy or a broken transfer
//! produced, and a panic in either is a run that ends without saying what it was
//! doing.
//!
//! This is issue #90's adoption, and its bound is stated here rather than in the
//! issue alone. It is a mutation generator over a seed corpus with no coverage
//! feedback: it finds shallow failures near valid input and it does not search.
//! What a coverage guided fuzzer would add, and what adopting one would cost, is
//! in #90.
//!
//! # What it checks and what it does not
//!
//! It checks that neither surface panics. It does not check termination, which
//! needs a timeout this harness has no way to impose, and it does not check for
//! a non-finite result, because the numeric core those inputs would reach is not
//! written. Both absences are in #90 rather than being left for somebody to
//! infer from what is here.
//!
//! # Reproducing a failure
//!
//! The run is deterministic. `SEED` below fixes it, every case is derived from
//! the seed and its index, and a case that fails is printed as its index and its
//! bytes in hexadecimal. A case found that way belongs in the ordinary suite as
//! a named fixture rather than here, because a failure that only the generator
//! can reach is a failure nobody will notice again.

use bremsweg_fit::{archive, compilation, fixture};
use bremsweg_needs_hardware_network_or_time::{Requirement, require};

/// What fixes this run. A constant rather than a clock, so two runs of one
/// commit feed the same bytes and a case reported by one can be reached by the
/// other.
const SEED: u64 = 0x6272_656d_7377_6567;

/// Cases per seed. Eleven corpus entries at this number is 4.4 million cases,
/// which took 19.19 s here against an ordinary suite that finishes in
/// hundredths. That difference is what puts this in the harness rather than in
/// the ordinary suite, and the number is a floor: it is the cost of two readers
/// and a tree with no physics in it yet.
const CASES_PER_SEED: u32 = 400_000;

#[test]
#[ignore = "generates and parses millions of cases, which a per change run cannot afford"]
fn the_archive_reader_does_not_panic_on_bytes_it_did_not_write() {
    require(&[Requirement::Minutes(1)]);
    run_over(&archive_seeds(), "archive::members", |bytes| {
        let _ = archive::members(bytes);
    });
}

#[test]
#[ignore = "generates and parses millions of cases, which a per change run cannot afford"]
fn the_counter_does_not_panic_on_bytes_it_did_not_write() {
    require(&[Requirement::Minutes(1)]);
    run_over(&table_seeds(), "compilation::counts", |bytes| {
        let _ = compilation::counts(bytes);
    });
}

/// The corpus for the archive reader.
///
/// Built here rather than stored as files. The shapes that matter are the
/// published one and the ones next to it, and building them keeps anybody's
/// measurements out of the tree, which is the same reason the offline tests in
/// `crates/fit` build their responses.
fn archive_seeds() -> Vec<Vec<u8>> {
    let table = b"projectile_name,target_name\nHe,Au\n".to_vec();
    let inner = fixture::archive_of(&[("Table.csv", table.clone(), true)]);
    vec![
        // The published shape: an archive inside an archive.
        fixture::archive_of(&[("Table.zip", inner, false)]),
        // One deflated member.
        fixture::archive_of(&[("Table.csv", table.clone(), true)]),
        // One stored member.
        fixture::archive_of(&[("Table.csv", table, false)]),
        // Several members, which is where an offset walked wrongly shows.
        fixture::archive_of(&[
            ("a.csv", b"1\n".to_vec(), false),
            ("b.csv", b"2\n".to_vec(), true),
            ("c.csv", b"3\n".to_vec(), false),
        ]),
        // An archive with no members at all.
        fixture::archive_of(&[]),
    ]
}

/// The corpus for the counter, one shape per thing its reader has to get right.
fn table_seeds() -> Vec<Vec<u8>> {
    vec![
        b"projectile_name,ion_isotope,target_name\nHe,4.0,Au\n".to_vec(),
        b"projectile_name,target_name,citation_reference\nHe,Au,\"a,b,c\"\n".to_vec(),
        b"projectile_name,target_name,comments\nHe,Au,\"first\nsecond\"\n".to_vec(),
        b"projectile_name,target_name,comments\nHe,Au,\"a \"\"quoted\"\" word\"\n".to_vec(),
        b"projectile_name,target_name\r\nHe,Au\r\n".to_vec(),
        b"projectile_name,target_name\n".to_vec(),
    ]
}

/// Feeds `target` mutations of every seed and fails on the first panic, naming
/// the case well enough to reach it again.
///
/// `target` is a function pointer rather than a closure so that it carries
/// nothing across the boundary below, which is what makes catching a panic
/// around it sound without any assertion about unwind safety.
fn run_over(seeds: &[Vec<u8>], named: &str, target: fn(&[u8])) {
    let mut cases = 0u32;
    for (which, seed) in seeds.iter().enumerate() {
        let entry = u64::try_from(which).unwrap_or(0);
        let mut rng = Rng::from(SEED ^ entry.wrapping_mul(0x9E37_79B9_7F4A_7C15));
        for index in 0..CASES_PER_SEED {
            let case = mutate(&mut rng, seed);
            let panicked = std::panic::catch_unwind(|| target(&case)).is_err();
            assert!(
                !panicked,
                "{named} panicked on a generated case.\n\
                 seed {SEED:#018x}, corpus entry {which}, case {index}\n\
                 bytes: {}\n\
                 Put this case in the ordinary suite as a named fixture before fixing it.",
                hex(&case),
            );
            cases = cases.saturating_add(1);
        }
    }
    assert!(cases > 0, "{named} was fed nothing");
}

/// One mutation of `seed`. Near it rather than random, because a reader refuses
/// arbitrary bytes at its first check and never reaches the code worth
/// exercising.
fn mutate(rng: &mut Rng, seed: &[u8]) -> Vec<u8> {
    let mut bytes = seed.to_vec();
    if bytes.is_empty() {
        return bytes;
    }
    let how_many = below(rng, 8).saturating_add(1);
    for _ in 0..how_many {
        match below(rng, 5) {
            // Change one byte to another value.
            0 => {
                let at = below(rng, bytes.len());
                let value = a_byte(rng);
                if let Some(byte) = bytes.get_mut(at) {
                    *byte = value;
                }
            }
            // Set one byte to a value a length or a count field is often wrong
            // by: the largest, the smallest, and one either side of each.
            1 => {
                let interesting = [0u8, 1, 0x7F, 0x80, 0xFE, 0xFF];
                let at = below(rng, bytes.len());
                let pick = below(rng, interesting.len());
                if let (Some(byte), Some(value)) = (bytes.get_mut(at), interesting.get(pick)) {
                    *byte = *value;
                }
            }
            // Cut it short, which is every truncated transfer.
            2 => {
                let keep = below(rng, bytes.len());
                bytes.truncate(keep);
            }
            // Append, which is every response with something after the end.
            3 => {
                let how_much = below(rng, 64);
                for _ in 0..how_much {
                    let value = a_byte(rng);
                    bytes.push(value);
                }
            }
            // Copy a run of it somewhere else, which is what a retried or
            // spliced transfer looks like.
            _ => {
                let from = below(rng, bytes.len());
                let length = below(rng, 32);
                let to = below(rng, bytes.len());
                let taken: Vec<u8> = bytes.iter().skip(from).take(length).copied().collect();
                for (offset, byte) in taken.into_iter().enumerate() {
                    if let Some(slot) = bytes.get_mut(to.saturating_add(offset)) {
                        *slot = byte;
                    }
                }
            }
        }
    }
    bytes
}

/// A number below `bound`, and zero when `bound` is zero.
///
/// The remainder is taken with a checked operation rather than an operator,
/// because this workspace denies an operator that can divide by zero and a
/// generator is exactly where a bound comes from a length that can be empty.
fn below(rng: &mut Rng, bound: usize) -> usize {
    usize::try_from(rng.next())
        .unwrap_or(0)
        .checked_rem(bound)
        .unwrap_or(0)
}

fn a_byte(rng: &mut Rng) -> u8 {
    u8::try_from(below(rng, 256)).unwrap_or(0)
}

fn hex(bytes: &[u8]) -> String {
    // Long cases are cut, with the length said, so a failure message stays
    // readable and nothing pretends the whole case is shown.
    let shown: String = bytes.iter().take(512).map(|b| format!("{b:02x}")).collect();
    if bytes.len() > 512 {
        format!(
            "{shown} ... ({} bytes in all, first 512 shown)",
            bytes.len()
        )
    } else {
        shown
    }
}

/// A generator with a state this tree can read, rather than a dependency.
///
/// It is not a source of randomness for anything that matters; it exists to
/// make a reproducible sequence of choices. The physics uses the generator
/// `docs/decisions/0006` fixes and nothing here reaches it.
struct Rng(u64);

impl Rng {
    const fn from(seed: u64) -> Self {
        // Zero is the one state this recurrence cannot leave.
        Self(if seed == 0 {
            0x9E37_79B9_7F4A_7C15
        } else {
            seed
        })
    }

    fn next(&mut self) -> u64 {
        // xorshift64, whose three shifts are the published ones.
        let mut x = self.0;
        x ^= x << 13;
        x ^= x >> 7;
        x ^= x << 17;
        self.0 = x;
        x
    }
}
