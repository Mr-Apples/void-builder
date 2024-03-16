# Introduction
This project automatically builds packages from the [void package repository](https://github.com/void-linux/void-packages) or your own repo. This tool is primarily to help install packages that cannot have binary packages because of licensing requirements.
# Building
This project is built with cmake:
```shell
git clone https://github.com/Mr-Apples/void-builder.git # Clone the repo
mkdir void-builder/build && cd void-builder/build # Create and enter build directory
cmake ../src # Generate build files
make # Build with make
```
After this, place the generate executable wherever you like.
# Usage
The tool runs a daemon which will periodically check for updates to the repos it is set up to build, after an update it will build the packages and output a local repo containing them. Current the project is still literally hello world so I will document it more after it's more complete.