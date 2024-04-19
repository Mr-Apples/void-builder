# Introduction
Many packages in the void repository do not have downloadable binary packages due to licensing restrictions and must be built from source, Void-Builder is a tool to do this.

# Features
Features that will be included in the final project are:
- [ ] Build packages with xbps-src automatically
- [ ] Build packages from multiple repositories
- [ ] Output a local package repository to be automatically used xbps
- [ ] Send a notification when new updates are available

# Building
Build with cargo:
```bash
git clone https://github.com/Mr-Apples/void-builder.git # Clone the repo
cd void-builder # Change directory to the cloned repo
cargo build # Build the project
```