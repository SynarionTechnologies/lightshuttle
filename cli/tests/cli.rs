#[cfg(unix)]
mod unix_cli_tests {
    use assert_cmd::prelude::*;
    use std::process::Command;

    #[test]
    fn fails_when_running_as_root() {
        let mut cmd = Command::cargo_bin("lightshuttle-cli").unwrap();
        cmd.assert()
            .failure()
            .stderr(predicates::str::contains("Refusing to run as root."));
    }
}

#[cfg(windows)]
mod windows_cli_tests {
    use assert_cmd::prelude::*;
    use std::process::Command;

    #[test]
    fn runs_normally_on_windows() {
        let mut cmd = Command::cargo_bin("lightshuttle-cli").unwrap();
        cmd.assert()
            .success()
            .stdout(predicates::str::contains("LightShuttle CLI is ready."));
    }
}
