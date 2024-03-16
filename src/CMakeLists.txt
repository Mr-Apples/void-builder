cmake_minimum_required(VERSION 3.12)
project(void-builder)

find_package(PkgConfig REQUIRED)

# Compile git helper
add_subdirectory(git)

# Compile main executable
add_executable(void-builder ./main.cpp)
target_link_libraries(void-builder PUBLIC ${LIBGIT2_LIBRARY_DIRS} git-helper)
target_include_directories(void-builder PUBLIC ${LIBGIT2_INCLUDE_DIRS})