# Hack Assembly
## Installing Rust
`curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`
Check installation by running `rustup --version`
Should show something like this: `rustup 1.25.1 (bb60b1e89 2022-07-12)`
## Running Hack Assembly Program
Navigate into top level \\hack\_assembly directory.
Add the assembly file you wish to assemble into this directory (must be error free).
Run program with `cargo run <FILE_NAME>`