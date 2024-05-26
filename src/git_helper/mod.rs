use git2::*;
use std::*;
use std::error::Error;

/// TODO: Fix error and add trait for errors
///
/// Returns a vector that contains all repositories in the given directory,
/// takes the path that contains the repos.
/// 
/// ```rust
/// // Gets all repos in a given directory
/// repos = detect_repos_in_dir("/var/local/void-builder/");
/// ```
pub fn detect_repos_in_dir(repo_storage_dir: &path::Path) -> Result<Vec<Repository>, impl VoidBuilderError> {
    // Get an iterator for the directory in which to detect repositories
    let repo_dir_iter = fs::read_dir(repo_storage_dir)?;

    // Create a vector to store the repository objects
    let mut repositories: Vec<Repository> = Vec::new();

    for repo_dir_result in repo_dir_iter {
        match repo_dir_result {
            Ok(dir_entry) => {
                let repo = Repository::open(dir_entry.path())?;

                repositories.push(repo);
            }
            Err(e) => return Err(e)
        };
    }

    return Ok(repositories);
}

/// Clones a repository and returns the repository object, 
/// takes a repository url and the location to clone to.
/// 
/// ```rust
/// // Clone a repo
/// repo = clone_repo("https://github.com/Mr-Apples/void-builder.git", "/var/local/void-builder/");
/// ```
pub fn clone_repo(url: &str, dir: &path::Path) -> Option<Repository> {
    let url_object = match url::Url::parse(url) {
        Ok(url) => url,
        Err(e) => {
            eprintln!("Void-Builder: {}", e.to_string());
            return None;
        }
    };

    let mut dir_buf: path::PathBuf = dir.to_path_buf();
    dir_buf = dir_buf.join(path::Path::new(
        &url_object.to_string().replace("/", "")
    ));

    return match Repository::clone(&url, dir_buf) {
        Ok(repo) => Some(repo),
        Err(e) => {
            eprintln!("Void-Builder: {}", e.message());
            return None;
        }
    };
}

/// Updates the given repository with upstream commit history, takes the repository object.
/// Returns () if it succeeds else it returns the git2::Error object.
/// 
/// ```rust
/// // Updates a repository 
/// repository = git2::Repository::open("/path/to/repo");
/// update_repo(repository);
/// ```
pub fn update_repo(repo: &Repository, branch: &str) -> Result<(), impl VoidBuilderError> {
    // Get remote
    let mut remote = repo.find_remote("origin")?;

    // Do the fetch
    remote.fetch(&format!("refs/heads/{}:refs/remotes/origin/{}", branch, branch), None, None)?;

    // Get
}

/// Gets the string array of fetch refspecs for a remote
fn get_remote_fetch_refspecs(remote: &Remote) -> Result<string_array::StringArray, impl VoidBuilderError> {
    return match remote.fetch_refspecs() {
        Ok(str_array) => Ok(str_array),
        Err(e) => {
            Err(e)
        }
    }
}

/// Trait for error types void-builder will encounter, contains a function to get the error message.
trait VoidBuilderError {
    /// Returns the error message
    fn err_msg(self) -> String;
}

// Implement the trait on git2::Error
impl VoidBuilderError for git2::Error {
    fn err_msg(self) -> String {
        return self.message().to_string();
    }
}

// Implement the trait on std::io::Error
impl VoidBuilderError for std::io::Error {
    fn err_msg(self) -> String {
        return format!("{}", self);
    }
}