//! Reading the tree's Rust sources, for the checks written here that judge
//! them.
//!
//! It is one module because two checks now read the same files and blank the
//! same comments and strings before looking at them, and a second copy of
//! `only_the_code` is a second answer to what a string literal is. The checks
//! themselves stay apart: what they refuse is what they are read for.

use std::path::{Path, PathBuf};

/// Directories no check reads: the build output and git's own store.
const NOT_SOURCE: [&str; 2] = ["target", ".git"];

/// The text with every comment, string and character literal replaced by
/// spaces, so a version number in a message is not read as a float and a
/// commented out assertion is not read at all. Lengths and line breaks are
/// preserved, so an offset into this text is an offset into the original.
pub fn only_the_code(text: &str) -> String {
    #[derive(Clone, Copy, PartialEq)]
    enum In {
        Code,
        LineComment,
        BlockComment,
        Text(char),
    }

    let characters: Vec<char> = text.chars().collect();
    let mut out = String::with_capacity(text.len());
    let mut state = In::Code;
    let mut at = 0usize;

    while let Some(here) = characters.get(at).copied() {
        let next = characters.get(at.saturating_add(1)).copied();
        let mut consumed = 1usize;
        let keep = match state {
            In::Code => match (here, next) {
                ('/', Some('/')) => {
                    state = In::LineComment;
                    false
                }
                ('/', Some('*')) => {
                    state = In::BlockComment;
                    false
                }
                ('"', _) => {
                    state = In::Text(here);
                    false
                }
                // A quote that opens a character literal, and not a lifetime.
                // `&'static str` is in this file and every other one, and
                // reading it as a literal would swallow the code after it.
                ('\'', Some('\\')) => {
                    state = In::Text(here);
                    false
                }
                ('\'', _) if characters.get(at.saturating_add(2)) == Some(&'\'') => {
                    state = In::Text(here);
                    false
                }
                _ => true,
            },
            In::LineComment => {
                if here == '\n' {
                    state = In::Code;
                }
                here == '\n'
            }
            In::BlockComment => {
                if (here, next) == ('*', Some('/')) {
                    state = In::Code;
                    consumed = 2;
                }
                here == '\n'
            }
            In::Text(opener) => {
                if here == '\\' {
                    consumed = 2;
                    false
                } else {
                    if here == opener {
                        state = In::Code;
                    }
                    here == '\n'
                }
            }
        };
        for step in 0..consumed {
            let Some(character) = characters.get(at.saturating_add(step)).copied() else {
                break;
            };
            out.push(if keep && step == 0 {
                character
            } else if character == '\n' {
                '\n'
            } else {
                ' '
            });
        }
        at = at.saturating_add(consumed);
    }
    out
}

/// The line an offset falls on, counting from one.
pub fn line_of(text: &str, at: usize) -> usize {
    text.get(..at)
        .unwrap_or_default()
        .matches('\n')
        .count()
        .saturating_add(1)
}

/// Every Rust file under the tree, sorted, so two runs report the same thing.
pub fn rust_files(repo_root: &Path) -> Vec<PathBuf> {
    let mut found = Vec::new();
    walk(repo_root, &mut found);
    found.sort();
    found
}

fn walk(directory: &Path, found: &mut Vec<PathBuf>) {
    let Ok(entries) = std::fs::read_dir(directory) else {
        return;
    };
    for path in entries.filter_map(Result::ok).map(|entry| entry.path()) {
        let name = path.file_name().unwrap_or_default().to_string_lossy();
        if path.is_dir() {
            if !NOT_SOURCE.contains(&name.as_ref()) {
                walk(&path, found);
            }
        } else if path.extension().is_some_and(|it| it == "rs") {
            found.push(path);
        }
    }
}

/// A path as a report shows it: relative to the repository root, with forward
/// slashes, so a message reads the same on every platform.
pub fn shown(repo_root: &Path, path: &Path) -> String {
    path.strip_prefix(repo_root)
        .unwrap_or(path)
        .display()
        .to_string()
        .replace('\\', "/")
}
