use git2::*;
use std::*;

/// Returns a vector that contains all repositories in the given directory,
/// takes the path that contains the repos.
/// 
/// ```rust
/// // Gets all repos in a given directory
/// repos = detect_repos_in_dir("/var/local/void-builder/");
/// ```
pub fn detect_repos_in_dir(repo_storage_dir: &path::Path) -> Result<Vec<Repository>, VoidBuilderError> {
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
            Err(e) => return Err(VoidBuilderError::from(e))
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
pub fn clone_repo(url: &str, dir: &path::Path) -> Result<Repository, VoidBuilderError> {
    let url_object = url::Url::parse(url)?;

    let mut dir_buf: path::PathBuf = dir.to_path_buf();
    dir_buf = dir_buf.join(path::Path::new(
        &url_object.to_string().replace("/", "")
    ));

    return Ok(Repository::clone(&url, dir_buf)?);
}

/// Updates the given repository with upstream commit history, takes the repository object.
/// Returns () if it succeeds else it returns the git2::Error object.
/// 
/// ```rust
/// // Updates a repository 
/// repository = git2::Repository::open("/path/to/repo");
/// update_repo(repository);
/// ```
pub fn update_repo_branch(repo: &Repository, branch: &str) -> Result<(), VoidBuilderError> {
    // Get remote
    let mut remote = repo.find_remote("origin")?;

    // Do the fetch
    remote.fetch(&[format!("refs/heads/{}:refs/remotes/origin/{}", branch, branch)], None, None)?;

    // Get latest current commit
    let latest_current_commit = get_latest_commit_for_branch(repo, branch)?;

    // Get latest upstream commit
    let latest_upstream_commit = repo.find_reference("FETCH_HEAD")?.peel_to_commit()?;

    // Merge latest changes and return the index
    let mut index = repo.merge_commits(&latest_current_commit, &latest_upstream_commit, None)?;

    // Check out the index
    repo.checkout_index(Some(&mut index), None)?;

    // Set head to the newly merged commit
    return Ok(repo.set_head(branch)?);
}

/// Returns the commit pointed at by the given branch
fn get_latest_commit_for_branch<'a>(repo: &'a Repository, branch_name: &str) -> Result<Commit<'a>, Error> {
    return repo.find_branch(branch_name, BranchType::Local)?.into_reference().peel_to_commit();
}

/// Struct to contain errors encountered by Void Builder
pub struct VoidBuilderError {
    message: String
}

impl VoidBuilderError {
    /// Creates a new VoidBuilderError from the given error
    fn new(message: String) -> VoidBuilderError {
        VoidBuilderError {
            message
        }
    }
}

/// Implements From<git2::Error> on VoidBuilderError
impl From<Error> for VoidBuilderError {
    /// Creates a VoidBuilderError from a git2::Error
    fn from(value: Error) -> Self {
        return VoidBuilderError::new(value.message().to_string());
    }
}

/// Implements From<std::io::Error> on VoidBuilderError
impl From<io::Error> for VoidBuilderError {
    /// Create a VoidBuilderError from a std::io::Error
    fn from(value: io::Error) -> Self {
        return VoidBuilderError::new(value.to_string());
    }
}

/// Implements From<url::ParseError> on VoidBuilderError
impl From<url::ParseError> for VoidBuilderError {
    /// Create a VoidBuilderError from a url::ParseError
    fn from(value: url::ParseError) -> Self {
        return VoidBuilderError::new(value.to_string());
    }
}