//! The check that refuses a physical constant written anywhere but the one
//! module that holds them.
//!
//! Which module that is, why the constants are held in the units they are held
//! in, and why a charge and a permittivity are stored as their product is fixed
//! in `docs/decisions/0005`. This is the half that bites.
//!
//! The near miss it is built around is not a second definition of the Bohr
//! radius, which a reader would see and argue with. It is the rounded copy in
//! the middle of an expression, `0.5292` written inline because reaching for
//! the module was inconvenient. Nothing about that number looks wrong, and it
//! goes on saying what it says on the day the module is revised.
//!
//! So a literal is judged against the precision it was written to, rather than
//! against the module's digits one by one. A value carrying four digits is
//! refused when it agrees with a constant to four digits, which catches the
//! copy that was rounded as well as the copy that was truncated. Comparing the
//! digit strings would not: `0.529_177` correctly rounded to four digits is
//! `0.5292` and shares only three of them.
//!
//! The magnitude is dropped before the comparison, so the same constant in
//! another unit is refused too. What the check cannot do is see a constant that
//! was folded into another number before it was typed, or one assembled by
//! arithmetic from two others, and nothing in this tree refuses either of
//! those.
//!
//! It reads Rust sources only. A value quoted in a document is prose about the
//! module rather than a second definition of it, and `0005` quotes several.

use crate::source::{line_of, only_the_code, rust_files, shown};
use std::fmt;
use std::path::Path;

/// The module that holds them. A path and not a pattern, so the permission
/// cannot spread to a second file.
pub const MODULE: &str = "crates/core/src/constants.rs";

/// The one directory whose files exist in order to be refused. Nothing compiles
/// them, and a check that read them would be refusing its own examples.
const FIXTURES: &str = "xtask/tests/refused";

/// What a constant's doc comment has to carry, which is the source it was taken
/// from and the revision of that source.
const REQUIRED_FIELDS: [&str; 2] = ["Source:", "Revision:"];

/// The fewest significant digits a value can be policed by.
///
/// Below this the digits of a constant are the digits of ordinary numbers: a
/// bin edge at `6.0`, a tolerance of `0.53`. So a constant written with fewer
/// than this many digits is refused rather than admitted unpoliced, and a
/// literal elsewhere carrying fewer is passed over rather than guessed about.
pub const SHORTEST_POLICEABLE: usize = 4;

/// A constant the module declares, as the check reads it.
#[derive(Debug, PartialEq, Eq)]
pub struct Declared {
    /// Its name, which is what a refusal tells a contributor to reach for.
    pub name: String,
    /// Its significant digits, with the separators, the point, the sign and the
    /// exponent removed, so that the same value written another way still
    /// matches.
    pub digits: String,
    /// The literal as it is written, for the message.
    pub wrote: String,
}

/// One thing this check refuses, as a value rather than a message, so a fixture
/// asserts the reason and not the wording.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Refusal {
    /// The module the rule names is not in the tree. Without it every literal
    /// in every file is unpoliced and the leg would pass in silence.
    NoModule { path: String },
    /// A constant declared without the source it came from or the revision of
    /// that source.
    NoCitation { name: String, field: String },
    /// A constant written with too few digits to be told apart from an ordinary
    /// number, which would make it a constant nothing could police.
    TooFewDigits { name: String, wrote: String },
    /// A physical constant written outside the module. This is the one the rule
    /// exists for.
    SecondCopy {
        file: String,
        line: usize,
        wrote: String,
        name: String,
    },
}

