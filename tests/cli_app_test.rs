#[cfg(test)]
mod cli_app {
    // use super::*;

    use assert_cmd::prelude::*;
    use assert_fs::prelude::*;
    use predicates::prelude::*;
    use std::process::Command;

    //Test cases for Options///////////////////////////////////////////////////

    #[test]
    fn help_opton() -> Result<(), Box<dyn std::error::Error>> {
        let mut cmd = Command::cargo_bin("berry")?;

        cmd.arg("-h")
            .assert()
            .stdout(predicate::str::contains("To print this help"));

        cmd = Command::cargo_bin("berry")?;

        cmd.arg("--help")
            .assert()
            .stdout(predicate::str::contains("To print this help"));

        Ok(())
    }
    #[test]
    fn unknown_optons() -> Result<(), Box<dyn std::error::Error>> {
        let mut cmd = Command::cargo_bin("berry")?;
        cmd.arg("-hl")
            .assert()
            .failure()
            .stderr(predicate::str::contains("--help or -h"));

        cmd = Command::cargo_bin("berry")?;

        cmd.arg("--helk")
            .assert()
            .failure()
            .stderr(predicate::str::contains("--help or -h"));

        Ok(())
    }

    #[test]
    fn input_optons_no_file() -> Result<(), Box<dyn std::error::Error>> {
        let mut cmd = Command::cargo_bin("berry")?;
        cmd.arg("-i")
            .arg("somefolder/somefolder/somefilethatdon'texist.txt")
            .assert()
            .failure()
            .stderr(predicate::str::contains("not found"));

        Ok(())
    }

    //Test cases for Commands//////////////////////////////////////////////////

    #[test]
    fn run_cmd() -> Result<(), Box<dyn std::error::Error>> {
        let input_file = assert_fs::NamedTempFile::new("sample.txt")?;
        input_file.write_str("someting for now :)")?;

        let mut cmd = Command::cargo_bin("berry")?;
        cmd.arg("run").arg("-i").arg(input_file.path());
        cmd.assert()
            .stdout(predicate::str::contains("someting for ever")); //FIXME test case for run command

        Ok(())
    }
}
