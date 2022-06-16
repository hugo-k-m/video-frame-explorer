use assert_cmd::prelude::{CommandCargoExt, OutputAssertExt};
use predicates::prelude::predicate;
use std::error::Error;
use std::path::PathBuf;
use std::process::Command;

#[test]
fn help_message() -> Result<(), Box<dyn Error>> {
    let mut expected_help_message = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    expected_help_message.push("resources/help_message.txt");

    let mut cmd = Command::cargo_bin("video-frame-explorer")?;
    cmd.arg("--help");

    cmd.assert()
        .success()
        .stdout(predicate::path::eq_file(expected_help_message));

    Ok(())
}
