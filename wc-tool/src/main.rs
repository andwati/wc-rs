use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::io::{self, BufReader};

fn main() -> io::Result<()> {
    let current_dir = env::current_dir()?;
    let file_name = "test.txt";
    let file_path = current_dir.join(file_name);
    println!("file path: {:?}", file_path);

    let f = File::open(file_path)?;

    let mut reader = BufReader::new(f);
    let mut buffer = Vec::new();

    // read the whole file
    reader.read_to_end(&mut buffer)?;

    let total_bytes = buffer.len();
    println!("total bytes: {}", total_bytes);
    Ok(())
}
