#[test]
fn verifier_contract_mode_is_the_ci_default() {
    let output = xtask()
        .args(["verify-api-endpoints", "--mode", "contract"])
        .output()
        .expect("xtask contract verification should run");
    assert!(
        output.status.success(),
        "{}{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("official accounted 78"));
    assert!(stdout.contains("official rejected 0"));
}

#[test]
fn forbidden_string_checker_exists() {
    let output = xtask()
        .arg("check-forbidden-strings")
        .output()
        .expect("xtask checker should run");
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
}

fn xtask() -> std::process::Command {
    let mut command = std::process::Command::new("cargo");
    command.args(["run", "-p", "xtask", "--quiet", "--"]);
    command.env("RUSTC_BOOTSTRAP", "1");
    command
}
