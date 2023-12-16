use assert_cmd::Command;

#[test]
fn runs() {
    let mut cmd = Command::cargo_bin("my-project").unwrap();
    cmd.assert().success();
}