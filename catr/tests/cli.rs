use anyhow::Result;
use assert_cmd::Command;
use std::fs;
use predicates::prelude::*;
use rand::{distributions::Alphanumeric, Rng};

const PRG: &str = "catr";
const EMPTY: &str = "tests/inputs/empty.txt";
const FOX: &str = "tests/inputs/fox.txt";
const SPIDERS: &str = "tests/inputs/spiders.txt";
const BUSTLE: &str = "tests/inputs/the-bustle.txt";

#[test]
fn usage() -> Result<()> {
    for flag in &["-h", "--help"] {
        Command::cargo_bin(PRG)?
            .arg(flag)
            .assert()
            .stdout(predicate::str::contains("Usage"));
    }
    Ok(())
}

fn gen_bad_file() -> String {
    loop {
        let filename: String = rand::thread_rng()   // create a single-threaded random number generator
            .sample_iter(&Alphanumeric)             // consisting of alphanumeric characters
            .take(7)                                // 7 characters in length
            .map(char::from)                        // cast each into a char from the Unicode produced by &Alphanumeric iterator
            .collect();                             // collect into a collection (String)

        if fs::metadata(&filename).is_err() {       // if there's an error when attempting to read the file (i.e. file doesn't exist)
            return filename;                        // return the filename. 
        }
    }
}

fn skips_bad_file() -> Result<()> {
    let bad_path = gen_bad_file();
    let expected_result = format!("{bad_path}: .* [(]os error 2[)]");
    Command::cargo_bin(PRG)?
        .arg(bad_path)
        .assert()
        .success()
        .stderr(predicate::str::is_match(expected_result)?);
    Ok(())
}

fn run(args: &[&str], expected_file: &str) -> Result<()> {
    let expected = fs::read_to_string(expected_file)?;
    Command::cargo_bin(PRG)?
        .args(args)
        .assert()
        .success()
        .stdout(expected);
    Ok(())
}

fn run_stdin(
    input_file: &str,
    args: &[&str],
    expected_file: &str,
) -> Result<()> {
    let input = fs::read_to_string(input_file)?;
    let expected = fs::read_to_string(expected_file)?;
    Command::cargo_bin(PRG)?
        .write_stdin(input)
        .args(args)
        .assert()
        .success()
        .stdout(expected);
    Ok(())
}

#[test]
fn all_b() -> Result<()> {
    run(&["-b", EMPTY, FOX, SPIDERS, BUSTLE], "tests/expected/all.txt.b.out")
}

#[test]
fn all_n() -> Result<()> {
    run(&["-n", EMPTY, FOX, SPIDERS, BUSTLE], "tests/expected/all.txt.n.out")
}

#[test]
fn all() -> Result<()> {
    run(&[EMPTY, FOX, SPIDERS, BUSTLE], "tests/expected/all.txt.out")
}

#[test]
fn empty_b() -> Result<()> {
    run(&["-b", EMPTY], "tests/expected/empty.txt.b.out")
}

#[test]
fn empty_n() -> Result<()> {
    run(&["-n", EMPTY], "tests/expected/empty.txt.n.out")
}

#[test]
fn empty() -> Result<()> {
    run(&[EMPTY], "tests/expected/empty.txt.out")
}

#[test]
fn fox_b() -> Result<()> {
    run(&["-b", FOX], "tests/expected/fox.txt.b.out")
}

#[test]
fn fox_n() -> Result<()> {
    run(&["-n", FOX], "tests/expected/fox.txt.n.out")
}

#[test]
fn fox() -> Result<()> {
    run(&[FOX], "tests/expected/fox.txt.out")
}

#[test]
fn spiders_b() -> Result<()> {
    run(&["-b", SPIDERS], "tests/expected/spiders.txt.b.out")
}

#[test]
fn spiders_n() -> Result<()> {
    run(&["-n", SPIDERS], "tests/expected/spiders.txt.n.out")
}

#[test]
fn spiders() -> Result<()> {
    run(&[SPIDERS], "tests/expected/spiders.txt.out")
}

#[test]
fn bustle_b() -> Result<()> {
    run(&["-b", BUSTLE], "tests/expected/the-bustle.txt.b.out")
}

#[test]
fn bustle_n() -> Result<()> {
    run(&["-n", BUSTLE], "tests/expected/the-bustle.txt.n.out")
}

#[test]
fn bustle() -> Result<()> {
    run(&[BUSTLE], "tests/expected/the-bustle.txt.out")
}

#[test]
fn bustle_stdin_b() -> Result<()> {
    run_stdin(BUSTLE, &["-b", "-"], "tests/expected/the-bustle.txt.stdin.b.out")
}

#[test]
fn bustle_stdin_n() -> Result<()> {
    run_stdin(BUSTLE, &["-n", "-"], "tests/expected/the-bustle.txt.stdin.n.out")
}

#[test]
fn bustle_stdin() -> Result<()> {
    run_stdin(BUSTLE, &["-"], "tests/expected/the-bustle.txt.stdin.out")
}