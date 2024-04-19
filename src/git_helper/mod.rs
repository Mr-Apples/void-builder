use git2::Repository;
use std::*;

// Returns a vector that contains all repositories in a given directory
pub fn detect_repos_in_dir(repo_storage_dir: &path::Path) -> Option<Vec<Repository>> {
    // Get an iterator for the directory in which to detect repositories
    let repo_dir_iter = match fs::read_dir(repo_storage_dir) {
        Ok(read_dir) => read_dir,
        Err(e) => {
            println!("Void-Builder: {}", e.to_string());
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
                        println!("Void-Builder: {}", e.message());
                        return None;
                    }
                };

                repositories.push(repo);
            }
            Err(e) => {
                println!("Void-Builder: {}", e.to_string());
                return None;
            }
        };
    }

    return Some(repositories);
}

// Clones a repository and returns the repository object
pub fn clone_repo(url: &str, dir: &path::Path) -> Option<Repository> {
    let mut dir_buf: path::PathBuf = dir.to_path_buf();
    dir_buf = dir_buf.join(path::Path::new(
        &url.replace("/", "")
    ));

    return match git2::Repository::clone(url, dir_buf) {
        Ok(repo) => Some(repo),
        Err(e) => {
            println!("Void-Builder: {}", e.message());
            return None;
        }
    };
}

pub fn update_repo(repo: &Repository) -> Option<()> {
    match repo.find_remote("origin") {
        Ok(mut remote) => {
            match remote.update_tips(None, true, git2::AutotagOption::Auto, None) {
                Ok(_) => (),
                Err(e) => {
                    println!("Void-Builder: {}", e.message());
                    return None;
                }
            };

            return Some(());
        },
        Err(e) => {
            println!("Void-Builder: {}", e.message());
            return Some(());
        }
    }
}