impl fmt::Display for Refusal {
    fn fmt(&self, out: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NoModule { path } => write!(
                out,
                "{path} is not in the tree, so nothing here holds the physical constants \
                 and no literal anywhere is checked against one. docs/decisions/0005 is \
                 where that module is required"
            ),
            Self::NoCitation { name, field } => write!(
                out,
                "{MODULE} declares {name} with no {field} line above it, so nothing says \
                 where the number came from or which revision of it this is"
            ),
            Self::TooFewDigits { name, wrote } => write!(
                out,
                "{MODULE} declares {name} as {wrote}, which carries fewer than \
                 {SHORTEST_POLICEABLE} significant digits. A value that short cannot be \
                 told from an ordinary number, so a copy of it elsewhere would pass"
            ),
            Self::SecondCopy {
                file,
                line,
                wrote,
                name,
            } => write!(
                out,
                "{file}:{line} writes {wrote}, which agrees with {name} in {MODULE} to \
                 every digit it carries. Use the constant: a second copy is what stays \
                 behind when the module is revised"
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
    let Ok(module_text) = std::fs::read_to_string(repo_root.join(MODULE)) else {
        return vec![Refusal::NoModule {
            path: MODULE.to_string(),
        }];
    };

    let declared = declared(&module_text);
    let mut refusals = how_the_module_declares_them(&declared, &module_text);

    for path in rust_files(repo_root) {
        let shown = shown(repo_root, &path);
        if shown == MODULE || shown.starts_with(FIXTURES) {
            continue;
        }
        let Ok(text) = std::fs::read_to_string(&path) else {
            continue;
        };
        refusals.extend(second_copies(&text, &declared, &shown));
    }

    refusals.sort();
    refusals
}

/// What the module owes about the constants it declares: a citation for each,
/// and enough digits for the rule below to reach a copy of it.
pub fn how_the_module_declares_them(declared: &[Declared], module_text: &str) -> Vec<Refusal> {
    let mut refusals = Vec::new();
    for constant in declared {
        let documentation = documentation_above(module_text, &constant.name);
        for field in REQUIRED_FIELDS {
            if !documentation.contains(field) {
                refusals.push(Refusal::NoCitation {
                    name: constant.name.clone(),
                    field: field.to_string(),
                });
            }
        }
        if constant.digits.len() < SHORTEST_POLICEABLE {
            refusals.push(Refusal::TooFewDigits {
                name: constant.name.clone(),
                wrote: constant.wrote.clone(),
            });
        }
    }
    refusals
}

/// Every constant the module declares.
///
/// A line reader rather than a parser. What is read is `pub const NAME: f64 =`
/// followed by a literal, which is the only shape that module is written in. A
/// declaration written another way is one this check does not see, and that
/// hole is narrow because the module is the next file a reader of this check
/// opens.
pub fn declared(module_text: &str) -> Vec<Declared> {
    let mut found = Vec::new();
    for line in only_the_code(module_text).lines() {
        let Some(rest) = line.trim_start().strip_prefix("pub const ") else {
            continue;
        };
        let Some((name, tail)) = rest.split_once(':') else {
            continue;
        };
        let Some((declared_type, value)) = tail.split_once('=') else {
            continue;
        };
        // A constant of another type is not a physical one. The module holds
        // only these two, and a name or a path declared beside them would
        // otherwise be read as a value with no digits in it.
        if !matches!(declared_type.trim(), "f64" | "f32") {
            continue;
        }
        let wrote = value.trim().trim_end_matches(';').trim().to_string();
        found.push(Declared {
            digits: significant_digits(&wrote),
            name: name.trim().to_string(),
            wrote,
        });
    }
    found
}

/// The doc comment lines immediately above a declaration, which is where its
/// citation is written. An attribute between the comment and the declaration
/// does not break the run, because that is a shape somebody will write.
fn documentation_above(module_text: &str, name: &str) -> String {
    let lines: Vec<&str> = module_text.lines().collect();
    let declares = |line: &&str| {
        line.trim_start()
            .strip_prefix("pub const ")
            .is_some_and(|rest| rest.split(':').next().is_some_and(|it| it.trim() == name))
    };
    let Some(at) = lines.iter().position(declares) else {
        return String::new();
    };

    let mut above = Vec::new();
    let mut here = at;
    while let Some(index) = here.checked_sub(1) {
        let line = lines.get(index).copied().unwrap_or_default().trim_start();
        if line.starts_with("///") {
            above.push(line);
        } else if !line.starts_with('#') {
            break;
        }
        here = index;
    }
    above.join("\n")
}

/// Every literal in one file that is a physical constant written out.
///
/// Comments and strings are blanked first, so a value quoted in a doc comment
/// or in a failure message is prose about the module rather than a copy of it.
pub fn second_copies(text: &str, declared: &[Declared], shown: &str) -> Vec<Refusal> {
    let mut refusals = Vec::new();
    for (line, wrote) in float_literals(&only_the_code(text)) {
        let digits = significant_digits(&wrote);
        if digits.len() < SHORTEST_POLICEABLE {
            continue;
        }
        let Some(found) = declared
            .iter()
            .find(|constant| agrees_to_every_digit_written(&digits, &constant.digits))
        else {
            continue;
        };
        refusals.push(Refusal::SecondCopy {
            file: shown.to_string(),
            line,
            wrote,
            name: found.name.clone(),
        });
    }
    refusals
}

/// Whether a literal agrees with a constant to every significant digit the
/// literal carries.
///
/// Both are normalised to a leading digit and a fraction, which is what drops
/// the magnitude: a constant written in another unit differs from this one by
/// its exponent and by nothing else. The distance allowed is half a unit in the
/// last place the literal wrote, so a value correctly rounded to the digits it
/// shows is inside it and a value that genuinely differs there is not.
pub fn agrees_to_every_digit_written(written: &str, constant: &str) -> bool {
    let (Some(here), Some(there)) = (as_a_mantissa(written), as_a_mantissa(constant)) else {
        return false;
    };
    let places = i32::try_from(written.len()).unwrap_or(i32::MAX);
    // Both sides are a leading digit and a fraction, so the last place the
    // literal wrote is ten to the minus (places - 1) and half of it is this.
    let half_a_unit_in_the_last_place = 5.0 / 10.0_f64.powi(places);
    (here - there).abs() < half_a_unit_in_the_last_place
}

/// A digit string read as a leading digit and a fraction, so that `529177` and
/// `52917721` are 5.29177 and 5.2917721 and can be compared with each other.
fn as_a_mantissa(digits: &str) -> Option<f64> {
    let leading = digits.get(..1)?;
    let rest = digits.get(1..).unwrap_or_default();
    format!("{leading}.{rest}").parse().ok()
}

/// The significant digits of a literal: the separators, the point, the sign,
/// the exponent, the suffix and the leading zeros all removed.
pub fn significant_digits(literal: &str) -> String {
    let mantissa = literal
        .split(['e', 'E'])
        .next()
        .unwrap_or_default()
        .trim_end_matches("f64")
        .trim_end_matches("f32");
    mantissa
        .chars()
        .filter(char::is_ascii_digit)
        .skip_while(|digit| *digit == '0')
        .collect()
}

/// Every floating point literal in a piece of code, as the line it is on and
/// what was written.
///
/// A literal with no point and no exponent is an integer, and a count, a bin
/// number or a hash constant is not a physical constant. That is what keeps
/// this check away from the parts of the tree that are full of them.
pub fn float_literals(code: &str) -> Vec<(usize, String)> {
    let bytes = code.as_bytes();
    let mut found = Vec::new();
    let mut at = 0usize;

    while at < bytes.len() {
        let Some(byte) = bytes.get(at).copied() else {
            break;
        };
        if !byte.is_ascii_digit() {
            at = at.saturating_add(1);
            continue;
        }
        // Only at the start of a number. `t.1.0` is a nested tuple index and
        // `x1.max(y)` is a method call, and neither is a literal.
        let before = at.checked_sub(1).and_then(|it| bytes.get(it)).copied();
        if before.is_some_and(|it| it.is_ascii_alphanumeric() || it == b'_' || it == b'.') {
            at = at.saturating_add(1);
            continue;
        }
        let (literal, after) = read_a_number(code, at);
        if is_floating_point(&literal) {
            found.push((line_of(code, at), literal));
        }
        at = after.max(at.saturating_add(1));
    }

    found
}

/// One number starting at `at`, and where it ends.
fn read_a_number(code: &str, at: usize) -> (String, usize) {
    let characters: Vec<char> = code.get(at..).unwrap_or_default().chars().collect();
    let mut taken = String::new();
    let mut index = 0usize;
    let mut seen_point = false;
    let mut seen_exponent = false;

    while let Some(here) = characters.get(index).copied() {
        let next = characters.get(index.saturating_add(1)).copied();
        let keep = if here.is_ascii_digit() || here == '_' {
            true
        } else if here == '.' && !seen_point && !seen_exponent {
            // A point, but not the one that opens a range and not the one that
            // starts a method call.
            let ends_the_number =
                next == Some('.') || next.is_some_and(|it| it.is_alphabetic() || it == '_');
            seen_point = !ends_the_number;
            !ends_the_number
        } else if (here == 'e' || here == 'E') && !seen_exponent {
            let signed = next.is_some_and(|it| it == '+' || it == '-')
                && characters
                    .get(index.saturating_add(2))
                    .is_some_and(char::is_ascii_digit);
            if signed {
                taken.push(here);
                taken.push(next.unwrap_or('+'));
                index = index.saturating_add(2);
                seen_exponent = true;
                continue;
            }
            seen_exponent = next.is_some_and(|it| it.is_ascii_digit());
            seen_exponent
        } else {
            false
        };
        if !keep {
            break;
        }
        taken.push(here);
        index = index.saturating_add(1);
    }

    // The suffix is part of the literal and says nothing about its value, so it
    // is taken and then ignored.
    let ends_at = at.saturating_add(index);
    for suffix in ["f64", "f32"] {
        if code
            .get(ends_at..)
            .is_some_and(|rest| rest.starts_with(suffix))
        {
            taken.push_str(suffix);
            return (taken, ends_at.saturating_add(suffix.len()));
        }
    }

    (taken, ends_at)
}

/// Whether a literal is a floating point one rather than an integer.
fn is_floating_point(literal: &str) -> bool {
    literal.contains('.')
        || literal.contains('e')
        || literal.contains('E')
        || literal.ends_with("f64")
        || literal.ends_with("f32")
}
