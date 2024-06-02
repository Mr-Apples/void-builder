//! A tool that automatically builds xbps-src packages from git repositories
use error::VoidBuilderError;
use std::process::ExitCode;
use std::*;

// Modules
/// A module containing helper functions for git related tasks
pub mod git_helper;

/// A module containing an error struct used by Void Builder
pub mod error;

#[cfg(test)]
/// A module containing all tests as submodules
mod test;

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
fn daemonize(
    stdout: &path::Path,
    stderr: &path::Path,
    pidfile: &path::Path,
) -> Result<(), VoidBuilderError> {
    let stdout = fs::OpenOptions::new()
        .append(true)
        .create(true)
        .open(stdout)?;

    let stderr = fs::OpenOptions::new()
        .append(true)
        .create(true)
        .open(stderr)?;

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

fn main() -> ExitCode {
    // Get command-line args
    let args: Vec<_> = env::args().collect();

    // If not told not to daemonize
    if !args.contains(&"--no-daemon".to_string()) {
        // Daemonize
        if error::handle(daemonize(
            path::Path::new("/tmp/void-builder.out"),
            path::Path::new("/tmp/void-builder.err"),
            path::Path::new("/tmp/void-builder.pid"),
        )).is_err()
        {
            return ExitCode::FAILURE;
        }
    }

    if !args.contains(&"--config".to_string()) {
        let setting = error::handle(
            config::Config::builder()
                .add_source(config::File::with_name("/etc/void-builder/void-builder"))
                .build(),
        );
    }

    // Return success
    return ExitCode::SUCCESS;
}
