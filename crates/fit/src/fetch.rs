//! Getting the bytes, and recording when they were got.
//!
//! # The means
//!
//! `curl`, as a process. A client that speaks TLS is not something this tree
//! can hold: it is a protocol implementation, a certificate store and a
//! dependency on somebody's root list, and none of that is what this project is
//! for. The force is real and it is held to its smallest surface, which is one
//! process, one argument list, no shell, and nothing read back except the bytes
//! and the response headers.
//!
//! Every platform this project supports ships it. What each of them ships to
//! read an archive does not agree, which is why the archive is read in this
//! tree and only the transfer is not.
//!
//! # Where the date comes from
//!
//! The response's own `Date` header, not this machine's clock. A provenance
//! record answers how far behind a copy is likely to be, and the service's
//! answer to that is better evidence than the clock of whoever ran the command,
//! which may be wrong by a year and nothing would say so. A response carrying
//! no date is refused rather than dated from here.

use std::path::PathBuf;
use std::process::Command;

/// What one request returned.
#[derive(Debug)]
pub struct Response {
    /// The body, as it arrived.
    pub bytes: Vec<u8>,
    /// The date the service put on the response, as `YYYY-MM-DD`.
    pub obtained: String,
}

/// The months, as a response header spells them.
const MONTHS: [&str; 12] = [
    "Jan", "Feb", "Mar", "Apr", "May", "Jun", "Jul", "Aug", "Sep", "Oct", "Nov", "Dec",
];

/// The bytes at `url`, with the date the service dated them.
///
/// # Errors
///
/// When the process cannot be started, when it exits non-zero, which
/// `--fail` makes it do for an HTTP error as well, or when the response carries
/// no readable date.
pub fn get(url: &str) -> Result<Response, String> {
    let headers_at = headers_path();
    let result = Command::new("curl")
        .arg("--silent")
        .arg("--show-error")
        .arg("--location")
        // An HTTP error is an error. Without this curl exits zero and the body
        // is somebody's error page, which would then be hashed and recorded as
        // if it were the compilation.
        .arg("--fail")
        .arg("--max-time")
        .arg("600")
        .arg("--dump-header")
        .arg(&headers_at)
        .arg(url)
        .output()
        .map_err(|e| format!("could not run curl: {e}"))?;

    let headers = std::fs::read_to_string(&headers_at).unwrap_or_default();
    let _ = std::fs::remove_file(&headers_at);

    if !result.status.success() {
        return Err(format!(
            "curl refused {url}: {} {}",
            result.status,
            String::from_utf8_lossy(&result.stderr).trim()
        ));
    }

    let obtained = date_in(&headers).ok_or_else(|| {
        format!("{url} answered with no Date header, so nothing can say when this was obtained")
    })?;

    Ok(Response {
        bytes: result.stdout,
        obtained,
    })
}

/// Where the headers of one run are put. Named for the process so two runs at
/// once do not read each other's.
fn headers_path() -> PathBuf {
    std::env::temp_dir().join(format!("bremsweg-fetch-{}.headers", std::process::id()))
}

/// The date of the last `Date:` header in `headers`, as `YYYY-MM-DD`.
///
/// The last, because a redirect leaves the headers of every hop in the file and
/// the response that carried the bytes is the final one.
///
/// The header's form is fixed by the protocol: a day name, then the day, the
/// month by name and the year. Nothing here parses a general date, and a header
/// this does not recognise produces nothing rather than a guess.
#[must_use]
pub fn date_in(headers: &str) -> Option<String> {
    let mut found = None;
    for line in headers.lines() {
        let Some((name, value)) = line.split_once(':') else {
            continue;
        };
        if !name.trim().eq_ignore_ascii_case("date") {
            continue;
        }
        // `Sun, 09 Aug 2026 07:58:44 GMT`
        let parts: Vec<&str> = value.split_whitespace().collect();
        let (Some(day), Some(month), Some(year)) = (parts.get(1), parts.get(2), parts.get(3))
        else {
            continue;
        };
        let Some(month) = MONTHS.iter().position(|name| name == month) else {
            continue;
        };
        if day.len() != 2 || year.len() != 4 {
            continue;
        }
        if !day.chars().all(|c| c.is_ascii_digit()) || !year.chars().all(|c| c.is_ascii_digit()) {
            continue;
        }
        // The position is zero based and a month is not.
        let month = month.saturating_add(1);
        found = Some(format!("{year}-{month:02}-{day}"));
    }
    found
}

#[cfg(test)]
mod tests {
    use super::date_in;

    #[test]
    fn the_date_a_response_carried() {
        let headers = "HTTP/1.1 200 OK\r\nDate: Sun, 09 Aug 2026 07:58:44 GMT\r\n\
                       Content-Type: application/zip\r\n";
        assert_eq!(date_in(headers).as_deref(), Some("2026-08-09"));
    }

    #[test]
    fn january_and_december_are_the_ends_of_the_table() {
        assert_eq!(
            date_in("Date: Fri, 01 Jan 2027 00:00:00 GMT").as_deref(),
            Some("2027-01-01")
        );
        assert_eq!(
            date_in("Date: Wed, 31 Dec 2025 23:59:59 GMT").as_deref(),
            Some("2025-12-31")
        );
    }

    /// A redirect leaves two responses in the file and the second is the one
    /// that carried the bytes. Taking the first would date the copy from the
    /// hop that carried nothing.
    #[test]
    fn a_redirect_is_dated_by_the_response_that_carried_the_bytes() {
        let headers = "HTTP/1.1 302 Found\r\nDate: Sat, 08 Aug 2026 11:00:00 GMT\r\n\r\n\
                       HTTP/1.1 200 OK\r\nDate: Sun, 09 Aug 2026 07:58:44 GMT\r\n";
        assert_eq!(date_in(headers).as_deref(), Some("2026-08-09"));
    }

    #[test]
    fn a_lower_case_header_name_is_the_same_header() {
        assert_eq!(
            date_in("date: Sun, 09 Aug 2026 07:58:44 GMT").as_deref(),
            Some("2026-08-09")
        );
    }

    #[test]
    fn no_date_header_produces_nothing_rather_than_today() {
        assert_eq!(date_in("HTTP/1.1 200 OK\r\nServer: nowhere\r\n"), None);
    }

    #[test]
    fn a_month_name_the_table_does_not_hold_produces_nothing() {
        assert_eq!(date_in("Date: Sun, 09 Nix 2026 07:58:44 GMT"), None);
    }

    #[test]
    fn a_date_that_is_not_the_shape_the_protocol_fixes_produces_nothing() {
        assert_eq!(date_in("Date: 2026-08-09"), None);
        assert_eq!(date_in("Date: Sun, 9 Aug 26 07:58:44 GMT"), None);
    }
}
