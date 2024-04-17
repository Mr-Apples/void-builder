use std::*;
use git2::Repository;

fn detect_repos_in_dir(repo_parent_dir: &path::Path) -> Option<Vec<Repository>> {
    // Get an iterator for the directory in which to detect repositories
    let repo_dir_iter = match fs::read_dir(repo_parent_dir) {
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
            },
            Err(e) => {
                println!("Void-Builder: {}", e.to_string());
                return None;
            }
        };
    };

    return Some(repositories);
}