//! The command that obtains the experimental compilation.
//!
//!     cargo fetch-compilation
//!
//! It takes no argument. Which version it fetches is `COMPILATION_VERSION` in
//! this crate, so a clone of one commit obtains what every other clone of that
//! commit obtains.
//!
//! It writes into `data/`, each table with its provenance record beside it, and
//! prints what it wrote, what each table hashes to, whether that differs from
//! what the record beside it said before the run, and how many measurements and
//! how many ion and target combinations arrived. The counts are printed rather
//! than checked, because what they should be compared with is the figure the
//! database states for the version, and that is a reading rather than a
//! constant this tree can hold.
//!
//! Nothing it writes is tracked. `data/.gitignore` keeps the tables out of the
//! repository, because whether the compilation may be redistributed here is the
//! third entry of issue #1 and is open, and `docs/data-terms.md` reads the terms
//! as unclear on exactly that. Running this command is how a clone gets the
//! data, and the record beside each table is what says where it came from.

use bremsweg_fit::landing::{Source, Was};
use bremsweg_fit::{
    COMPILATION, COMPILATION_VERSION, compilation, compilation_request, fetch, landing,
};
use std::path::{Path, PathBuf};

fn main() {
    let unexpected: Vec<String> = std::env::args().skip(1).collect();
    if !unexpected.is_empty() {
        eprintln!(
            "fetch-compilation takes no arguments, and was given: {}",
            unexpected.join(" ")
        );
        std::process::exit(2);
    }

    match run() {
        Ok(report) => print!("{report}"),
        Err(problem) => {
            eprintln!("{problem}");
            std::process::exit(1);
        }
    }
}

fn run() -> Result<String, String> {
    let root = repo_root();
    let directory = root.join("data");
    if !directory.is_dir() {
        return Err(format!("{} is not a directory", directory.display()));
    }

    let request = compilation_request(COMPILATION_VERSION);
    let response = fetch::get(&request)?;

    let source = Source {
        named: COMPILATION,
        version: COMPILATION_VERSION,
        request: &request,
        obtained: &response.obtained,
    };
    let written = landing::land(&response.bytes, &directory, "data", &source)?;

    let mut report = String::new();
    report.push_str(&format!(
        "{COMPILATION}\nversion {COMPILATION_VERSION}, obtained {} from {request}\n\
         the download is {} bytes and hashes to sha256:{}\n\n",
        response.obtained,
        response.bytes.len(),
        bremsweg_fit::sha256::hex(&response.bytes),
    ));

    for table in &written {
        let was = match &table.was {
            Was::New => "new here".to_string(),
            Was::Unchanged => "the same bytes the record beside it already described".to_string(),
            Was::Changed { previously } => format!(
                "DIFFERENT bytes from the ones the record beside it described, which were \
                 {previously}. One version is supposed to give one answer, so this is a \
                 finding rather than an update"
            ),
        };
        report.push_str(&format!(
            "{}\n  {} bytes, {}\n  {was}\n",
            table.file, table.bytes, table.hash,
        ));

        let bytes = std::fs::read(directory.join(name_of(&table.file)))
            .map_err(|e| format!("could not read back {}: {e}", table.file))?;
        match compilation::counts(&bytes) {
            Ok(counts) => report.push_str(&format!(
                "  {} measurements, {} ion and target combinations\n",
                counts.datapoints, counts.systems,
            )),
            Err(problem) => report.push_str(&format!("  not counted as measurements: {problem}\n")),
        }
    }

    report.push_str(
        "\nWhat these counts are: a measurement is a row, and a combination is a distinct \
         ion and target. The database also states a number of experiments, and no grouping \
         of the columns in this table reproduces it, so nothing above is reported under \
         that name.\n",
    );
    Ok(report)
}

fn name_of(file: &str) -> &str {
    file.rsplit('/').next().unwrap_or(file)
}

/// The repository root, from cargo rather than from the working directory,
/// which the command may assume nothing about.
fn repo_root() -> PathBuf {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    manifest_dir
        .ancestors()
        .nth(2)
        .unwrap_or_else(|| {
            panic!(
                "no workspace root two levels above {}",
                manifest_dir.display()
            )
        })
        .to_path_buf()
}
