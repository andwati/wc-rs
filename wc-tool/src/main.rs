use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::io::{self, BufReader};

fn number_of_bytes(file_path: &str) -> io::Result<()> {
    let f = File::open(file_path)?;

    let mut reader = BufReader::new(f);
    let mut buffer = Vec::new();

    // read the whole file
    reader.read_to_end(&mut buffer)?;

    let total_bytes = buffer.len();
    println!("{} {}", total_bytes, file_path);
    Ok(())
}

fn number_of_lines(file_path: &str) -> io::Result<()> {
    let f = File::open(file_path)?;
    let reader = BufReader::new(f);

    // let mut line = String::new();
    // let len = reader.read_line(&mut line)?;
    // println!("{} {}", len, file_path);
    let line_count = reader.lines().count();
    println!("{} {}", line_count, file_path);

    Ok(())
}
fn number_of_words() {}
fn number_of_characters() {}

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[2];

    if args.len() > 1 && args[1] == "-c" {
        number_of_bytes(file_path).unwrap();
    } else if args.len() > 1 && args[1] == "-l" {
        number_of_lines(file_path).unwrap();
    } else if args.len() > 1 && args[1] == "-w" {
        number_of_words();
    } else if args.len() > 1 && args[1] == "-m" {
        number_of_characters();
    } else {
        eprintln!("Usage: wc-tool -c <filepath>");
        std::process::exit(1);
    }
}
