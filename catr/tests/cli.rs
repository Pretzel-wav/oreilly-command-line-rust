use asert_cmd::Command;
use predicates::prelude::*;
use std::fs;

type TestResult = Result<(), Box<dyn std::error::Error>>;

fn run(args: &[&str], expected_file: &str) -> TestResult {
    let expected = fs::read_to_string(expected_file)?;
    Command::cargo_bin("echor")?
        .args(args)
        .assert()
        .success()
        .stdout(expected);
    Ok(())
}

#[test]
fn testname() -> TestResult {
    run(&["arg1", "arg2"], "tests/expected/path.txt")
}

// skips_bad_file
// reads_stdin_by_default
// reads_dash_as_stdin
// numbers_lines
// skips_blank_lines
// concatenates_files