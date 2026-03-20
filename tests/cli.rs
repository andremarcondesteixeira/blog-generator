use std::process::Command;

#[test]
fn fails_without_config_file_argument() {
    let output = Command::new(env!("CARGO_BIN_EXE_blog-generator"))
        .output()
        .expect("Failed to execute binary");

    assert!(!output.status.success());

    let stderr = String::from_utf8(output.stderr).expect("stderr is not valid utf-8");
    assert!(
        stderr.contains("Usage:"),
        "Expected usage message in stderr, got: {stderr}"
    );
}
