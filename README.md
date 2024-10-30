# Hack Assembler
For my CS590 class at Phillips Exeter Academy, we followed the [Nand to Tetris](https://www.nand2tetris.org/) book for designing and developing a computer from first principles. As part of our midterm, we were tasked with writing an assembler that translated Hack Assembly instructions into machine code. I decided to write it in Rust as a way to learn the language and to see what its capabilities were. 

## Installing Rust
Install Rust:
`curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`<br>
Verifiy your installation: `rustup --version`<br>

## Running Program
1. Navigate into top level directory.<br>
2. Add the assembly file you wish to assemble into this directory (must be error free, currently this tool doesn't support error corrections).<br>
3. Run program with `cargo run <FILE_NAME>`<br><br>

## Dependencies Used
[Clap](https://github.com/clap-rs/clap): Rust command line argument parser. <br>
[Dict](https://github.com/nachoparker/rust-dict): Dictionaries in rust. <br>
[Progress_bar](https://github.com/Mubelotix/cli_progress_bar): Rust command line progress indicator.<br>
[Regex](https://github.com/rust-lang/regex): Rust regular expression framework.
