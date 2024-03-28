#include <iostream>
#include "git/gitHelper.hpp"
using namespace std;

int main(int argc, char** argv) {
    // Init libgi2
    git_libgit2_init();

    // Shutdown libgit2
    git_libgit2_shutdown();
    
    return 0;
}