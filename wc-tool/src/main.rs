use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::io::{self, BufReader};

fn total_bytes(file_path: &str) -> io::Result<()> {
    let f = File::open(file_path)?;

    let mut reader = BufReader::new(f);
    let mut buffer = Vec::new();

    // read the whole file
    reader.read_to_end(&mut buffer)?;

    let total_bytes = buffer.len();
    println!("{} {}", total_bytes, file_path);
    Ok(())
}
fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[2];

    if args.len() > 1 && args[1] == "-c" {
        total_bytes(file_path).unwrap();
    } else {
        eprintln!("Usage: wc-tool -c <filepath>");
        std::process::exit(1);
    }
}
