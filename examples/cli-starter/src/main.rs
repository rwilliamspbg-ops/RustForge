//! The `tally` binary — deliberately thin: it only wires I/O around
//! `cli_starter`'s pure logic (argument parsing, counting, rendering),
//! which is what the unit tests cover. The real end-to-end behavior is
//! covered by the integration tests in `tests/cli.rs`, which spawn this
//! binary.

use std::io::{Read, Write};
use std::process::ExitCode;

fn main() -> ExitCode {
    let args: Vec<String> = std::env::args().skip(1).collect();

    if args.iter().any(|a| a == "-h" || a == "--help") {
        println!("{}", cli_starter::HELP);
        return ExitCode::SUCCESS;
    }
    if args.iter().any(|a| a == "-V" || a == "--version") {
        println!("tally {}", env!("CARGO_PKG_VERSION"));
        return ExitCode::SUCCESS;
    }

    let options = match cli_starter::parse_args(&args) {
        Ok(options) => options,
        Err(message) => {
            eprintln!("tally: {message}");
            eprintln!("(try --help)");
            return ExitCode::FAILURE;
        }
    };

    let input = match options.path.as_deref() {
        Some(path) => match std::fs::read_to_string(path) {
            Ok(contents) => contents,
            Err(err) => {
                eprintln!("tally: cannot read {path}: {err}");
                return ExitCode::FAILURE;
            }
        },
        None => {
            let mut buffer = String::new();
            if let Err(err) = std::io::stdin().read_to_string(&mut buffer) {
                eprintln!("tally: cannot read standard input: {err}");
                return ExitCode::FAILURE;
            }
            buffer
        }
    };

    let output = cli_starter::render(cli_starter::Counts::of(&input), options);
    print!("{output}");
    // A broken pipe (e.g. `tally | head -1`) is not an error worth
    // reporting — the consumer just stopped reading.
    if std::io::stdout().flush().is_err() {
        return ExitCode::FAILURE;
    }
    ExitCode::SUCCESS
}
