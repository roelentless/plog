use std::env;
use std::fs;
use std::io::{Read, Write};
use std::path::PathBuf;
use std::process::{Command, Stdio};
use std::sync::atomic::{AtomicI32, Ordering};
use std::sync::Arc;
use std::thread;

fn make_slug(args: &[String]) -> String {
    let raw = args.join(" ");
    let mut slug = String::new();
    let mut prev_sep = true;
    for c in raw.chars() {
        if c.is_ascii_alphanumeric() {
            slug.push(c);
            prev_sep = false;
        } else if !prev_sep {
            slug.push('-');
            prev_sep = true;
        }
    }
    while slug.ends_with('-') {
        slug.pop();
    }
    slug.chars().take(60).collect()
}

fn write_info(log_dir: &PathBuf, cmd: &str, started: &str, pid: Option<u32>, exit_code: Option<i32>) {
    let info = serde_json::json!({
        "command": cmd,
        "started": started,
        "pid": pid,
        "exit_code": exit_code,
    });
    let tmp = log_dir.join("info.json.tmp");
    fs::write(&tmp, info.to_string()).expect("failed to write info.json");
    fs::rename(tmp, log_dir.join("info.json")).expect("failed to rename info.json");
}

fn copy_to_both(mut reader: impl Read, mut terminal: impl Write, mut file: impl Write) {
    let mut buf = [0u8; 8192];
    loop {
        match reader.read(&mut buf) {
            Ok(0) => break,
            Ok(n) => {
                terminal.write_all(&buf[..n]).expect("failed to write to terminal");
                terminal.flush().expect("failed to flush terminal");
                file.write_all(&buf[..n]).expect("failed to write to log file");
            }
            Err(e) => panic!("read error: {e}"),
        }
    }
}

fn ensure_gitignore() {
    let path = std::path::Path::new(".gitignore");
    if !path.exists() {
        return;
    }
    let content = fs::read_to_string(path).expect("failed to read .gitignore");
    if !content.lines().any(|l| l.trim() == "plogs") {
        let mut f = fs::OpenOptions::new().append(true).open(path).expect("failed to open .gitignore");
        let prefix = if content.ends_with('\n') || content.is_empty() { "" } else { "\n" };
        f.write_all(format!("{}plogs\n", prefix).as_bytes()).expect("failed to write .gitignore");
    }
}

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();

    if args.is_empty() {
        eprintln!("Usage: plog <command> [args...]");
        std::process::exit(1);
    }

    let slug = make_slug(&args);
    let log_dir = PathBuf::from("plogs").join(slug);

    fs::create_dir_all(&log_dir).expect("failed to create log dir");
    ensure_gitignore();

    let cmd_str = args.join(" ");
    let started = chrono::Local::now().to_rfc3339();

    write_info(&log_dir, &cmd_str, &started, None, None);

    let mut child = Command::new(&args[0])
        .args(&args[1..])
        .stdin(Stdio::inherit())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .unwrap_or_else(|e| {
            eprintln!("plog: failed to run '{}': {}", args[0], e);
            std::process::exit(127);
        });

    let pid = child.id();

    // Forward SIGTERM and SIGINT to the child.
    let child_pid = Arc::new(AtomicI32::new(pid as i32));
    {
        let child_pid = Arc::clone(&child_pid);
        thread::spawn(move || {
            let mut signals =
                signal_hook::iterator::Signals::new([signal_hook::consts::SIGTERM, signal_hook::consts::SIGINT])
                    .expect("failed to register signals");
            for sig in signals.forever() {
                let p = child_pid.load(Ordering::SeqCst);
                if p > 0 {
                    unsafe { libc::kill(p, sig) };
                }
            }
        });
    }

    let stdout_child = child.stdout.take().unwrap();
    let stderr_child = child.stderr.take().unwrap();
    let stdout_log = fs::File::create(log_dir.join("stdout.log")).unwrap();
    let stderr_log = fs::File::create(log_dir.join("stderr.log")).unwrap();

    let t1 = thread::spawn(move || copy_to_both(stdout_child, std::io::stdout(), stdout_log));
    let t2 = thread::spawn(move || copy_to_both(stderr_child, std::io::stderr(), stderr_log));

    let status = child.wait().expect("failed to wait for child");
    child_pid.store(0, Ordering::SeqCst);

    t1.join().unwrap();
    t2.join().unwrap();

    use std::os::unix::process::ExitStatusExt;
    let exit_code = status
        .code()
        .unwrap_or_else(|| 128 + status.signal().expect("process has neither exit code nor signal"));

    write_info(&log_dir, &cmd_str, &started, Some(pid), Some(exit_code));

    std::process::exit(exit_code);
}
