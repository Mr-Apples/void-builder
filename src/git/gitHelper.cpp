#include "gitHelper.hpp"
#include <git2.h>
#include <iostream>
#include <regex>

namespace gitHelper {
    std::string removeSlashesFromString(std::string string) {
        for (char i : string) {
            if (i == '/') {
                string.erase(std::remove(string.begin(), string.end(), '/'), string.end());
            }
        }

        return string;
    }

    git_repository* clone(std::string repoURL, std::string path) {
        // Create a pointer to a git_repository object
        git_repository* repo;
        path += removeSlashesFromString(repoURL) + '/';

        int error = git_clone(&repo, repoURL.c_str(), path.c_str(), NULL);
        
        // If there was an error print the error code
        if (error) {
            std::cout << "void-builder: " << git_error_last()->message << std::endl;
        }

        return repo;
    }
}