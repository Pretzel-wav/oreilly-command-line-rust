use assert_cmd::Command;

#[test]
fn runs() {
    assert!(true)
}

#[test]
fn true_ok() {
    let mut cmd = Command::cargo_bin("truer").unwrap();
    cmd.assert().success();
}