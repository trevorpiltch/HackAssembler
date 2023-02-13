# Hack Assembly
## Installing Rust
`curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`<br>
Check installation by running `rustup --version`<br>
Should show something like this: `rustup 1.25.1 (bb60b1e89 2022-07-12)`<br><br>
## Running Hack Assembly Program
Navigate into top level \\hack\_assembly directory.<br>
Add the assembly file you wish to assemble into this directory (must be error free).<br>
Run program with `cargo run <FILE_NAME>`<br><br>

## Dependencies Used
Clap: Rust command line argument parser. [https://github.com/clap-rs/clap]<br>
Dict: Dictionaries in rust. [https://github.com/nachoparker/rust-dict]<br>
Progress_bar: Rust command line progress indicator. [https://github.com/Mubelotix/cli_progress_bar]<br>
Regex: Rust regular expression framework. [https://github.com/rust-lang/regex]<br>