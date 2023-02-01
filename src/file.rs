use clap::Parser;
use std::fs;
use std::fs::File;
use std::io::prelude::*;

/// Private helped struct for parsing command line arguments
#[derive(Parser, Default, Debug)]
#[clap(author="Trevor Piltch", version="0.1", about="Assembles the provided input file into binary instructions.")]
struct Args {
    file_name: String
}

/// Gets the file and returns its contents as a single string
pub fn get_file() -> String {
    let args = Args::parse();
    println!("\nFile name: {:?}", args.file_name);

    fs::read_to_string(args.file_name).expect("Must be a file in the current directory.")
}

pub fn get_file_test(name: &str) -> String {
    fs::read_to_string(name).expect("Must be a file in the current directory.")
}

/// Creates the file in the current directory named name with contents
pub fn create_file(name: &str, contents: String) -> std::io::Result<()> {
    let bytes = contents.as_bytes();
    let mut file = File::create(name)?;
    file.write_all(&bytes)?;

    println!("Successfully created file {}", name);

    Ok(())
}

/// Returns a vec of each line in the given contents
pub fn parse_lines<'a>(contents: &'a String) -> Vec<&'a str> {
    let mut lines = vec![];

    for line in contents.lines() {
        lines.push(line);
    }

    return lines
}