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

    let line_count = reader.lines().count();
    println!("{} {}", line_count, file_path);

    Ok(())
}
fn number_of_words(file_path: &str) {
    let f = File::open(file_path).expect("Error opening the file");
    let reader = BufReader::new(f);

    let mut word_count: u32 = 0;
    for line in reader.lines() {
        let curr: String = line.expect("Error reading content of the file");
        // let words: Vec<&str> = curr.split(" ").collect();
        let words: Vec<&str> = curr.split_whitespace().collect();
        let filtered_words: Vec<&str> = words.into_iter().filter(|word| word.len() > 0).collect();
        word_count += filtered_words.len() as u32
    }

    println!("{} {}", word_count, file_path);
}

fn number_of_characters(file_path: &str) {
    let mut file = File::open(file_path).unwrap();
    let mut s = String::new();
    file.read_to_string(&mut s).unwrap();
    print!("{} {}", s.chars().count(), file_path);
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[2];

    if args.len() > 1 && args[1] == "-c" {
        number_of_bytes(file_path).unwrap();
    } else if args.len() > 1 && args[1] == "-l" {
        number_of_lines(file_path).unwrap();
    } else if args.len() > 1 && args[1] == "-w" {
        number_of_words(&file_path);
    } else if args.len() > 1 && args[1] == "-m" {
        number_of_characters(file_path);
    } else {
        eprintln!("Usage: wc-tool -c <filepath>");
        std::process::exit(1);
    }
}
