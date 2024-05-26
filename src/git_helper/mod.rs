use git2::*;
use std::*;

/// Returns a vector that contains all repositories in the given directory, 
/// takes the path that contains the repos.
/// 
/// ```rust
/// // Gets all repos in a given directory
/// repos = detect_repos_in_dir("/var/local/void-builder/");
/// ```
pub fn detect_repos_in_dir(repo_storage_dir: &path::Path) -> Option<Vec<Repository>> {
    // Get an iterator for the directory in which to detect repositories
    let repo_dir_iter = match fs::read_dir(repo_storage_dir) {
        Ok(read_dir) => read_dir,
        Err(e) => {
            eprintln!("Void-Builder: {}", e.to_string());
            return None;
        }
    };

    // Create a vector to store the repository objects
    let mut repositories: Vec<Repository> = Vec::new();

    for repo_dir_result in repo_dir_iter {
        match repo_dir_result {
            Ok(dir_entry) => {
                let repo = match Repository::open(dir_entry.path()) {
                    Ok(repo) => repo,
                    Err(e) => {
                        eprintln!("Void-Builder: {}", e.message());
                        return None;
                    }
                };

                repositories.push(repo);
            }
            Err(e) => {
                eprintln!("Void-Builder: {}", e.to_string());
                return None;
            }
        };
    }

    return Some(repositories);
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

    return match git2::Repository::clone(&url, dir_buf) {
        Ok(repo) => Some(repo),
        Err(e) => {
            eprintln!("Void-Builder: {}", e.message());
            return None;
        }
    };
}

/// Updates the given repositoy with 
/// upstream commit history for the given refspecs, 
/// takes the repository object.
/// Returns true if it succeeds
/// 
/// ```rust
/// // Updates a repository 
/// repository = git2::Repository::open("/path/to/repo");
/// update_repo_branch(repository);
/// ```
pub fn update_repo_branch(repo: &Repository, branch_name: &str) -> Result<(), git2::Error> {
    // Get the remote and fetch all new commits
    let mut remote = repo.find_remote("origin")?;
    remote.fetch(&[""], None, None)?;

    // Get the current latest commit for this branch
    let current_commit = find_last_commit_for_branch(repo, branch_name)?;

    // Find the new latest commit
    let new_commit_reference = repo.find_reference("FETCH_HEAD")?;

    let new_commit = new_commit_reference.peel_to_commit()?;

    let mut index = repo.merge_commits(&current_commit, &new_commit, None)?;

    repo.checkout_index(Some(&mut index), None)?;
    
    return Ok(());
}

/// Gets the string array of fetch refspecs for a remote
fn get_remote_fetch_refspecs(remote: &Remote) -> Result<string_array::StringArray, git2::Error> {
    return Ok(remote.fetch_refspecs()?);
}

/// Returns the last commit on a given branch
fn find_last_commit_for_branch<'a>(repo: &'a Repository, branch_name: &str) -> Result<Commit<'a>, Error> {
    let obj = repo.find_branch(branch_name, BranchType::Local)?.into_reference().peel(ObjectType::Commit)?;
    
    match obj.into_commit() {
        Ok(commit) => return Ok(commit),
        Err(obj) => return Err(Error::from_str(&format!("Unable to find the last commit of branch: {}", branch_name)))
    }
}