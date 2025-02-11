use assert_cmd::Command;
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

// #[test]
// fn testname() -> TestResult {
//     run(&["arg1", "arg2"], "tests/expected/path.txt")
// }

#[test]
fn skips_bad_file() -> TestResult {
    run(&["this_doesnt_exist.txt", "inputs/fox.txt"], "tests/expected/skips_bad_file.txt")
}

#[test]
fn reads_stdin_by_default() -> TestResult {
    let input = Command::cargo_bin("ls")?
        .arg("inputs");
    Command::cargo_bin("catr")?
        .write_stdin(input)
        .assert()
        .success()
        .stdout("tests/expected/reads_stdin_by_default.txt");
    Ok(())
}
    
#[test]
fn reads_dash_as_stdin() -> TestResult {
    let input = Command::cargo_bin("ls")
        .arg("inputs");
    Command::cargo_bin("catr")?
        .write_stdin(input)
        .arg("-")
        .assert()
        .success()
        .stdout("tests/expected/reads_dash_as_stdin.txt");
    Ok(())
}

#[test]
fn numbers_lines() -> TestResult {
    run(&["-n", "inputs/the-bustle.txt"], "tests/expected/numbers_lines.txt")
}

#[test]
fn skips_blank_lines() -> TestResult {
    run(&["-b", "inputs/the-bustle.txt"], "tests/expected/skips_blank_lines.txt")
}

#[test]
fn concatenates_files() -> TestResult {
    run(&["inputs/the-bustle.txt", "inputs/fox.txt", "inputs/spiders.txt"], "tests/expected/concatenates_files.txt")
}