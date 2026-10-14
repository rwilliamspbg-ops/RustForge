//! Counting logic and argument parsing for the `tally` CLI — a domain
//! starter showing CLI testing patterns: pure logic in the lib (unit
//! tests), a thin `main` that only does I/O wiring, and integration
//! tests that spawn the real binary. Hand-rolled arg parsing keeps the
//! example dependency-free and MSRV-clean.
//!
//! Standalone workspace of its own (an empty `[workspace]` table — the
//! `fuzz/` detachment pattern), *not* a member of the RustForge
//! workspace: it adds nothing to the template's default
//! `cargo test --workspace`. See `docs/domain-starters.md`.
//!
//! `unsafe_code`/`missing_docs` lint policy: mirrors the RustForge
//! workspace's `[lints]` table via the crate attributes below.
#![forbid(unsafe_code)]
#![warn(missing_docs)]

/// The usage text shown by `tally --help`.
pub const HELP: &str = "\
usage: tally [-l] [-w] [-c] [--json] [FILE]

Counts lines, words, and bytes (wc-style). With no flags, prints all
three. Reads FILE, or standard input when no FILE is given.

  -l, --lines    count lines
  -w, --words    count whitespace-separated words
  -c, --bytes    count bytes
      --json     print {\"lines\":N,\"words\":N,\"bytes\":N}
  -h, --help     show this help
  -V, --version  show the version";

/// Line/word/byte counts.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct Counts {
    /// Number of lines (a trailing newline does not add one).
    pub lines: usize,
    /// Number of whitespace-separated words.
    pub words: usize,
    /// Number of bytes.
    pub bytes: usize,
}

impl Counts {
    /// Counts `text`.
    pub fn of(text: &str) -> Self {
        Counts {
            lines: text.lines().count(),
            words: text.split_whitespace().count(),
            bytes: text.len(),
        }
    }
}

/// Which counts to print (wc semantics: no flags selected = all three).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Options {
    /// Print the line count.
    pub lines: bool,
    /// Print the word count.
    pub words: bool,
    /// Print the byte count.
    pub bytes: bool,
    /// Print JSON instead of the aligned text format.
    pub json: bool,
    /// Input file; standard input when `None`.
    pub path: Option<String>,
}

impl Options {
    fn any_count_selected(&self) -> bool {
        self.lines || self.words || self.bytes
    }
}

/// Parses CLI arguments into [`Options`]. Errors carry a user-facing
/// message (printed to stderr by `main`).
pub fn parse_args(args: &[String]) -> Result<Options, String> {
    let mut options = Options {
        lines: false,
        words: false,
        bytes: false,
        json: false,
        path: None,
    };
    for arg in args {
        match arg.as_str() {
            "-l" | "--lines" => options.lines = true,
            "-w" | "--words" => options.words = true,
            "-c" | "--bytes" => options.bytes = true,
            "--json" => options.json = true,
            other if other.starts_with('-') => return Err(format!("unknown flag: {other}")),
            other => {
                if options.path.is_some() {
                    return Err(format!("unexpected extra input: {other}"));
                }
                options.path = Some(other.to_string());
            }
        }
    }
    Ok(options)
}

/// Renders `counts` per `options` — all three when no count flag was
/// selected, JSON when `--json`.
pub fn render(counts: Counts, options: Options) -> String {
    let show_lines = options.lines || !options.any_count_selected();
    let show_words = options.words || !options.any_count_selected();
    let show_bytes = options.bytes || !options.any_count_selected();

    if options.json {
        let mut fields: Vec<String> = Vec::new();
        if show_lines {
            fields.push(format!("\"lines\":{}", counts.lines));
        }
        if show_words {
            fields.push(format!("\"words\":{}", counts.words));
        }
        if show_bytes {
            fields.push(format!("\"bytes\":{}", counts.bytes));
        }
        format!("{{{}}}\n", fields.join(","))
    } else {
        let mut parts: Vec<String> = Vec::new();
        if show_lines {
            parts.push(counts.lines.to_string());
        }
        if show_words {
            parts.push(counts.words.to_string());
        }
        if show_bytes {
            parts.push(counts.bytes.to_string());
        }
        format!("{}\n", parts.join(" "))
    }
}

#[cfg(test)]
mod tests {
    use super::{render, Counts, Options};

    fn base_options() -> Options {
        Options {
            lines: false,
            words: false,
            bytes: false,
            json: false,
            path: None,
        }
    }

    #[test]
    fn counts_lines_words_and_bytes() {
        let counts = Counts::of("hello world\nsecond line\n");
        assert_eq!(counts.lines, 2);
        assert_eq!(counts.words, 4);
        assert_eq!(counts.bytes, 24);
    }

    #[test]
    fn trailing_newline_does_not_add_a_line() {
        assert_eq!(Counts::of("a\n").lines, 1);
        assert_eq!(Counts::of("a").lines, 1);
        assert_eq!(Counts::of("").lines, 0);
    }

    #[test]
    fn default_render_is_all_three_space_separated() {
        let counts = Counts::of("hello world\nsecond line\n");
        assert_eq!(render(counts, base_options()), "2 4 24\n");
    }

    #[test]
    fn single_flags_render_one_column() {
        let counts = Counts::of("hello world\nsecond line\n");
        let lines_only = Options {
            lines: true,
            ..base_options()
        };
        assert_eq!(render(counts, lines_only), "2\n");
    }

    #[test]
    fn json_render_is_one_line_of_valid_shape() {
        let counts = Counts::of("a b\nc\n");
        let json = Options {
            json: true,
            ..base_options()
        };
        assert_eq!(render(counts, json), "{\"lines\":2,\"words\":3,\"bytes\":6}\n");
    }

    #[test]
    fn parse_args_accepts_flags_and_one_path() {
        let args: Vec<String> = ["-l", "--json", "file.txt"]
            .iter()
            .map(|s| s.to_string())
            .collect();
        let options = super::parse_args(&args).expect("valid args");
        assert!(options.lines && options.json);
        assert_eq!(options.path.as_deref(), Some("file.txt"));
    }

    #[test]
    fn parse_args_rejects_unknown_flags_and_extra_inputs() {
        let args = vec!["--nope".to_string()];
        assert_eq!(
            super::parse_args(&args),
            Err("unknown flag: --nope".to_string())
        );
        let args = vec!["a.txt".to_string(), "b.txt".to_string()];
        assert_eq!(
            super::parse_args(&args),
            Err("unexpected extra input: b.txt".to_string())
        );
    }
}
