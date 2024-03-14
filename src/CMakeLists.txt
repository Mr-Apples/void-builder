cmake_minimum_required(VERSION 3.12)
project(void-builder)

find_package(PkgConfig REQUIRED)

pkg_check_modules(LIBGIT2 REQUIRED libgit2)

add_executable(void-builder ./main.cpp)
target_link_libraries(void-builder PUBLIC ${LIBGIT2_LIBRARY_DIRS})
target_include_directories(void-builder PUBLIC ${LIBGIT2_INCLUDE_DIRS})