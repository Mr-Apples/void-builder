pkg_check_modules(LIBGIT2 REQUIRED libgit2)

add_library(git-helper STATIC gitHelper.cpp gitHelper.hpp)