//! The check that refuses an equality assertion over floating point numbers.
//!
//! `clippy::float_cmp` refuses the operator spelling, `a == b`, wherever it
//! appears, `assert!(a == b)` included. It does not see inside `assert_eq!`,
//! which is a macro of the standard library and therefore an external one, and
//! that is the spelling a test actually uses. Measured rather than supposed,
//! against a file holding all three spellings:
//!
//! ```text
//! clippy-driver --edition 2024 --test -D clippy::float_cmp fc.rs
//! error: strict comparison of f32 or f64      (the bare a == b line)
//! error: strict comparison of f32 or f64      (the assert!(a == b) line)
//! (nothing about the assert_eq! line)
//! ```
//!
//! So this leg covers the half the linter cannot reach, and neither covers the
//! other.
//!
//! What it cannot do, and it is the larger half: it reads text and has no
//! types. It refuses an equality assertion carrying a floating point literal or
//! a cast to one, and it passes `assert_eq!(computed, expected)` where both
//! sides are floats reached through a name. Nothing in this tree refuses that
//! today.
//!
//! The near miss it is built around is not somebody writing `==` between two
//! measurements. It is the ordinary `assert_eq!(stopping_power(e), 2.35)` that
//! passes on the machine it was written on.

use crate::source::{line_of, only_the_code, rust_files, shown};
use std::fmt;
use std::path::Path;

/// The macros an equality assertion is spelled with.
const ASSERTIONS: [&str; 4] = [
    "assert_eq",
    "assert_ne",
    "debug_assert_eq",
    "debug_assert_ne",
];

/// The one directory whose files exist in order to be refused. Every file in it
/// is a fixture handed to a tool by a test, and none is a member of any target,
/// so nothing compiles them and this leg would be reading its own examples.
///
/// It is a path and not a pattern, so the exception cannot spread to a second
/// directory. It is also a hole: a comparison hidden there passes, and the only
/// thing standing against that is that a file there which no test names is a
/// file with no reason to exist.
const FIXTURES: &str = "xtask/tests/refused";

/// A comparison that means exactness and says why.
pub struct Deliberate {
    /// The file, written the way the report writes it.
    pub file: &'static str,
    /// Text from the line, enough to name the site and no more.
    pub line_contains: &'static str,
    /// Why exactness is the property under test there.
    pub because: &'static str,
}

/// The register of comparisons that are exact on purpose.
///
/// Empty, and that is a statement about this tree rather than a placeholder:
/// the determinism tests `0006` describes and the result round trip `0007`
/// describes are the two that belong here, and neither exists yet. A site is
/// added with the reason it is exact, and `bremsweg_core::close::exactly_equal`
/// is what it should be written with, because that one compares the bits and
/// `assert_eq!` does not.
///
/// It fails closed in both directions. A site not in it is refused, and an
/// entry in it that names no line is refused too, so a waiver outlives neither
/// the site it excuses nor the reader who has to weigh it.
pub const DELIBERATELY_EXACT: &[Deliberate] = &[];

/// One thing this check refuses, as a value rather than a message, so a fixture
/// asserts the reason and not the wording.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Refusal {
    /// An equality assertion over a floating point number.
    FloatEquality {
        file: String,
        line: usize,
        wrote: String,
    },
    /// An entry in the register above that matches nothing in the tree. A
    /// waiver for a site that has gone is a waiver nobody can weigh, and the
    /// register fails closed in this direction too.
    StaleEntry { file: String, line_contains: String },
}

impl fmt::Display for Refusal {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::FloatEquality { file, line, wrote } => write!(
                formatter,
                "{file}:{line} compares floating point numbers for equality: {wrote}. \
                 Use `bremsweg_core::close`, or say at the site why exactness is the \
                 property"
            ),
            Self::StaleEntry {
                file,
                line_contains,
            } => write!(
                formatter,
                "the register of deliberate exact comparisons names {file} containing \
                 `{line_contains}`, and there is no such line"
            ),
        }
    }
}

/// The leg, as the gate runs it.
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

/// Everything this check refuses about one tree.
pub fn refusals(repo_root: &Path) -> Vec<Refusal> {
    refusals_against(repo_root, DELIBERATELY_EXACT)
}

/// The same, against a register given rather than the one above.
///
/// The register is a parameter so that both of its directions can be exercised
/// against a tree arranged for the purpose. With the real register empty, a
/// test that could not supply its own would be asserting that nothing happens.
pub fn refusals_against(repo_root: &Path, register: &[Deliberate]) -> Vec<Refusal> {
    let mut refusals = Vec::new();
    let mut matched = vec![false; register.len()];

    for path in rust_files(repo_root) {
        let shown = shown(repo_root, &path);
        if shown.starts_with(FIXTURES) {
            continue;
        }
        let Ok(text) = std::fs::read_to_string(&path) else {
            continue;
        };
        for (line, wrote) in equality_assertions_over_floats(&text) {
            match excused(register, &shown, &text, line) {
                Some(entry) => matched[entry] = true,
                None => refusals.push(Refusal::FloatEquality {
                    file: shown.clone(),
                    line,
                    wrote,
                }),
            }
        }
    }

    for (entry, was_matched) in matched.iter().enumerate() {
        if !was_matched {
            let Some(declared) = register.get(entry) else {
                continue;
            };
            refusals.push(Refusal::StaleEntry {
                file: declared.file.to_string(),
                line_contains: declared.line_contains.to_string(),
            });
        }
    }

    refusals.sort();
    refusals
}

