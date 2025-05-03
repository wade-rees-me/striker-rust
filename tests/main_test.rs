use rstest::rstest;
use std::process::Command;

#[rstest]
#[case("--mimic", 10000000, "path")]
#[case("--basic", 10000000, "path")]
#[case("--linear", 100000, "path")]
#[case("--polynomial", 100000, "path")]
#[case("--neural", 100000, "path")]
#[case("--high-low", 100000, "path")]
#[case("--wong", 10000000, "path")]
fn test_striker(#[case] strategy: &str, #[case] hands: usize, #[case] path: &str) {
    println!("{}:{}:{}", strategy, hands, path);
    run_striker_test(strategy, hands);
}

fn run_striker_test(strategy_flag: &str, hands: usize) {
    let output = Command::new(env!("CARGO_BIN_EXE_striker"))
        .args([strategy_flag, "--single-deck", "--number-of-threads", "1", "--number-of-hands", &hands.to_string()])
        .output()
        .expect("Failed to execute process");

    assert!(output.status.success(), "Main did not run successfully. Exit status: {:?}", output.status);

    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);

    println!("STDOUT:\n{}", stdout);
    println!("STDERR:\n{}", stderr);

    for keyword in ["Start: striker", "results", "insert"] {
        assert!(stdout.contains(keyword), "Missing '{}' in output", keyword);
    }

    assert!(!stderr.contains("panicked"), "There was a panic in stderr: {}", stderr);
}

#[test]
fn test_main_invalid_args() {
    let output = Command::new(env!("CARGO_BIN_EXE_striker"))
        .arg("--invalid-flag")
        .output()
        .expect("failed to execute process");

    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(!output.status.success() || stderr.contains("error") || stderr.contains("Usage"));
}
