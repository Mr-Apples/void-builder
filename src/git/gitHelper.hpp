#ifndef _VOID_BUILDER_GIT_HELPER_H
#define _VOID_BUILDER_GIT_HELPER_H
#include <iostream>
#include <git2.h>
#include <regex>

namespace gitHelper {
    std::string getRepoName(std::string);
    
    git_repository* clone(std::string, std::string);
}
#endif