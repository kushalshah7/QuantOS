use std::process::Command;

#[test]
fn cli_reports_the_core_status_without_financial_results() {
    let output = Command::new(env!("CARGO_BIN_EXE_quantos-cli"))
        .output()
        .expect("CLI must start");

    assert!(output.status.success());
    assert!(output.stderr.is_empty());
    assert_eq!(
        String::from_utf8(output.stdout).expect("CLI output must be UTF-8"),
        format!("{}\n", quantos_core::foundation_status())
    );
}
