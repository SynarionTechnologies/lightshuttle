/// LightShuttle CLI core logic.
///
/// Returns an error if the program is executed as the `root` user.
///
/// # Examples
/// ```
/// use lightshuttle_cli::run;
/// assert!(run(0).is_err());
/// assert!(run(1000).is_ok());
/// ```
pub fn run(current_uid: u32) -> Result<(), &'static str> {
    if current_uid == 0 {
        Err("Refusing to run as root.")
    } else {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn errors_for_root_user() {
        assert_eq!(run(0), Err("Refusing to run as root."));
    }

    #[test]
    fn ok_for_non_root_user() {
        assert!(run(1000).is_ok());
    }
}