/// Which register entry, if any, covers a site.
fn excused(register: &[Deliberate], shown: &str, text: &str, line: usize) -> Option<usize> {
    let the_line = text.lines().nth(line.checked_sub(1)?)?;
    register.iter().position(|declared| {
        declared.file == shown
            && the_line.contains(declared.line_contains)
            && !declared.because.trim().is_empty()
    })
}

/// Every equality assertion in `text` whose arguments carry a floating point
/// number, as the line it starts on and what was written.
pub fn equality_assertions_over_floats(text: &str) -> Vec<(usize, String)> {
    let code = only_the_code(text);
    let bytes = code.as_bytes();
    let mut found = Vec::new();
    let mut at = 0;

    while at < bytes.len() {
        let Some(call) = ASSERTIONS
            .iter()
            .find(|name| starts_a_call(&code, at, name))
        else {
            at = at.saturating_add(1);
            continue;
        };
        let Some(opener) = code[at..]
            .find(['(', '[', '{'])
            .map(|it| at.saturating_add(it))
        else {
            break;
        };
        let Some(close) = balanced(bytes, opener) else {
            break;
        };
        let arguments = &code[opener.saturating_add(1)..close];
        if the_compared_pair_is_floating_point(arguments) {
            found.push((
                line_of(&code, at),
                format!("{call}!({})", one_line(arguments)),
            ));
        }
        at = close;
    }

    found
}

/// Whether the macro `name` is invoked at `at`, rather than merely spelled
/// there. A function called `assert_eq_within` is not this.
fn starts_a_call(code: &str, at: usize, name: &str) -> bool {
    if !code[at..].starts_with(name) {
        return false;
    }
    let before = code[..at].chars().next_back();
    if before.is_some_and(|it| it.is_alphanumeric() || it == '_') {
        return false;
    }
    code[at.saturating_add(name.len())..]
        .trim_start()
        .starts_with('!')
}

/// The offset of the delimiter closing the one at `opener`.
fn balanced(bytes: &[u8], opener: usize) -> Option<usize> {
    let open = *bytes.get(opener)?;
    let close = match open {
        b'(' => b')',
        b'[' => b']',
        b'{' => b'}',
        _ => return None,
    };
    let mut depth = 0usize;
    for (at, byte) in bytes.iter().enumerate().skip(opener) {
        if *byte == open {
            depth = depth.saturating_add(1);
        } else if *byte == close {
            depth = depth.saturating_sub(1);
            if depth == 0 {
                return Some(at);
            }
        }
    }
    None
}

/// Whether the two things an assertion compares are floating point numbers.
///
/// Only the first two arguments, because the rest are the failure message and
/// what it formats, and a number in a message is not a comparison. And not when
/// either of them is a boolean literal, because `assert_eq!(is_close(1.0, 2.0),
/// false)` compares two booleans and the floats in it are arguments to the
/// thing being asked about.
fn the_compared_pair_is_floating_point(arguments: &str) -> bool {
    let pair: Vec<&str> = top_level_arguments(arguments).into_iter().take(2).collect();
    if pair.iter().any(|it| matches!(*it, "true" | "false")) {
        return false;
    }
    pair.iter().any(|it| carries_a_float(it))
}

/// An argument list split at the commas that belong to it, rather than at the
/// commas inside a call, a slice or a generic argument.
fn top_level_arguments(arguments: &str) -> Vec<&str> {
    let mut split = Vec::new();
    let mut depth = 0usize;
    let mut start = 0usize;
    for (at, character) in arguments.char_indices() {
        match character {
            '(' | '[' | '{' => depth = depth.saturating_add(1),
            ')' | ']' | '}' => depth = depth.saturating_sub(1),
            ',' if depth == 0 => {
                split.push(arguments.get(start..at).unwrap_or_default().trim());
                start = at.saturating_add(1);
            }
            _ => {}
        }
    }
    split.push(arguments.get(start..).unwrap_or_default().trim());
    split
}

/// Whether one argument carries a floating point number.
///
/// A literal with a decimal point, a literal with an exponent, a literal with a
/// float suffix, or a cast to one. Not the bare word `f64`, which appears in a
/// type argument that has nothing to do with a comparison.
fn carries_a_float(arguments: &str) -> bool {
    if arguments.contains("as f64") || arguments.contains("as f32") {
        return true;
    }
    let bytes = arguments.as_bytes();
    for (at, byte) in bytes.iter().enumerate() {
        if !byte.is_ascii_digit() {
            continue;
        }
        // Only at the start of a number. `t.1.0` is a nested tuple index and
        // `x1.max(y)` is a method call, and neither is a literal.
        let before = at.checked_sub(1).and_then(|it| bytes.get(it)).copied();
        if before.is_some_and(|it| it.is_ascii_alphanumeric() || it == b'_' || it == b'.') {
            continue;
        }
        let rest = arguments.get(at..).unwrap_or_default();
        let digits = rest
            .find(|it: char| !it.is_ascii_digit() && it != '_')
            .unwrap_or(rest.len());
        let tail = rest.get(digits..).unwrap_or_default();
        if tail.starts_with("f64") || tail.starts_with("f32") {
            return true;
        }
        if tail.starts_with('.') && !tail.starts_with("..") {
            let after = tail.get(1..).unwrap_or_default();
            if !after.starts_with(|it: char| it.is_ascii_alphabetic() || it == '_') {
                return true;
            }
        }
        if (tail.starts_with('e') || tail.starts_with('E'))
            && tail
                .get(1..)
                .is_some_and(|it| it.starts_with(['+', '-']) || it.starts_with(char::is_numeric))
        {
            return true;
        }
    }
    false
}

fn one_line(text: &str) -> String {
    text.split_whitespace().collect::<Vec<_>>().join(" ")
}
