use assert_cmd::Command;

#[test]
fn false_not_ok() {
    let cmd = Command::cargo_bin("false").unwrap();
    cmd.assert().failure();
}