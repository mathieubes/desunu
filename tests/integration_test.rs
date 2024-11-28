#[cfg(test)]
mod integration_test {
    use assert_cmd::Command;

    #[test]
    fn no_args() {
        let mut cmd = Command::cargo_bin("desunu").unwrap();
        cmd.assert().failure();
    }
}
