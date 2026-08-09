//! Counting what a fetch obtained.
//!
//! The database states how many datapoints a version holds, so a fetch that
//! yields a different number has lost something, and a count nobody compares is
//! a count nobody can act on. This is the half that produces the number; the
//! comparison is made by whoever reads the report.
//!
//! It counts and nothing else. Turning these rows into something the fit can
//! run against, with units converted at the edge, is issue #30, and doing any
//! of it here would decide that issue from the wrong end.
//!
//! # Why this reads records rather than lines
//!
//! A field in this table may contain a comma and it may contain a newline: a
//! citation runs to several authors and a comment runs to several lines. At
//! version 2026-01 the file holds 71,680 line feeds and 64,612 measurements, so
//! a reader that counted lines would report a third more measurements than were
//! obtained and nothing about the number would look wrong.

/// The column naming the ion.
const PROJECTILE: &str = "projectile_name";
/// The column naming the target.
const TARGET: &str = "target_name";

/// What a table of measurements was counted to hold.
#[derive(Debug, PartialEq, Eq)]
pub struct Counts {
    /// Measurements, which is rows after the header.
    pub datapoints: usize,
    /// Distinct ion and target combinations.
    ///
    /// This is a combination and not an experiment. The database also states a
    /// number of experiments, and no grouping of these columns reproduces it,
    /// so nothing here is reported under that name.
    pub systems: usize,
}

/// Why a table could not be counted.
#[derive(Debug, PartialEq, Eq)]
pub enum Problem {
    /// The bytes are not text.
    NotText,
    /// The table has no header row.
    Empty,
    /// The header does not name a column the count needs.
    NoSuchColumn { wanted: String },
}

impl std::fmt::Display for Problem {
    fn fmt(&self, out: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NotText => write!(out, "the table is not text"),
            Self::Empty => write!(out, "the table has no header row"),
            Self::NoSuchColumn { wanted } => {
                write!(out, "the header names no column {wanted}")
            }
        }
    }
}

/// What `table` holds, counted rather than read from anywhere.
///
/// # Errors
///
/// Every case in [`Problem`]. A table missing a column the count needs is
/// refused rather than counted as zero, because a zero here reads as an empty
/// database.
pub fn counts(table: &[u8]) -> Result<Counts, Problem> {
    let text = std::str::from_utf8(table).map_err(|_| Problem::NotText)?;
    let mut records = records(text);
    let header = records.next().ok_or(Problem::Empty)?;

    let projectile = column(&header, PROJECTILE)?;
    let target = column(&header, TARGET)?;

    let mut datapoints = 0usize;
    let mut systems = std::collections::BTreeSet::new();
    for record in records {
        // A trailing newline at the end of the file is not a measurement.
        if record.len() == 1 && record[0].is_empty() {
            continue;
        }
        datapoints = datapoints.saturating_add(1);
        let ion = record.get(projectile).cloned().unwrap_or_default();
        let material = record.get(target).cloned().unwrap_or_default();
        systems.insert((ion, material));
    }

    Ok(Counts {
        datapoints,
        systems: systems.len(),
    })
}

fn column(header: &[String], wanted: &str) -> Result<usize, Problem> {
    header
        .iter()
        .position(|name| name == wanted)
        .ok_or_else(|| Problem::NoSuchColumn {
            wanted: wanted.to_string(),
        })
}

/// The records of a comma separated table, each a list of fields.
///
/// A field is quoted where it holds a comma, a newline or a quote, and a quote
/// inside a quoted field is written twice. Nothing else is interpreted: this
/// does not trim, does not convert and does not decide what a field means.
fn records(text: &str) -> impl Iterator<Item = Vec<String>> {
    let mut fields: Vec<String> = Vec::new();
    let mut field = String::new();
    let mut inside_quotes = false;
    let mut after_a_quote = false;
    let mut out: Vec<Vec<String>> = Vec::new();

    for character in text.chars() {
        if after_a_quote {
            after_a_quote = false;
            if character == '"' {
                field.push('"');
                continue;
            }
            inside_quotes = false;
        }
        match character {
            '"' if inside_quotes => after_a_quote = true,
            '"' if field.is_empty() && !inside_quotes => inside_quotes = true,
            ',' if !inside_quotes => {
                fields.push(std::mem::take(&mut field));
            }
            '\n' if !inside_quotes => {
                fields.push(std::mem::take(&mut field));
                out.push(std::mem::take(&mut fields));
            }
            // A carriage return before a record separator belongs to the line
            // ending rather than to the field. One inside a quoted field is
            // kept, because there it is content.
            '\r' if !inside_quotes => {}
            other => field.push(other),
        }
    }
    if !field.is_empty() || !fields.is_empty() {
        fields.push(field);
        out.push(fields);
    }

    out.into_iter()
}

