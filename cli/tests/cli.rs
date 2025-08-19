#[cfg(unix)]
mod unix_cli_tests {
    use assert_cmd::prelude::*;
    use predicates::prelude::*;
    use std::process::Command;

    #[test]
    fn ensure_not_root_behavior_matches_current_uid() {
        let mut cmd = Command::cargo_bin("lightshuttle-cli").unwrap();

        let is_root = users::get_current_uid() == 0;
        if is_root {
            cmd.assert()
                .failure()
                .stderr(predicate::str::contains("Refusing to run as root."));
        } else {
            cmd.assert()
                .success()
                .stdout(predicate::str::contains("LightShuttle CLI is ready."));
        }
    }
}

#[cfg(windows)]
mod windows_cli_tests {
    use assert_cmd::prelude::*;
    use predicates::prelude::*;
    use std::process::Command;

    #[test]
    fn runs_normally_on_windows() {
        let mut cmd = Command::cargo_bin("lightshuttle-cli").unwrap();
        cmd.assert()
            .success()
            .stdout(predicate::str::contains("LightShuttle CLI is ready."));
    }
}
