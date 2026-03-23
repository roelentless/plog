use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::time::Duration;
use tempfile::TempDir;

fn plog_bin() -> PathBuf {
    let mut path = std::env::current_exe().unwrap();
    path.pop();
    if path.ends_with("deps") {
        path.pop();
    }
    path.push("plog");
    path
}

fn log_dir(base: &Path) -> PathBuf {
    fs::read_dir(base.join("plogs"))
        .unwrap()
        .filter_map(|e| e.ok())
        .map(|e| e.path())
        .find(|p| p.is_dir())
        .unwrap()
}

fn plog(tmp: &TempDir, args: &[&str]) -> std::process::Output {
    Command::new(plog_bin())
        .current_dir(tmp.path())
        .args(args)
        .output()
        .unwrap()
}

// ── exit codes ───────────────────────────────────────────────────────────────

#[test]
fn exit_code_zero() {
    let tmp = TempDir::new().unwrap();
    assert_eq!(plog(&tmp, &["true"]).status.code(), Some(0));
}

#[test]
fn exit_code_one() {
    let tmp = TempDir::new().unwrap();
    assert_eq!(plog(&tmp, &["false"]).status.code(), Some(1));
}

#[test]
fn exit_code_custom() {
    let tmp = TempDir::new().unwrap();
    assert_eq!(plog(&tmp, &["sh", "-c", "exit 42"]).status.code(), Some(42));
}

// ── stdout ───────────────────────────────────────────────────────────────────

#[test]
fn stdout_forwarded() {
    let tmp = TempDir::new().unwrap();
    let out = plog(&tmp, &["echo", "hello world"]);
    assert_eq!(String::from_utf8_lossy(&out.stdout).trim(), "hello world");
}

#[test]
fn stdout_captured() {
    let tmp = TempDir::new().unwrap();
    plog(&tmp, &["echo", "hello world"]);
    let log = fs::read_to_string(log_dir(tmp.path()).join("stdout.log")).unwrap();
    assert!(log.contains("hello world"));
}

// ── stderr ───────────────────────────────────────────────────────────────────

#[test]
fn stderr_forwarded() {
    let tmp = TempDir::new().unwrap();
    let out = plog(&tmp, &["sh", "-c", "echo err output >&2"]);
    assert_eq!(String::from_utf8_lossy(&out.stderr).trim(), "err output");
}

#[test]
fn stderr_captured() {
    let tmp = TempDir::new().unwrap();
    plog(&tmp, &["sh", "-c", "echo err output >&2"]);
    let log = fs::read_to_string(log_dir(tmp.path()).join("stderr.log")).unwrap();
    assert!(log.contains("err output"));
}

// ── separation ───────────────────────────────────────────────────────────────

#[test]
fn stdout_stderr_are_separate() {
    let tmp = TempDir::new().unwrap();
    plog(&tmp, &["sh", "-c", "echo out line; echo err line >&2"]);
    let log_dir = log_dir(tmp.path());
    let stdout = fs::read_to_string(log_dir.join("stdout.log")).unwrap();
    let stderr = fs::read_to_string(log_dir.join("stderr.log")).unwrap();
    assert!(stdout.contains("out line"));
    assert!(!stdout.contains("err line"));
    assert!(stderr.contains("err line"));
    assert!(!stderr.contains("out line"));
}

// ── info.json ────────────────────────────────────────────────────────────────

#[test]
fn info_json_fields() {
    let tmp = TempDir::new().unwrap();
    plog(&tmp, &["echo", "test"]);
    let info: serde_json::Value =
        serde_json::from_str(&fs::read_to_string(log_dir(tmp.path()).join("info.json")).unwrap()).unwrap();
    assert_eq!(info["command"], "echo test");
    assert_eq!(info["exit_code"], 0);
    assert!(info["pid"].is_number());
    assert!(info["started"].is_string());
}

#[test]
fn info_json_exit_code_on_failure() {
    let tmp = TempDir::new().unwrap();
    plog(&tmp, &["false"]);
    let info: serde_json::Value =
        serde_json::from_str(&fs::read_to_string(log_dir(tmp.path()).join("info.json")).unwrap()).unwrap();
    assert_eq!(info["exit_code"], 1);
}

// ── directory structure ───────────────────────────────────────────────────────

#[test]
fn creates_plogs_in_cwd() {
    let tmp = TempDir::new().unwrap();
    plog(&tmp, &["true"]);
    assert!(tmp.path().join("plogs").is_dir());
}

#[test]
fn creates_log_files() {
    let tmp = TempDir::new().unwrap();
    plog(&tmp, &["true"]);
    let log_dir = log_dir(tmp.path());
    assert!(log_dir.join("stdout.log").exists());
    assert!(log_dir.join("stderr.log").exists());
    assert!(log_dir.join("info.json").exists());
}

// ── stdin ─────────────────────────────────────────────────────────────────────

#[test]
fn stdin_forwarded() {
    let tmp = TempDir::new().unwrap();
    let mut child = Command::new(plog_bin())
        .current_dir(tmp.path())
        .arg("cat")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .unwrap();
    child.stdin.take().unwrap().write_all(b"piped input\n").unwrap();
    let out = child.wait_with_output().unwrap();
    assert_eq!(String::from_utf8_lossy(&out.stdout).trim(), "piped input");
}

// ── signals ───────────────────────────────────────────────────────────────────

#[test]
fn sigterm_forwarded_to_child() {
    let tmp = TempDir::new().unwrap();
    let child = Command::new(plog_bin())
        .current_dir(tmp.path())
        .args(["bash", "-c", "trap 'exit 143' TERM; sleep 10"])
        .spawn()
        .unwrap();
    let pid = child.id();
    std::thread::sleep(Duration::from_millis(300));
    unsafe { libc::kill(pid as libc::pid_t, libc::SIGTERM) };
    let out = child.wait_with_output().unwrap();
    assert_eq!(out.status.code(), Some(143));
}

#[test]
fn info_json_updated_after_sigterm() {
    let tmp = TempDir::new().unwrap();
    let child = Command::new(plog_bin())
        .current_dir(tmp.path())
        .args(["bash", "-c", "trap 'exit 143' TERM; sleep 10"])
        .spawn()
        .unwrap();
    let pid = child.id();
    std::thread::sleep(Duration::from_millis(300));
    unsafe { libc::kill(pid as libc::pid_t, libc::SIGTERM) };
    child.wait_with_output().unwrap();
    let info: serde_json::Value =
        serde_json::from_str(&fs::read_to_string(log_dir(tmp.path()).join("info.json")).unwrap()).unwrap();
    assert_eq!(info["exit_code"], 143);
}
