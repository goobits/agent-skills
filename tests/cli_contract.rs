mod support;

use std::process::Command;

fn aw() -> Command {
    Command::new(support::command::aw())
}

#[test]
fn help_prints_public_cli_header_on_stdout() {
    let output = aw().arg("help").output().expect("run aw help");
    assert!(output.status.success());
    assert!(
        String::from_utf8_lossy(&output.stdout).starts_with("aw: Zero-friction Zellij workspaces")
    );
    assert!(output.stderr.is_empty());
}

#[test]
fn commit_request_rejects_missing_paths_before_queue_lookup() {
    let output = aw()
        .args(["commit", "request", "Missing paths"])
        .output()
        .expect("run aw commit request");
    assert_eq!(output.status.code(), Some(2));
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("commit request requires a title and at least one path"));
    assert!(stderr.contains("aw commit request <title> <path>..."));
    assert!(!stderr.contains("aw: Zero-friction Zellij workspaces"));
}

#[test]
fn namespace_help_is_scoped() {
    let commit = aw()
        .args(["commit", "--help"])
        .output()
        .expect("run aw commit --help");
    assert!(commit.status.success());
    let commit_stdout = String::from_utf8_lossy(&commit.stdout);
    assert!(commit_stdout.contains("aw commit request <title> <path>..."));
    assert!(!commit_stdout.contains("workspaces:"));

    let repo = aw()
        .args(["repo", "--help"])
        .output()
        .expect("run aw repo --help");
    assert!(repo.status.success());
    let repo_stdout = String::from_utf8_lossy(&repo.stdout);
    assert!(repo_stdout.contains("aw repo routes [doctor]"));
    assert!(repo_stdout.contains("aw repo worktree <path>"));
    assert!(!repo_stdout.contains("commit queue:"));
}

#[test]
fn paths_reports_aw_home_layout() {
    let output = aw().arg("paths").output().expect("run aw paths");
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("AW Paths"));
    assert!(stdout.contains(".aw"));
    assert!(stdout.contains("Legacy"));
}
