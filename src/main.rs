//! A tool that automatically builds xbps-src packages from github repositories
use std::*;

pub mod git_helper;

/// Daemonizes the program, creates a pidfile and appends stdout + stdin to the given locations.
/// The parameter `stdout`, `stderr` and `pidfile` are the locations of the stdout file, stdin file and pid file respectively.
/// Returns `true` if the daemonization succeeded.
/// 
/// ```rust
/// // Daemonize the program
/// daemonize(
///     Path::new("/tmp/void-builder.out"), 
///     Path::new("/tmp/void-builder.err"), 
///     Path::new("/tmp/void-builder.pid"));
/// ```
fn daemonize(stdout: &path::Path, stderr: &path::Path, pidfile: &path::Path) -> bool {
    let stdout = match fs::OpenOptions::new().append(true).create(true).open(stdout) {
        Ok(file) => file,
        Err(e) => {
            eprintln!("Void-Builder: {}", e.to_string());

            return false;
        }
    };

    let stderr = match fs::OpenOptions::new().append(true).create(true).open(stderr) {
        Ok(file) => file,
        Err(e) => {
            eprintln!("Void-Builder: {}", e.to_string());

            return false;
        }
    };

    let daemon = daemonize::Daemonize::new()
        .pid_file(pidfile)
        .chown_pid_file(true)
        .working_directory("/tmp/")
        .user("nobody")
        .group("daemon")
        .stdout(stdout)
        .stderr(stderr);

    match daemon.start() {
      Ok(_) => {
            println!("Void-Builder: Daemonized successfully");

            return true;
        },
      Err(e) => {
            eprintln!("Void-Builder: {}", e);

            return false;
        }
    };
}

fn main() {
    let repo = git2::Repository::open("/var/local/void-builder/https:github.comMr-Applesrepo-update-test.git/").unwrap();
    
}
