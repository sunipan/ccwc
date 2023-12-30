use bytecount::num_chars;
use clap::Parser;
use std::{
    fs::read,
    fs::File,
    io::{self, BufRead, BufReader},
};

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

    if !args.count && !args.lines && !args.words && !args.m {
        run_default(&args.file_name);
        return;
    } else {
        if args.count {
            if let Err(err) = count_file_bytes(&args.file_name) {
                eprintln!("Problem reading error: {:?}", err);
                return;
            }
        }
        if args.lines {
            if let Err(err) = count_file_lines(&args.file_name) {
                eprintln!("Problem reading error: {:?}", err);
            }
        }
        if args.words {
            if let Err(err) = count_file_words(&args.file_name) {
                eprintln!("Problem reading error: {:?}", err);
            }
        }
        if args.m {
            if let Err(err) = count_file_characters(&args.file_name) {
                eprintln!("Problem reading error: {:?}", err);
            }
        }
    }
}

fn count_file_bytes(file_name: &str) -> io::Result<()> {
    let file = read(file_name)?;
    println!("Bytes: {}", file.len());
    Ok(())
}

fn count_file_lines(file_name: &str) -> io::Result<()> {
    let reader = open_file(&file_name)?;
    let line_count = reader.lines().count();
    println!("Lines: {}", line_count);
    Ok(())
}

fn count_file_words(file_name: &str) -> io::Result<()> {
    let mut word_count = 0;
    let reader = open_file(&file_name)?;

    for line_result in reader.lines() {
        let line = line_result?;
        word_count += line.split_whitespace().count();
    }

    println!("Words: {}", word_count);
    Ok(())
}

fn count_file_characters(file_name: &str) -> io::Result<()> {
    let file = read(&file_name)?;
    let char_count = num_chars(&file);
    println!("Characters: {}", char_count);
    Ok(())
}

fn run_default(file_name: &str) {
    if let Err(err) = count_file_bytes(&file_name) {
        eprintln!("Problem reading error: {:?}", err);
        return;
    }
    if let Err(err) = count_file_lines(&file_name) {
        eprintln!("Problem reading error: {:?}", err);
    }
    if let Err(err) = count_file_words(&file_name) {
        eprintln!("Problem reading error: {:?}", err);
    }
    if let Err(err) = count_file_characters(&file_name) {
        eprintln!("Problem reading error: {:?}", err);
    }
}

fn open_file(file_name: &str) -> io::Result<BufReader<File>> {
    let file = File::open(file_name)?;
    Ok(BufReader::new(file))
}
