use clap::Parser;
use std::fs;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    // Option to count bytes of file
    #[arg(short)]
    count: bool,

    // Option to count lines of file
    #[arg(short)]
    lines: bool,

    // Option to count words of file
    #[arg(short)]
    words: bool,

    // Option to count characters of file
    #[arg(short)]
    m: bool,

    // File to run operations on
    file_name: String,
}

fn main() {
    let args = Args::parse();

    if args.count {
        count_file_bytes(args.file_name);
    }
    if args.lines {
        count_file_lines();
    }
    if args.words {
        count_file_words();
    }
    if args.m {
        count_file_characters();
    }
}

fn count_file_bytes(file_name: String) {
    let file_result = fs::read(file_name);
    let file = match file_result {
        Ok(file_vec) => file_vec,
        Err(error) => panic!("Problem reading error: {:?}", error),
    };

    println!("Bytes: {}", file.len());
}

fn count_file_lines() {}

fn count_file_words() {}

fn count_file_characters() {}
