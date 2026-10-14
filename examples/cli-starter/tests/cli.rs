//! End-to-end tests: spawn the real `tally` binary and assert on its
//! actual stdout/stderr/exit codes — the layer the lib's unit tests
//! can't cover.

use std::io::Write;
use std::process::{Command, Stdio};

/// Runs `tally` with `args`, feeding `input` on stdin, and returns
/// `(stdout, stderr, exit_success)`.
fn tally(args: &[&str], input: &str) -> (String, String, bool) {
    let mut child = Command::new(env!("CARGO_BIN_EXE_tally"))
        .args(args)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("spawn tally");
    child
        .stdin
        .as_mut()
        .expect("stdin piped")
        .write_all(input.as_bytes())
        .expect("write stdin");
    let output = child.wait_with_output().expect("wait for tally");
    (
        String::from_utf8(output.stdout).expect("utf8 stdout"),
        String::from_utf8(output.stderr).expect("utf8 stderr"),
        output.status.success(),
    )
}

#[test]
fn default_output_is_lines_words_bytes() {
    let (stdout, _, ok) = tally(&[], "hello world\nsecond line\n");
    assert!(ok);
    assert_eq!(stdout, "2 4 24\n");
}

#[test]
fn json_output_is_one_line() {
    let (stdout, _, ok) = tally(&["--json"], "a b\nc\n");
    assert!(ok);
    assert_eq!(stdout, "{\"lines\":2,\"words\":3,\"bytes\":6}\n");
}

#[test]
fn single_flag_prints_one_column() {
    let (stdout, _, ok) = tally(&["-w"], "a b c\n");
    assert!(ok);
    assert_eq!(stdout, "3\n");
}

#[test]
fn unknown_flag_fails_with_a_message() {
    let (stdout, stderr, ok) = tally(&["--nope"], "");
    assert!(!ok);
    assert!(stdout.is_empty());
    assert!(stderr.contains("unknown flag: --nope"));
}

#[test]
fn help_and_version_succeed() {
    let (stdout, _, ok) = tally(&["--help"], "");
    assert!(ok);
    assert!(stdout.contains("usage: tally"));
    let (stdout, _, ok) = tally(&["-V"], "");
    assert!(ok);
    assert!(stdout.starts_with("tally "));
}

#[test]
fn reads_a_file_argument() {
    let dir = std::env::temp_dir().join(format!("tally-file-test-{}", std::process::id()));
    std::fs::create_dir_all(&dir).expect("create temp dir");
    let file = dir.join("input.txt");
    std::fs::write(&file, b"one two\nthree\n").expect("write temp file");

    let output = Command::new(env!("CARGO_BIN_EXE_tally"))
        .arg(&file)
        .output()
        .expect("run tally on file");
    assert!(output.status.success());
    assert_eq!(String::from_utf8(output.stdout).unwrap(), "2 3 14\n");

    std::fs::remove_dir_all(&dir).ok();
}

#[test]
fn missing_file_fails_with_a_message() {
    let output = Command::new(env!("CARGO_BIN_EXE_tally"))
        .arg("/nonexistent/tally-input.txt")
        .output()
        .expect("run tally on missing file");
    assert!(!output.status.success());
    assert!(String::from_utf8(output.stderr)
        .unwrap()
        .contains("cannot read"));
}
