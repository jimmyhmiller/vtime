use std::process::Command;

#[test]
fn double_dash_runs_the_following_command() {
    let output = Command::new(env!("CARGO_BIN_EXE_vtime"))
        .args(["--", "sh", "-c", "exit 0"])
        .output()
        .expect("vtime should run");

    assert!(output.status.success(), "stderr: {}", String::from_utf8_lossy(&output.stderr));
}

#[test]
fn help_exits_successfully_and_prints_usage() {
    let output = Command::new(env!("CARGO_BIN_EXE_vtime"))
        .arg("--help")
        .output()
        .expect("vtime should run");

    let stderr = String::from_utf8_lossy(&output.stderr);

    assert!(output.status.success(), "stderr: {stderr}");
    assert!(stderr.contains("Usage: vtime [flags] <command> [args...]"), "stderr: {stderr}");
    assert!(stderr.contains("-a, --all"), "stderr: {stderr}");
}

#[test]
fn no_command_prints_usage_and_exits_with_error() {
    let output = Command::new(env!("CARGO_BIN_EXE_vtime"))
        .output()
        .expect("vtime should run");

    let stderr = String::from_utf8_lossy(&output.stderr);

    assert_eq!(output.status.code(), Some(1), "stderr: {stderr}");
    assert!(stderr.contains("Usage: vtime [flags] <command> [args...]"), "stderr: {stderr}");
}

#[test]
fn invalid_grouped_short_flag_is_rejected() {
    let output = Command::new(env!("CARGO_BIN_EXE_vtime"))
        .arg("-cz")
        .output()
        .expect("vtime should run");

    let stderr = String::from_utf8_lossy(&output.stderr);

    assert_eq!(output.status.code(), Some(1), "stderr: {stderr}");
    assert!(stderr.contains("vtime: unknown flag '-z'"), "stderr: {stderr}");
    assert!(stderr.contains("Usage: vtime [flags] <command> [args...]"), "stderr: {stderr}");
}

#[test]
fn command_exit_code_is_propagated() {
    let output = Command::new(env!("CARGO_BIN_EXE_vtime"))
        .args(["sh", "-c", "exit 7"])
        .output()
        .expect("vtime should run");

    let stderr = String::from_utf8_lossy(&output.stderr);

    assert_eq!(output.status.code(), Some(7), "stderr: {stderr}");
    assert!(stderr.contains("Wall clock time"), "stderr: {stderr}");
    assert!(stderr.contains("User CPU time"), "stderr: {stderr}");
    assert!(stderr.contains("System CPU time"), "stderr: {stderr}");
}

#[test]
fn cpu_flag_prints_cpu_details() {
    let output = Command::new(env!("CARGO_BIN_EXE_vtime"))
        .args(["-c", "sh", "-c", "exit 0"])
        .output()
        .expect("vtime should run");

    let stderr = String::from_utf8_lossy(&output.stderr);

    assert!(output.status.success(), "stderr: {stderr}");
    assert!(stderr.contains("CPU utilization"), "stderr: {stderr}");
    assert!(stderr.contains("Voluntary ctx sw"), "stderr: {stderr}");
    assert!(stderr.contains("Involuntary ctx sw"), "stderr: {stderr}");
}

#[test]
fn memory_flag_prints_memory_details() {
    let output = Command::new(env!("CARGO_BIN_EXE_vtime"))
        .args(["-m", "sh", "-c", "exit 0"])
        .output()
        .expect("vtime should run");

    let stderr = String::from_utf8_lossy(&output.stderr);

    assert!(output.status.success(), "stderr: {stderr}");
    assert!(stderr.contains("Max memory (RSS)"), "stderr: {stderr}");
    assert!(stderr.contains("Page faults (major)"), "stderr: {stderr}");
    assert!(stderr.contains("Page faults (minor)"), "stderr: {stderr}");
}

#[test]
fn interrupted_run_still_prints_summary() {
    let output = Command::new(env!("CARGO_BIN_EXE_vtime"))
        .args(["sh", "-c", "sleep 1; kill -INT $$"])
        .output()
        .expect("vtime should run");

    let stderr = String::from_utf8_lossy(&output.stderr);

    assert_eq!(output.status.code(), Some(130), "stderr: {stderr}");
    assert!(stderr.contains("Wall clock time"), "stderr: {stderr}");
    assert!(stderr.contains("Killed by signal"), "stderr: {stderr}");
}
