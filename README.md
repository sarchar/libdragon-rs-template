# LibDragon Rust Template
This project is a skeleton project meant to kickoff new [libdragon-rs](https://github.com/sarchar/libdragon-rs) projects quickly.  This project is meant to be built only in Linux (or WSL2).

# Building
1. Install dependencies:
`apt-get install build-essential gcc make clang`
2. Install Rust and make sure `cargo` is in your path.
3. Install Justfile:
`cargo install just`
4. Clone this repo:
`git clone https://github.com/sarchar/libdragon-rs-template`
5. Build this repo:
`cd libdragon-rs-template && just build`

The output N64 program will be at `./target/mips-nintendo64-none/debug/libdragon-rs-template.z64`, along with various intermediate files in the same directory.

