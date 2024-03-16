pkg_check_modules(LIBGIT2 REQUIRED libgit2)

add_library(git-helper STATIC git-helper.cpp git-helper.hpp)