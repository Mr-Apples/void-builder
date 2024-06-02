//! A tool that automatically builds xbps-src packages from gitHub repositories
use std::*;
use error::VoidBuilderError;

// Modules
/// A module containing helper functions for git related tasks
pub mod git_helper;

/// A module containing an error struct used by Void Builder
pub mod error;

/// Daemonize the program, creates a pidfile and appends stdout + stdin to the given locations.
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
fn daemonize(stdout: &path::Path, stderr: &path::Path, pidfile: &path::Path) -> Result<(), VoidBuilderError> {
    let stdout = fs::OpenOptions::new().append(true).create(true).open(stdout)?;

    let stderr = fs::OpenOptions::new().append(true).create(true).open(stderr)?;

    let daemon = daemonize::Daemonize::new()
        .pid_file(pidfile)
        .chown_pid_file(true)
        .working_directory("/tmp/")
        .user("nobody")
        .group("daemon")
        .stdout(stdout)
        .stderr(stderr);

    return Ok(daemon.start()?);
}

fn main() {}
