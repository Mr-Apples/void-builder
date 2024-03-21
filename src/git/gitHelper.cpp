#include "gitHelper.hpp"
#include <git2.h>
#include <iostream>
#include <regex>
using namespace std;

namespace gitHelper {
    std::string getRepoName(std::string repoURL) {
        // If '.git' is in the string remove it
        if (repoURL.substr(repoURL.size() - string(".git").size()) == ".git") {
            repoURL = repoURL.substr(0, repoURL.size() - string(".git").size());
        }

        // Find the last '/' in the URL, anything after this '/' will be the name
        int _i = 0;
        int nameIndex = 0;
        for (auto i = repoURL.begin(); i < repoURL.end(); i++) {
            if (*i == *"/") {
                nameIndex = _i + 1;
            }

            _i++;
        }
        
        // Otherwise we can remove everything before name index
        repoURL = repoURL.substr(nameIndex, string::npos);

        return repoURL;
    }

    git_repository* clone(std::string repoURL) {
        
    }
}