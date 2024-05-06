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

/// Updates the given repositoy with upstream commit history, takes the repository object.
/// Returns true if it succeeds.
/// 
/// ```rust
/// // Updates a repository 
/// repository = git2::Repository::open("/path/to/repo");
/// update_repo(repository);
/// ```
pub fn update_repo(repo: &Repository) -> bool {
    let remotes = match repo.remotes() {
        Ok(str_array) => str_array,
        Err(e) => {
            eprintln!("Void-Builder: {}", e.message());
            return false;
        }
    };

    for remote_name in remotes.iter() {
        let mut remote = match remote_name {
            Some(remote_name) => {
                match repo.find_remote(remote_name) {
                    Ok(remote) => remote,
                    Err(e) => {
                        eprintln!("Void-Builder: {}", e.message());
                        return false;
                    }
                }
            },
            None => {
                eprintln!("Void-Builder: Error: Failed to parse remote name: name not in utf8");
                return false;
            }
        };

        let refspecs = match get_remote_fetch_refspecs(&remote) {
            Ok(str_array) => str_array,
            Err(e) => {
                println!("Void-Builder: {}", e.message());
                return false;
            }
        };

        for refspec in refspecs.into_iter() {
            let refspec = match refspec {
                Some(_str) => _str,
                None => {
                    eprintln!("Void-Builder: Error: Cannot parse refspec: refspec is not utf8");
                    return false;
                }
            };

            match remote.fetch(&[refspec], None, None) {
                Ok(()) => (),
                Err(e) => {
                    println!("Void-Builder: {}", e.message());
                    return false;
                }
            }
        }
    };

    return true;
}

/// Gets the string array of fetch refspecs for a remote
fn get_remote_fetch_refspecs(remote: &Remote) -> Result<string_array::StringArray, git2::Error> {
    match remote.fetch_refspecs() {
        Ok(str_array) => return Ok(str_array),
        Err(e) => {
            return Err(e);
        }
    }
}