pkg_check_modules(LIBGIT2 REQUIRED libgit2)

add_library(gitHelper STATIC gitHelper.cpp gitHelper.hpp)
target_link_libraries(gitHelper PUBLIC ${LIBGIT2_LINK_LIBRARIES})
target_include_directories(gitHelper PUBLIC ${LIBGIT2_INCLUDE_DIRS})