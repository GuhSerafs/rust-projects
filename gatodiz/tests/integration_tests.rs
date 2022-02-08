#[cfg(test)]
mod tests {
    use std::process::Command;
    use assert_cmd::prelude::{CommandCargoExt, OutputAssertExt};
    use predicates::prelude::*;

    #[test]
    fn run_with_defaults() -> Result<(), Box<dyn std::error::Error>> {
        Command::cargo_bin("gatodiz")
            .expect("binary exists")
            .assert()
            .success()
            .stdout(predicate::str::contains("KK eae mein"));
        Ok(())
    }

    #[test]
    fn fail_on_non_existing_file() -> Result<(), Box<dyn std::error::Error>> {
        Command::cargo_bin("gatodiz")
            .expect("binary exists")
            .args(&["-f", "no/such/file.txt"])
            .assert()
            .failure();
        Ok(())
    }
}