#[cfg(test)]
mod tests {
    use super::{Counts, Problem, counts};

    const HEADER: &str = "projectile_name,ion_isotope,target_name,citation_reference\n";

    #[test]
    fn one_measurement_is_one_datapoint_and_one_system() {
        let table = format!("{HEADER}He,4.0,Au,Smith 1970\n");
        assert_eq!(
            counts(table.as_bytes()),
            Ok(Counts {
                datapoints: 1,
                systems: 1,
            })
        );
    }

    #[test]
    fn two_measurements_of_one_combination_are_one_system() {
        let table = format!("{HEADER}He,4.0,Au,Smith 1970\nHe,4.0,Au,Jones 1981\n");
        assert_eq!(
            counts(table.as_bytes()),
            Ok(Counts {
                datapoints: 2,
                systems: 1,
            })
        );
    }

    #[test]
    fn one_ion_into_two_targets_is_two_systems() {
        let table = format!("{HEADER}He,4.0,Au,Smith 1970\nHe,4.0,Si,Smith 1970\n");
        assert_eq!(
            counts(table.as_bytes()),
            Ok(Counts {
                datapoints: 2,
                systems: 2,
            })
        );
    }

    /// The case that decides whether the count means anything. The published
    /// table carries citations holding commas, and a reader splitting on every
    /// comma would put the target column somewhere else.
    #[test]
    fn a_comma_inside_a_quoted_field_does_not_end_it() {
        let table = format!("{HEADER}He,4.0,Au,\"M.Nigam,J.L.Duggan,C.Yang\"\n");
        assert_eq!(
            counts(table.as_bytes()),
            Ok(Counts {
                datapoints: 1,
                systems: 1,
            })
        );
    }

    /// The same case one step harder, and the one that makes a line count
    /// wrong rather than merely misaligned.
    #[test]
    fn a_newline_inside_a_quoted_field_does_not_end_the_record() {
        let table = format!("{HEADER}He,4.0,Au,\"first line\nsecond line\"\nNe,20.0,Si,x\n");
        assert_eq!(
            counts(table.as_bytes()),
            Ok(Counts {
                datapoints: 2,
                systems: 2,
            })
        );
    }

    #[test]
    fn a_doubled_quote_inside_a_quoted_field_is_one_quote() {
        let table = format!("{HEADER}He,4.0,Au,\"a \"\"quoted\"\" word\"\nNe,20.0,Si,x\n");
        assert_eq!(
            counts(table.as_bytes()),
            Ok(Counts {
                datapoints: 2,
                systems: 2,
            })
        );
    }

    #[test]
    fn a_table_without_a_trailing_newline_still_counts_its_last_row() {
        let table = format!("{HEADER}He,4.0,Au,Smith 1970");
        assert_eq!(
            counts(table.as_bytes()),
            Ok(Counts {
                datapoints: 1,
                systems: 1,
            })
        );
    }

    #[test]
    fn windows_line_endings_do_not_become_part_of_a_field() {
        let table = format!("{HEADER}He,4.0,Au,Smith 1970\r\nHe,4.0,Au,Jones 1981\r\n");
        assert_eq!(
            counts(table.as_bytes()),
            Ok(Counts {
                datapoints: 2,
                systems: 1,
            })
        );
    }

    #[test]
    fn a_header_without_the_columns_the_count_needs_is_refused() {
        let table = "energy,stopping_power\n1,2\n";
        assert_eq!(
            counts(table.as_bytes()),
            Err(Problem::NoSuchColumn {
                wanted: "projectile_name".to_string(),
            })
        );
    }

    #[test]
    fn nothing_at_all_is_refused_rather_than_counted_as_zero() {
        assert_eq!(counts(b""), Err(Problem::Empty));
    }

    #[test]
    fn a_header_with_no_rows_under_it_counts_nothing() {
        assert_eq!(
            counts(HEADER.as_bytes()),
            Ok(Counts {
                datapoints: 0,
                systems: 0,
            })
        );
    }
}
